// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::disk::usage_for_gradle_home;
use crate::filesystem::find_gradle_home;
use crate::models::{EvaluationOutcome, ExecutionOutcome, MachineResource, ResourceAllocation, WipeAction};
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
    let allocated: Vec<ResourceAllocation> = vec![];
    let outcome = EvaluationOutcome::new(allocated);
    Ok(ExecutionOutcome::Evaluation(outcome))
}

fn shallow_wipe_ram() -> anyhow::Result<ExecutionOutcome> {
    todo!()
}

fn deep_wipe_ram() -> anyhow::Result<ExecutionOutcome> {
    todo!()
}

fn evaluate_disk_space() -> anyhow::Result<ExecutionOutcome> {
    let gradle_home = find_gradle_home()?;
    let allocated = usage_for_gradle_home(gradle_home.as_path())?;
    let outcome = EvaluationOutcome::new(allocated);
    Ok(ExecutionOutcome::Evaluation(outcome))
}

fn shallow_wipe_disk() -> anyhow::Result<ExecutionOutcome> {
    todo!()
}

fn deep_wipe_ram_disk() -> anyhow::Result<ExecutionOutcome> {
    todo!()
}
