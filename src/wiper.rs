// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::disk;
use crate::models::{
    AllocatedResource, DiskCached, EvaluationOutcome, ExecutionOutcome, MachineResource, MemoryCached,
    ProjectLevelDiskCache, UserLevelDiskCache, WipeAction, WipingOutcome,
};
use crate::ram::{convert_to_allocated_resources, find_resources_used_by_jvm, locate_hsperfdata_dir, wipe_ram};
use log::debug;
use std::path::PathBuf;
use ubyte::ByteUnit;
use MachineResource::{DiskSpace, RamMemory};
use WipeAction::{DeepWipe, Evaluate, ShallowWipe};

pub fn execute(target: &MachineResource, action: WipeAction) -> anyhow::Result<ExecutionOutcome> {
    debug!("");
    debug!("Machine resource : {}", target);
    debug!("Requested operation : {}", action);

    match (target, action) {
        (RamMemory, Evaluate) => evaluate_ram_memory(),
        (RamMemory, ShallowWipe) => shallow_wipe_ram(),
        (RamMemory, DeepWipe) => deep_wipe_ram(),
        (DiskSpace, Evaluate) => evaluate_disk_space(),
        (DiskSpace, ShallowWipe) => shallow_wipe_disk(),
        (DiskSpace, DeepWipe) => deep_wipe_ram_disk(),
    }
}

fn evaluate_ram_memory() -> anyhow::Result<ExecutionOutcome> {
    let resources = find_resources_used_by_jvm(locate_hsperfdata_dir, convert_to_allocated_resources)?;
    let total_memory = calculate_total_allocated(&resources);
    let outcome = EvaluationOutcome::new(resources, total_memory);
    Ok(ExecutionOutcome::Evaluation(outcome))
}

fn shallow_wipe_ram() -> anyhow::Result<ExecutionOutcome> {
    let resources_before = find_resources_used_by_jvm(locate_hsperfdata_dir, convert_to_allocated_resources)?;
    let total_memory_before = calculate_total_allocated(&resources_before);

    let caches_to_remove = vec![MemoryCached::GradleBuildDaemon, MemoryCached::KotlinCompilerDaemon];

    wipe_ram(locate_hsperfdata_dir, &caches_to_remove);

    let resources_after = find_resources_used_by_jvm(locate_hsperfdata_dir, convert_to_allocated_resources)?;
    let total_memory_after = calculate_total_allocated(&resources_after);

    let reclaimed = total_memory_before - total_memory_after;

    let outcome = WipingOutcome::new(RamMemory, reclaimed);
    Ok(ExecutionOutcome::Wiping(outcome))
}

fn deep_wipe_ram() -> anyhow::Result<ExecutionOutcome> {
    todo!()
}

fn evaluate_disk_space() -> anyhow::Result<ExecutionOutcome> {
    let user_home = disk::user_home_locator();
    let gradle_home = disk::find_gradle_home(user_home.as_path());
    let gradle_home_resources = disk::resources_used_by_gradle_home(gradle_home.as_path())?;
    let total_size_for_gradle_home = calculate_total_allocated(&gradle_home_resources);

    if gradle_home.exists() {
        debug!("Gradle home : {}", &gradle_home.to_string_lossy());
        debug!("Storage taken by Gradle caches : {}", total_size_for_gradle_home);
    }

    let maven_local_repository = disk::find_maven_local_repository(user_home.as_path());
    let maven_local_resources = disk::resources_used_by_maven_local_repository(maven_local_repository.as_path())?;
    let total_size_for_maven_local = maven_local_resources.amount;

    if maven_local_repository.exists() {
        debug!(
            "Maven local repository path : {}",
            &maven_local_repository.to_string_lossy()
        );
        debug!("Storage taken by Maven local : {}", total_size_for_maven_local);
    }

    let konan_caches = disk::find_konan_caches(user_home.as_path());
    let konan_resources = disk::resources_used_by_konan(konan_caches.as_path())?;
    let total_size_for_konan_caches = konan_resources.amount;

    if konan_caches.exists() {
        debug!("Konan caches path : {}", &konan_caches.to_string_lossy());
        debug!("Storage taken by Konan : {}", total_size_for_konan_caches);
    }

    let gradle_projects = disk::find_all_gradle_projects(user_home);
    let gradle_projects_resources = disk::resources_used_by_gradle_projects(&gradle_projects)?;
    let total_size_for_gradle_projects = gradle_projects_resources.amount;

    let mut disk_resources: Vec<AllocatedResource> = Vec::new();
    disk_resources.extend(gradle_home_resources);
    disk_resources.push(maven_local_resources);
    disk_resources.push(konan_resources);
    disk_resources.push(gradle_projects_resources);

    let total_cached = total_size_for_konan_caches
        + total_size_for_gradle_home
        + total_size_for_maven_local
        + total_size_for_gradle_projects;

    let outcome = EvaluationOutcome::new(disk_resources, total_cached);
    Ok(ExecutionOutcome::Evaluation(outcome))
}

fn shallow_wipe_disk() -> anyhow::Result<ExecutionOutcome> {
    let caches_to_remove = vec![
        DiskCached::Shared(UserLevelDiskCache::GradleBuildCaching),
        DiskCached::Shared(UserLevelDiskCache::GradleConfigurationCaching),
        DiskCached::Shared(UserLevelDiskCache::GradleDaemonLogs),
        DiskCached::Shared(UserLevelDiskCache::GradleTemporaryFiles),
        DiskCached::Shared(UserLevelDiskCache::MavenLocalRepository),
        DiskCached::Shared(UserLevelDiskCache::KonanCaches),
        DiskCached::Standalone(ProjectLevelDiskCache::BuildOutput),
    ];

    wipe_disk(caches_to_remove)
}

fn deep_wipe_ram_disk() -> anyhow::Result<ExecutionOutcome> {
    let caches_to_remove = vec![
        DiskCached::Shared(UserLevelDiskCache::GradleBuildCaching),
        DiskCached::Shared(UserLevelDiskCache::GradleConfigurationCaching),
        DiskCached::Shared(UserLevelDiskCache::GradleDaemonLogs),
        DiskCached::Shared(UserLevelDiskCache::GradleTemporaryFiles),
        DiskCached::Shared(UserLevelDiskCache::MavenLocalRepository),
        DiskCached::Shared(UserLevelDiskCache::KonanCaches),
        DiskCached::Shared(UserLevelDiskCache::GradleJDKToolchains),
        DiskCached::Shared(UserLevelDiskCache::GradleNativeFiles),
        DiskCached::Shared(UserLevelDiskCache::GradleBuildScans),
        DiskCached::Shared(UserLevelDiskCache::GradleDistributions),
        DiskCached::Standalone(ProjectLevelDiskCache::BuildOutput),
        DiskCached::Standalone(ProjectLevelDiskCache::GradleMetadata),
        DiskCached::Standalone(ProjectLevelDiskCache::IdeaMetadata),
    ];

    wipe_disk(caches_to_remove)
}

fn wipe_disk(caches_to_remove: Vec<DiskCached>) -> anyhow::Result<ExecutionOutcome> {
    let before_cleaning = evaluate_disk_space()?;
    let before_cleaning_evaluation = before_cleaning.as_evaluation();

    let paths_to_remove = caches_to_remove
        .into_iter()
        .flat_map(|item| disk::find_associated_filepaths(disk::user_home_locator(), item))
        .collect::<Vec<PathBuf>>();

    disk::cleanup_resources(&paths_to_remove);

    let after_cleaning = evaluate_disk_space()?;
    let after_cleaning_evaluation = after_cleaning.as_evaluation();
    let reclaimed = before_cleaning_evaluation.total_size - after_cleaning_evaluation.total_size;
    let outcome = WipingOutcome::new(DiskSpace, reclaimed);

    Ok(ExecutionOutcome::Wiping(outcome))
}

fn calculate_total_allocated(resources: &[AllocatedResource]) -> ByteUnit {
    resources
        .iter()
        .fold(ByteUnit::from(0), |total, allocation| total + allocation.amount)
}
