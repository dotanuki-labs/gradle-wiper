// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod disk;
mod filesystem;
mod models;

use crate::cli::Cli;
use crate::models::MemoryCached::{GradleWorkerDaemon, KotlinCompilerDaemon};
use crate::models::{ExecutionOutcome, MachineResource, ResourceAllocation, UseCase};
use ubyte::ToByteUnit;

fn main() {
    let cli = Cli::new();
    let (machine_resources, _action) = cli.parsed_arguments();

    match machine_resources {
        MachineResource::DiskSpace => {
            let outcome = ExecutionOutcome::new(MachineResource::DiskSpace, 2400.megabytes(), 4);
            cli.show_execution_outcome(&outcome);
        },
        MachineResource::RamMemory => {
            let allocations = vec![
                ResourceAllocation::new(UseCase::from(KotlinCompilerDaemon), 440.megabytes()),
                ResourceAllocation::new(UseCase::from(KotlinCompilerDaemon), 410.megabytes()),
                ResourceAllocation::new(UseCase::from(GradleWorkerDaemon), 815.megabytes()),
                ResourceAllocation::new(UseCase::from(GradleWorkerDaemon), 750.megabytes()),
            ];

            cli.show_allocated_resources(&allocations)
        },
    }
}
