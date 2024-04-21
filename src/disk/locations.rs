// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::models::DiskCached;
use directories::BaseDirs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_gradle_home() -> anyhow::Result<PathBuf> {
    let base_dirs = BaseDirs::new().expect("Cannot access base directories");
    let home_dir = base_dirs.home_dir();
    Ok(home_dir.join(".gradle"))
}

pub fn find_maven_local_repository() -> anyhow::Result<PathBuf> {
    let base_dirs = BaseDirs::new().expect("Cannot access base directories");
    let home_dir = base_dirs.home_dir();
    Ok(home_dir.join(".m2"))
}

pub fn find_all_gradle_projects() -> anyhow::Result<Vec<PathBuf>> {
    let base_dirs = BaseDirs::new().expect("Cannot access base directories");
    let home_dir = base_dirs.home_dir();
    let all_projects = find_gradle_projects(home_dir)?;
    Ok(all_projects)
}

pub fn find_associated_filepaths(cached: DiskCached) -> Vec<PathBuf> {
    match cached {
        DiskCached::Standalone(_project_level) => {
            let gradle_projects = find_all_gradle_projects().unwrap_or_default();

            if gradle_projects.is_empty() {
                return gradle_projects;
            };

            let gradle_metadata = gradle_projects
                .iter()
                .map(|path| path.join(".gradle"))
                .collect::<Vec<_>>();

            let idea_metadata = gradle_projects
                .iter()
                .map(|path| path.join(".idea"))
                .collect::<Vec<_>>();

            let mut results: Vec<PathBuf> = Vec::new();
            results.extend(gradle_metadata);
            results.extend(idea_metadata);
            results
        },
        DiskCached::Shared(user_level) => match user_level.path_relative_to_user_home() {
            None => vec![],
            Some(path) => {
                let base_dirs = BaseDirs::new().expect("Cannot access base directories");
                let home_dir = base_dirs.home_dir();
                vec![home_dir.join(path)]
            },
        },
    }
}

fn find_gradle_projects(folder: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let projects = WalkDir::new(folder)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(ensure_gradle_project)
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();

    Ok(projects)
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
