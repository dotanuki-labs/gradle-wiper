// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

#[allow(dead_code)]
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

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum MemoryCached {
    GradleWorkerDaemon,
    KotlinCompilerDaemon,
    GroovyCompilerDaemon,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum DiskCached {
    GradleCachedConfiguration,
    GradleCacheBuildTask,
    GradleDaemonLogs,
    GradleCachedJDKToolchain,
    GradleCachedDistribution,
    GradleOtherCaches,
    BuildOutputForGradleProject,
    MavenLocalStorage,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum UseCase {
    Disk(DiskCached),
    Memory(MemoryCached),
}

impl From<DiskCached> for UseCase {
    fn from(value: DiskCached) -> Self {
        UseCase::Disk(value)
    }
}

#[allow(dead_code)]
impl From<MemoryCached> for UseCase {
    fn from(value: MemoryCached) -> Self {
        UseCase::Memory(value)
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
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
