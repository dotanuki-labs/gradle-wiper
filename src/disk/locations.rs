// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::disk::user_home_locator;
use crate::models::{DiskCached, ProjectLevelDiskCache};
use cached::proc_macro::cached;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_gradle_home(user_home: &Path) -> PathBuf {
    user_home.to_path_buf().join(".gradle")
}

pub fn find_maven_local_repository(user_home: &Path) -> PathBuf {
    user_home.to_path_buf().join(".m2")
}

#[cached]
pub fn find_all_gradle_projects(user_home: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(user_home)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(ensure_gradle_project)
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>()
}

pub fn find_associated_filepaths(user_home: PathBuf, cached: DiskCached) -> Vec<PathBuf> {
    match cached {
        DiskCached::Standalone(project_level) => {
            let gradle_projects = find_all_gradle_projects(user_home);

            if gradle_projects.is_empty() {
                return gradle_projects;
            };

            let target = match project_level {
                ProjectLevelDiskCache::BuildOutput => "build",
                ProjectLevelDiskCache::GradleMetadata => ".gradle",
                ProjectLevelDiskCache::IdeaMetadata => ".idea",
            };

            gradle_projects.iter().map(|path| path.join(target)).collect::<Vec<_>>()
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

    verifications.into_iter().all(|check| check)
}

#[cfg(test)]
mod tests {
    use crate::disk::find_all_gradle_projects;
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
            fs::create_dir(&temp_dir.path().join(folder)).expect("Cant create temporary fixture folder");
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
            fs::write(&temp_dir.path().join(file), "foo").expect("Cant create fixture file");
        }

        let fake_user_home = temp_dir.path().to_path_buf();
        let projects = find_all_gradle_projects(fake_user_home.clone());

        let expected = ["IdeaProjects/jvm-app", "AndroidStudioProjects/android-app"]
            .into_iter()
            .map(|item| fake_user_home.join(item))
            .collect::<Vec<_>>();

        assert_eq!(projects, expected);
    }
}
