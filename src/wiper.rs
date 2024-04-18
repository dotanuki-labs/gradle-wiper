// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::disk;
use crate::models::{
    AllocatedResource, DiskCached, EvaluationOutcome, ExecutionOutcome, MachineResource, UserLevelDiskCache,
    WipeAction, WipingOutcome,
};
use std::path::PathBuf;
use ubyte::{ByteUnit, ToByteUnit};
use MachineResource::{DiskSpace, RamMemory};
use WipeAction::{DeepWipe, Evaluate, ShallowWipe};

pub fn execute(target: &MachineResource, action: WipeAction) -> anyhow::Result<ExecutionOutcome> {
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
    // todo : real implementation to come
    let resources: Vec<AllocatedResource> = vec![];
    let outcome = EvaluationOutcome::new(resources, 0.megabytes());
    Ok(ExecutionOutcome::Evaluation(outcome))
}

fn shallow_wipe_ram() -> anyhow::Result<ExecutionOutcome> {
    todo!()
}

fn deep_wipe_ram() -> anyhow::Result<ExecutionOutcome> {
    todo!()
}

fn evaluate_disk_space() -> anyhow::Result<ExecutionOutcome> {
    let gradle_home = disk::find_gradle_home()?;
    let gradle_home_resources = disk::resources_used_by_gradle_home(gradle_home.as_path())?;
    let total_size_for_gradle_home = calculate_total_allocated(&gradle_home_resources);

    let maven_local_repository = disk::find_maven_local_repository()?;
    let maven_local_resources = disk::resources_used_by_maven_local_repository(maven_local_repository.as_path())?;
    let total_size_for_maven_local = maven_local_resources.amount;

    let gradle_projects = disk::find_all_gradle_projects()?;
    let gradle_projects_resources = disk::resources_used_by_gradle_projects(&gradle_projects)?;
    let total_size_for_gradle_projects = gradle_projects_resources.amount;

    let mut disk_resources: Vec<AllocatedResource> = Vec::new();
    disk_resources.extend(gradle_home_resources);
    disk_resources.push(maven_local_resources);
    disk_resources.push(gradle_projects_resources);

    let total_size_on_disk = total_size_for_gradle_home + total_size_for_maven_local + total_size_for_gradle_projects;
    let outcome = EvaluationOutcome::new(disk_resources, total_size_on_disk);
    Ok(ExecutionOutcome::Evaluation(outcome))
}

fn shallow_wipe_disk() -> anyhow::Result<ExecutionOutcome> {
    let before_cleaning = match evaluate_disk_space()? {
        ExecutionOutcome::Evaluation(outcome) => outcome.total_size,
        ExecutionOutcome::Wiping(_) => panic!("Expecting an evaluation, not a wipping"),
    };

    let caches_to_remove = vec![
        DiskCached::Shared(UserLevelDiskCache::GradleBuildCaching),
        DiskCached::Shared(UserLevelDiskCache::GradleConfigurationCaching),
        DiskCached::Shared(UserLevelDiskCache::GradleDaemonLogs),
        DiskCached::Shared(UserLevelDiskCache::GradleTemporaryFiles),
        DiskCached::Shared(UserLevelDiskCache::MavenLocalRepository),
    ];

    let paths_to_remove = caches_to_remove
        .into_iter()
        .flat_map(disk::find_associated_filepaths)
        .collect::<Vec<PathBuf>>();

    disk::cleanup_resources(&paths_to_remove);

    let after_cleaning = match evaluate_disk_space()? {
        ExecutionOutcome::Evaluation(outcome) => outcome.total_size,
        ExecutionOutcome::Wiping(_) => panic!("Expecting an evaluation, not a wipping"),
    };

    let outcome = WipingOutcome::new(DiskSpace, before_cleaning - after_cleaning);
    Ok(ExecutionOutcome::Wiping(outcome))
}

fn deep_wipe_ram_disk() -> anyhow::Result<ExecutionOutcome> {
    todo!()
}

fn calculate_total_allocated(resources: &[AllocatedResource]) -> ByteUnit {
    resources
        .iter()
        .fold(ByteUnit::from(0), |total, allocation| total + allocation.amount)
}
