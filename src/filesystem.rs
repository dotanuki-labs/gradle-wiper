// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use anyhow::anyhow;
use directories::BaseDirs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_gradle_home() -> anyhow::Result<PathBuf> {
    let base_dir = BaseDirs::new().ok_or(anyhow!("Cannot access base directories"))?;
    let home_dir = base_dir.home_dir();
    Ok(home_dir.join(".gradle"))
}

pub fn find_maven_local_repository() -> anyhow::Result<PathBuf> {
    let base_dir = BaseDirs::new().ok_or(anyhow!("Cannot access base directories"))?;
    let home_dir = base_dir.home_dir();
    Ok(home_dir.join(".m2"))
}

pub fn find_all_gradle_projects() -> anyhow::Result<Vec<PathBuf>> {
    let base_dir = BaseDirs::new().ok_or(anyhow!("Cannot access base directories"))?;
    let home_dir = base_dir.home_dir();

    let android_studio_projects_folder = home_dir.join("AndroidStudioProjects");
    let android_projects = find_gradle_projects(android_studio_projects_folder.as_path())?;

    let intellij_projects_folder = home_dir.join("IdeaProjects");
    let jvm_projects = find_gradle_projects(intellij_projects_folder.as_path())?;

    let mut all_projects: Vec<PathBuf> = Vec::new();
    all_projects.extend(android_projects);
    all_projects.extend(jvm_projects);

    Ok(all_projects)
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
