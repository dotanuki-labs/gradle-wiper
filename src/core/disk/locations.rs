// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::core::disk::user_home_locator;
use crate::core::models::{DiskCached, ProjectLevelDiskCache};
use cached::proc_macro::cached;
use itertools::Itertools;
use log::debug;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_gradle_home(user_home: &Path) -> PathBuf {
    user_home.to_path_buf().join(".gradle")
}

pub fn find_maven_local_repository(user_home: &Path) -> PathBuf {
    user_home.to_path_buf().join(".m2")
}

pub fn find_konan_caches(user_home: &Path) -> PathBuf {
    user_home.to_path_buf().join(".konan")
}

#[cached]
pub fn find_all_gradle_projects(user_home: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(user_home)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(standard_project_locations)
        .filter(ensure_gradle_project)
        .map(|entry| entry.into_path())
        .sorted()
        .collect::<Vec<_>>()
}

pub fn find_associated_filepaths(user_home: &Path, cached: DiskCached) -> Vec<PathBuf> {
    match cached {
        DiskCached::Standalone(project_level) => {
            let gradle_projects = find_all_gradle_projects(user_home.to_path_buf());

            if gradle_projects.is_empty() {
                return gradle_projects;
            };

            match project_level {
                ProjectLevelDiskCache::BuildOutput => all_build_output_folders(&gradle_projects),
                ProjectLevelDiskCache::GradleMetadata => all_metadata_files(&gradle_projects, ".gradle"),
                ProjectLevelDiskCache::IdeaMetadata => all_metadata_files(&gradle_projects, ".idea"),
            }
        },
        DiskCached::Shared(user_level) => match user_level.path_relative_to_user_home() {
            None => vec![],
            Some(path) => {
                let home_dir = user_home_locator();
                vec![home_dir.join(path)]
            },
        },
    }
}

fn standard_project_locations(entry: &DirEntry) -> bool {
    let entry_path = entry.path();
    let entry_path_raw = entry_path.to_str().expect("Not a valid path");

    entry_path_raw.contains("AndroidStudioProjects")
        || entry_path_raw.contains("IdeaProjects")
        || entry_path_raw.contains("Projects")
        || entry_path_raw.contains("Dev")
}

fn all_metadata_files(projects: &[PathBuf], target: &str) -> Vec<PathBuf> {
    projects.iter().map(|path| path.join(target)).collect::<Vec<_>>()
}

fn all_build_output_folders(projects: &[PathBuf]) -> Vec<PathBuf> {
    projects.iter().flat_map(find_build_output_dirs).collect::<Vec<_>>()
}

fn find_build_output_dirs(project: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(project)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().ends_with("build"))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>()
}

fn ensure_gradle_project(entry: &DirEntry) -> bool {
    let project_root = entry.path();

    let settings_file_groovy = project_root.join("settings.gradle");
    let settings_file_kotlin = project_root.join("settings.gradle.kts");
    let has_top_level_settings_config = settings_file_groovy.exists() || settings_file_kotlin.exists();

    let build_file_groovy = project_root.join("build.gradle");
    let build_file_kotlin = project_root.join("build.gradle.kts");
    let has_top_level_build_config = build_file_groovy.exists() || build_file_kotlin.exists();

    let has_top_level_gradlew = project_root.join("gradlew").exists();
    let has_top_level_gradle_properties = project_root.join("gradle.properties").exists();

    let verifications = vec![
        has_top_level_build_config,
        has_top_level_settings_config,
        has_top_level_gradlew,
        has_top_level_gradle_properties,
    ];

    let found = verifications.into_iter().all(|check| check);

    if found {
        let raw_path = entry.path().to_str().expect("Not a valid path");
        debug!("Found Gradle project -> {raw_path}");
    }

    found
}

#[cfg(test)]
mod tests {
    use crate::core::disk::find_all_gradle_projects;
    use itertools::Itertools;
    use std::fs;
    use temp_dir::TempDir;

    #[test]
    fn should_locate_no_projects() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");
        let fake_user_home = temp_dir.path().to_path_buf();
        let projects = find_all_gradle_projects(fake_user_home);

        assert!(projects.is_empty());
    }

    #[test]
    fn should_locate_existing_gradle_projects() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");

        let folders = [
            "AndroidStudioProjects",
            "AndroidStudioProjects/android-app",
            "IdeaProjects",
            "IdeaProjects/jvm-app",
        ];

        for folder in folders {
            fs::create_dir(temp_dir.path().join(folder)).expect("Cant create temporary fixture folder");
        }

        let files = [
            "AndroidStudioProjects/android-app/settings.gradle",
            "AndroidStudioProjects/android-app/build.gradle",
            "AndroidStudioProjects/android-app/gradlew",
            "AndroidStudioProjects/android-app/gradle.properties",
            "IdeaProjects/jvm-app/settings.gradle",
            "IdeaProjects/jvm-app/build.gradle",
            "IdeaProjects/jvm-app/gradlew",
            "IdeaProjects/jvm-app/gradle.properties",
        ];

        for file in files {
            fs::write(temp_dir.path().join(file), "foo").expect("Cant create fixture file");
        }

        let fake_user_home = temp_dir.path();
        let projects = find_all_gradle_projects(fake_user_home.to_path_buf());

        let expected = ["AndroidStudioProjects/android-app", "IdeaProjects/jvm-app"]
            .into_iter()
            .map(|item| fake_user_home.join(item))
            .sorted()
            .collect::<Vec<_>>();

        assert_eq!(projects, expected);
    }
}
