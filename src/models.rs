// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use std::fmt::{Display, Formatter};
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

impl Display for MemoryCached {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MemoryCached::GradleWorkerDaemon => "Gradle Daemon",
            MemoryCached::KotlinCompilerDaemon => "Kotlin Daemon",
            MemoryCached::GroovyCompilerDaemon => "Groovy Daemon",
        };

        formatter.write_str(name)
    }
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

impl Display for DiskCached {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            DiskCached::GradleConfigurationCaching => "Gradle Configuration Caches",
            DiskCached::GradleBuildCaching => "Gradle Build Caches",
            DiskCached::GradleDaemonLogs => "Gradle Daemon Logs",
            DiskCached::GradleJDKToolchains => "Gradle JDK toolchains",
            DiskCached::GradleDistributions => "Gradle Distributions",
            DiskCached::GradleTemporaryFiles => "Gradle Temporary Files",
            DiskCached::GradleNativeFiles => "Gradle platform-native caches",
            DiskCached::GradleBuildScans => "Gradle build-scans data",
            DiskCached::GradleOtherFiles => "Other files on Gradle Home",
            DiskCached::BuildOutputForGradleProject => "Build output on Gradle projects",
            DiskCached::MavenLocalStorage => "Maven Local storage",
        };

        formatter.write_str(name)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum UseCase {
    Disk(DiskCached),
    Memory(MemoryCached),
}

impl Display for UseCase {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            UseCase::Disk(disk_cached) => disk_cached.to_string(),
            UseCase::Memory(memory_cached) => memory_cached.to_string(),
        };

        formatter.write_str(&inner)
    }
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
pub struct AllocatedResource {
    pub use_case: UseCase,
    pub amount: ByteUnit,
}

impl AllocatedResource {
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
    pub resources: Vec<AllocatedResource>,
    pub total_size: ByteUnit,
}

impl EvaluationOutcome {
    pub fn new(resources: Vec<AllocatedResource>, total_size: ByteUnit) -> Self {
        Self { resources, total_size }
    }
}

#[allow(dead_code)]
pub enum ExecutionOutcome {
    Evaluation(EvaluationOutcome),
    Wipping(WippingOutcome),
}
