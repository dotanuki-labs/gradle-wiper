// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod disk;
mod models;

use crate::cli::Cli;
use crate::models::MemoryCached::{GradleWorkerDaemon, KotlinCompilerDaemon};
use crate::models::{ExecutionOutcome, MachineResource, ResourceAllocation, UseCase};

fn main() {
    let cli = Cli::new();
    let (machine_resources, action) = cli.parsed_arguments();
    println!("Action -> {:?}", action);

    match machine_resources {
        MachineResource::DiskSpace => {
            let outcome = ExecutionOutcome::new(MachineResource::DiskSpace, 2400, 4);
            cli.show_execution_outcome(&outcome);
        },
        MachineResource::RamMemory => {
            let allocations = vec![
                ResourceAllocation::new(UseCase::from(KotlinCompilerDaemon), 440),
                ResourceAllocation::new(UseCase::from(KotlinCompilerDaemon), 410),
                ResourceAllocation::new(UseCase::from(GradleWorkerDaemon), 815),
                ResourceAllocation::new(UseCase::from(GradleWorkerDaemon), 750),
            ];

            cli.show_allocated_resources(&allocations)
        },
    }
}
