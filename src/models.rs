// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

#[warn(dead_code)]
#[derive(Debug)]
pub enum MachineResource {
    RamMemory,
    DiskSpace,
}

#[derive(Debug)]
pub enum TidyAction {
    EvaluateResources,
    ShallowWiping,
    DeepWiping,
}

#[derive(Debug)]
pub enum UseCase {
    GradleBuildCache,
    GradleDaemonLogs,
    MavenLocalRepository,
    GradleProjectFiles,
    IdeaProjectFiles,
    GradleDaemon,
    KotlinDaemon,
    GroovyDaemon,
}

#[derive(Debug)]
pub struct ResourceAllocation {
    pub use_case: UseCase,
    pub megabytes: u64,
}

impl ResourceAllocation {
    pub fn new(what: UseCase, amount: u64) -> Self {
        Self {
            use_case: what,
            megabytes: amount,
        }
    }
}

pub struct ExecutionOutcome {
    pub subject: MachineResource,
    pub reclaimed_memory: u64,
    pub freed_entries: u32,
}

impl ExecutionOutcome {
    pub fn new(subject: MachineResource, reclaimed_memory: u64, freed_entries: u32) -> Self {
        Self {
            subject,
            reclaimed_memory,
            freed_entries,
        }
    }
}
