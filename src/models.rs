// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use ubyte::ByteUnit;

#[allow(dead_code)]
#[derive(Debug)]
pub enum MachineResource {
    RamMemory,
    DiskSpace,
}

#[derive(Debug)]
pub enum WipeAction {
    Evaluate,
    ShallowWipe,
    DeepWipe,
}

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum MemoryCached {
    GradleWorkerDaemon,
    KotlinCompilerDaemon,
    GroovyCompilerDaemon,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum DiskCached {
    GradleConfigurationCaching,
    GradleBuildCaching,
    GradleDaemonLogs,
    GradleJDKToolchains,
    GradleDistributions,
    GradleTemporaryFiles,
    GradleNativeFiles,
    GradleBuildScans,
    GradleOtherFiles,
    BuildOutputForGradleProject,
    MavenLocalStorage,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
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
    pub amount: ByteUnit,
}

impl ResourceAllocation {
    pub fn new(use_case: UseCase, amount: ByteUnit) -> Self {
        Self { use_case, amount }
    }
}

pub struct WippingOutcome {
    pub subject: MachineResource,
    pub reclaimed_memory: ByteUnit,
    pub freed_entries: u32,
}

#[allow(dead_code)]
impl WippingOutcome {
    pub fn new(subject: MachineResource, reclaimed_memory: ByteUnit, freed_entries: u32) -> Self {
        Self {
            subject,
            reclaimed_memory,
            freed_entries,
        }
    }
}

pub struct EvaluationOutcome {
    pub allocated: Vec<ResourceAllocation>,
}

impl EvaluationOutcome {
    pub fn new(allocated: Vec<ResourceAllocation>) -> Self {
        Self { allocated }
    }
}

#[allow(dead_code)]
pub enum ExecutionOutcome {
    Evaluation(EvaluationOutcome),
    Wipping(WippingOutcome),
}
