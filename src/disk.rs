// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::models::{DiskCached, ResourceAllocation, UseCase};
use itertools::Itertools;
use std::path::Path;
use ubyte::ByteUnit;
use walkdir::{DirEntry, WalkDir};

#[allow(dead_code)]
pub fn usage_for_gradle_home(gradle_home: &Path) -> anyhow::Result<Vec<ResourceAllocation>> {
    // We trust that gradle_home == $HOME/.gradle
    let Ok(true) = gradle_home.try_exists() else {
        return Ok(vec![]);
    };

    let total_per_use_case = WalkDir::new(gradle_home)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(ensure_file)
        .map(|entry| (size_for_entry(&entry), evaluate_use_case_from_gradle_home(&entry)))
        .group_by(|item| item.1.clone())
        .into_iter()
        .map(|(use_case, group)| (use_case, group.fold(0, |total, (entry_size, _)| total + entry_size)))
        .map(|(use_case, total)| ResourceAllocation::new(use_case, ByteUnit::from(total)))
        .sorted_by_key(|item| item.use_case.clone())
        .collect::<Vec<_>>();

    Ok(total_per_use_case)
}

#[allow(dead_code)]
fn size_for_entry(entry: &DirEntry) -> u64 {
    entry.metadata().expect("Expecting a valid metadata for entry").len()
}

#[allow(dead_code)]
fn ensure_file(entry: &DirEntry) -> bool {
    entry
        .metadata()
        .expect("Expecting a valid metadata for entry")
        .is_file()
}

#[allow(dead_code)]
fn evaluate_use_case_from_gradle_home(entry: &DirEntry) -> UseCase {
    let raw_path = entry
        .path()
        .to_str()
        .expect("Should be able to evaluate path as string");

    // https://docs.gradle.org/current/userguide/configuration_cache.html
    // https://docs.gradle.org/current/userguide/directory_layout.html
    let cache_type = match raw_path {
        _ if raw_path.contains(".gradle/caches") => DiskCached::GradleCacheBuildTask,
        _ if raw_path.contains(".gradle/configuration-cache") => DiskCached::GradleCachedConfiguration,
        _ if raw_path.contains(".gradle/daemon") => DiskCached::GradleDaemonLogs,
        _ if raw_path.contains(".gradle/jdks") => DiskCached::GradleCachedJDKToolchain,
        _ if raw_path.contains(".gradle/wrapper") => DiskCached::GradleCachedDistribution,
        _ => DiskCached::GradleOtherCaches,
    };

    UseCase::from(cache_type)
}

#[cfg(test)]
mod tests {
    use crate::disk::usage_for_gradle_home;
    use crate::models::{DiskCached, ResourceAllocation, UseCase};
    use fake::{Fake, StringFaker};
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use temp_dir::TempDir;
    use ubyte::ToByteUnit;
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
    fn should_track_no_use_cases_when_missing_gradle_home_() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");
        let fake_gradle_home_path = temp_dir.path();

        let use_cases = usage_for_gradle_home(fake_gradle_home_path).expect("Cannot compute use cases");

        assert!(use_cases.is_empty())
    }

    #[test]
    fn should_handle_gradle_home_with_different_files() {
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
        let use_cases = usage_for_gradle_home(fake_gradle_home_path).expect("Cannot compute use cases");

        let expected = vec![
            ResourceAllocation::new(UseCase::from(DiskCached::GradleCachedConfiguration), 1.kilobytes()),
            ResourceAllocation::new(UseCase::from(DiskCached::GradleCacheBuildTask), 3.kilobytes()),
            ResourceAllocation::new(UseCase::from(DiskCached::GradleDaemonLogs), 2.kilobytes()),
        ];

        assert_eq!(use_cases, expected);
    }
}
