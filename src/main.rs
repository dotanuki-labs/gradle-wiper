// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod models;

use crate::cli::Cli;
use crate::models::UseCase::{GradleDaemon, KotlinDaemon};
use crate::models::{ExecutionOutcome, MachineResource, ResourceAllocation};

fn main() {
    let cli = Cli::new();
    let (machine_resources, action) = cli.parsed_arguments();

    match machine_resources {
        MachineResource::DiskSpace => {
            let outcome = ExecutionOutcome::new(MachineResource::DiskSpace, 2400, 4);
            cli.show_execution_outcome(&outcome);
        },
        MachineResource::RamMemory => {
            let allocations = vec![
                ResourceAllocation::new(KotlinDaemon, 440),
                ResourceAllocation::new(KotlinDaemon, 410),
                ResourceAllocation::new(GradleDaemon, 815),
                ResourceAllocation::new(GradleDaemon, 750),
            ];

            cli.show_allocated_resources(&allocations)
        },
    }
}
