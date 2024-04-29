// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::core::models::{AllocatedResource, ProjectLevelDiskCache, UseCase, UserLevelDiskCache};
use itertools::Itertools;
use std::path::{Path, PathBuf};
use ubyte::ByteUnit;
use walkdir::{DirEntry, WalkDir};

pub fn resources_used_by_gradle_projects(projects: &[PathBuf]) -> anyhow::Result<AllocatedResource> {
    let use_case = UseCase::from(ProjectLevelDiskCache::BuildOutput);
    let allocated = projects
        .iter()
        .map(|project| resources_used_per_gradle_project(project.as_path()))
        .collect::<anyhow::Result<Vec<AllocatedResource>>>()?;

    let total_amount = allocated
        .iter()
        .fold(ByteUnit::from(0), |total, allocation| total + allocation.amount);

    Ok(AllocatedResource::new(use_case, total_amount))
}

fn resources_used_per_gradle_project(gradle_project: &Path) -> anyhow::Result<AllocatedResource> {
    let use_case = UseCase::from(ProjectLevelDiskCache::BuildOutput);
    let disk_dize = total_disk_size(gradle_project, ensure_build_output_file)?;
    Ok(AllocatedResource::new(use_case, disk_dize))
}

pub fn resources_used_by_maven_local_repository(maven_local: &Path) -> anyhow::Result<AllocatedResource> {
    let use_case = UseCase::from(UserLevelDiskCache::MavenLocalRepository);
    let disk_size = total_disk_size(maven_local, ensure_file)?;
    Ok(AllocatedResource::new(use_case, disk_size))
}

pub fn resources_used_by_konan(konan_folder: &Path) -> anyhow::Result<AllocatedResource> {
    let use_case = UseCase::from(UserLevelDiskCache::KonanCaches);
    let disk_size = total_disk_size(konan_folder, ensure_file)?;
    Ok(AllocatedResource::new(use_case, disk_size))
}

pub fn resources_used_by_gradle_home(gradle_home: &Path) -> anyhow::Result<Vec<AllocatedResource>> {
    let Ok(true) = gradle_home.try_exists() else {
        return Ok(vec![]);
    };

    let total_per_use_case = WalkDir::new(gradle_home)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(ensure_file)
        .map(|entry| (size_for_entry(&entry), evaluate_use_case_from_gradle_home(&entry)))
        .filter(|item| item.1 != UseCase::from(UserLevelDiskCache::GradleOtherCaches))
        .group_by(|item| item.1)
        .into_iter()
        .map(|(use_case, group)| (use_case, group.fold(0, |total, (entry_size, _)| total + entry_size)))
        .map(|(use_case, total)| AllocatedResource::new(use_case, ByteUnit::from(total)))
        .sorted_by_key(|item| item.use_case)
        .collect::<Vec<_>>();

    Ok(total_per_use_case)
}

fn total_disk_size(folder_path: &Path, filter: fn(&DirEntry) -> bool) -> anyhow::Result<ByteUnit> {
    let Ok(true) = folder_path.try_exists() else {
        return Ok(ByteUnit::from(0));
    };

    let disk_size = WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(filter)
        .map(|entry| size_for_entry(&entry))
        .sum::<u64>();

    Ok(ByteUnit::from(disk_size))
}

fn size_for_entry(entry: &DirEntry) -> u64 {
    entry.metadata().expect("Expecting a valid metadata for entry").len()
}

fn ensure_file(entry: &DirEntry) -> bool {
    entry
        .metadata()
        .expect("Expecting a valid metadata for entry")
        .is_file()
}

fn ensure_build_output_file(entry: &DirEntry) -> bool {
    let path = entry.path().to_str().expect("Expecting a valid path");
    let build_output = path.contains("build/");

    let build_output_file = entry
        .metadata()
        .expect("Expecting a valid metadata for entry")
        .is_file();

    build_output && build_output_file
}

fn evaluate_use_case_from_gradle_home(entry: &DirEntry) -> UseCase {
    let raw_path = entry
        .path()
        .to_str()
        .expect("Should be able to evaluate path as string");

    // https://docs.gradle.org/current/userguide/configuration_cache.html
    // https://docs.gradle.org/current/userguide/directory_layout.html
    let cache_type = match raw_path {
        _ if raw_path.contains(".gradle/caches") => UserLevelDiskCache::GradleBuildCaching,
        _ if raw_path.contains(".gradle/configuration-cache") => UserLevelDiskCache::GradleConfigurationCaching,
        _ if raw_path.contains(".gradle/daemon") => UserLevelDiskCache::GradleDaemonLogs,
        _ if raw_path.contains(".gradle/jdks") => UserLevelDiskCache::GradleJDKToolchains,
        _ if raw_path.contains(".gradle/wrapper") => UserLevelDiskCache::GradleDistributions,
        _ if raw_path.contains(".gradle/.tmp") => UserLevelDiskCache::GradleTemporaryFiles,
        _ if raw_path.contains(".gradle/native") => UserLevelDiskCache::GradleNativeFiles,
        _ if raw_path.contains(".gradle/build-scan-data") => UserLevelDiskCache::GradleBuildScans,
        _ => UserLevelDiskCache::GradleOtherCaches,
    };

    UseCase::from(cache_type)
}

#[cfg(test)]
mod tests {
    use crate::core::disk::{
        resources_used_by_gradle_home, resources_used_by_gradle_projects, resources_used_by_maven_local_repository,
    };
    use crate::core::models::{AllocatedResource, ProjectLevelDiskCache, UseCase, UserLevelDiskCache};
    use fake::{Fake, StringFaker};
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use temp_dir::TempDir;
    use ubyte::{ByteUnit, ToByteUnit};
    use uuid::Uuid;

    const CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn prepare_fake_gradle_home(dir: &TempDir) {
        let folders = vec![
            ".gradle",
            ".gradle/caches",
            ".gradle/caches/build-cache-1",
            ".gradle/configuration-cache",
            ".gradle/daemon",
            ".gradle/daemon/8.7",
        ];

        for folder in folders {
            fs::create_dir(dir.path().join(folder)).expect("Cant create temporary fixture folder");
        }
    }

    fn prepare_fake_gradle_projects(dir: &TempDir) {
        let folders = vec![
            "AndroidStudioProjects",
            "AndroidStudioProjects/my-project",
            "AndroidStudioProjects/my-project/build",
            "AndroidStudioProjects/my-project/.idea",
            "AndroidStudioProjects/my-project/.gradle",
        ];

        for folder in folders {
            fs::create_dir_all(dir.path().join(folder)).expect("Cant create temporary fixture folder");
        }

        let files = vec![
            "AndroidStudioProjects/my-project/settings.gradle",
            "AndroidStudioProjects/my-project/build.gradle",
            "AndroidStudioProjects/my-project/gradlew",
            "AndroidStudioProjects/my-project/gradle.properties",
        ];

        for file in files {
            fs::write(dir.path().join(file), "foo").expect("Cant create fixture file");
        }
    }

    fn create_fake_1kb_file(gradle_home: &TempDir, folder: &str) {
        let file_name = Uuid::new_v4();
        let relative_path = format!("{}/{}", folder, file_name);
        let complete_path = gradle_home.path().join(relative_path);

        let faker = StringFaker::with(Vec::from(CHARS), 1000);
        let fake: String = faker.fake();
        let mut fake_file = File::create(complete_path).expect("Cannot create temp file");
        fake_file
            .write_all(fake.as_bytes())
            .expect("Cannot write into temp file");
        fake_file.sync_all().expect("Cannot sync temp file with FileSystem");
    }

    #[test]
    fn should_compute_no_shared_caches_when_missing_gradle_home() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");
        let fake_gradle_home_path = temp_dir.path();

        let usages = resources_used_by_gradle_home(fake_gradle_home_path).expect("Cannot compute use cases");

        assert!(usages.is_empty())
    }

    #[test]
    fn should_compute_shared_caches_when_gradle_home_has_content() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");

        prepare_fake_gradle_home(&temp_dir);

        for _ in 0..2 {
            create_fake_1kb_file(&temp_dir, ".gradle/daemon/8.7");
        }

        for _ in 0..3 {
            create_fake_1kb_file(&temp_dir, ".gradle/caches/build-cache-1");
        }

        create_fake_1kb_file(&temp_dir, ".gradle/configuration-cache");

        let fake_gradle_home_path = temp_dir.path();
        let usages = resources_used_by_gradle_home(fake_gradle_home_path).expect("Cannot compute use cases");

        let expected = vec![
            AllocatedResource::new(
                UseCase::from(UserLevelDiskCache::GradleConfigurationCaching),
                1.kilobytes(),
            ),
            AllocatedResource::new(UseCase::from(UserLevelDiskCache::GradleBuildCaching), 3.kilobytes()),
            AllocatedResource::new(UseCase::from(UserLevelDiskCache::GradleDaemonLogs), 2.kilobytes()),
        ];

        assert_eq!(usages, expected);
    }

    #[test]
    fn should_compute_shared_caches_when_missing_maven_local() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");
        let fake_maven_local_path = temp_dir.path();

        let usage = resources_used_by_maven_local_repository(fake_maven_local_path).expect("Cannot compute use cases");

        let use_case = UseCase::from(UserLevelDiskCache::MavenLocalRepository);
        let expected = AllocatedResource::new(use_case, ByteUnit::from(0));
        assert_eq!(usage, expected)
    }

    #[test]
    fn should_compute_shared_caches_when_maven_local_present() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");
        let fake_maven_local_path = temp_dir.path();

        fs::create_dir(temp_dir.path().join(".m2")).expect("Cant create temporary fixture folder");

        for _ in 0..2 {
            create_fake_1kb_file(&temp_dir, ".m2");
        }

        let usage = resources_used_by_maven_local_repository(fake_maven_local_path).expect("Cannot compute use cases");

        let use_case = UseCase::from(UserLevelDiskCache::MavenLocalRepository);
        let expected = AllocatedResource::new(use_case, 2.kilobytes());
        assert_eq!(usage, expected)
    }

    #[test]
    fn should_not_compute_build_output_files_when_missing_gradle_projects() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");
        let fake_android_studio_projects_path = vec![temp_dir.path().to_path_buf()];

        let usage =
            resources_used_by_gradle_projects(&fake_android_studio_projects_path).expect("Cannot compute use cases");

        let use_case = UseCase::from(ProjectLevelDiskCache::BuildOutput);
        let expected = AllocatedResource::new(use_case, ByteUnit::from(0));
        assert_eq!(usage, expected)
    }

    #[test]
    fn should_compute_build_output_files_when_gradle_projects_present() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");

        prepare_fake_gradle_projects(&temp_dir);

        for _ in 0..5 {
            create_fake_1kb_file(&temp_dir, "AndroidStudioProjects/my-project/build");
        }

        let fake_android_studio_projects_path = vec![temp_dir.path().to_path_buf()];

        let usage =
            resources_used_by_gradle_projects(&fake_android_studio_projects_path).expect("Cannot compute use cases");

        let use_case = UseCase::from(ProjectLevelDiskCache::BuildOutput);
        let expected = AllocatedResource::new(use_case, 5.kilobytes());
        assert_eq!(usage, expected)
    }
}
