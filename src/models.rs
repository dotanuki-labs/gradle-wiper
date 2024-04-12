// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use std::fmt::{Display, Formatter};
use ubyte::ByteUnit;

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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum UserLevelDiskCache {
    GradleConfigurationCaching,
    GradleBuildCaching,
    GradleDaemonLogs,
    GradleJDKToolchains,
    GradleDistributions,
    GradleTemporaryFiles,
    GradleNativeFiles,
    GradleBuildScans,
    GradleOtherCaches,
    MavenLocalRepository,
}

impl Display for UserLevelDiskCache {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            UserLevelDiskCache::GradleConfigurationCaching => "Gradle Configuration Caches",
            UserLevelDiskCache::GradleBuildCaching => "Gradle Build Caches",
            UserLevelDiskCache::GradleDaemonLogs => "Gradle Daemon Logs",
            UserLevelDiskCache::GradleJDKToolchains => "Gradle JDK toolchains",
            UserLevelDiskCache::GradleDistributions => "Gradle Distributions",
            UserLevelDiskCache::GradleTemporaryFiles => "Gradle Temporary Files",
            UserLevelDiskCache::GradleNativeFiles => "Gradle platform-native caches",
            UserLevelDiskCache::GradleBuildScans => "Gradle build-scans data",
            UserLevelDiskCache::GradleOtherCaches => "Other files on Gradle Home",
            UserLevelDiskCache::MavenLocalRepository => "Maven local repository",
        };

        formatter.write_str(name)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum ProjectLevelDiskCache {
    BuildOutput,
    GradleMetadata,
    IdeaMetadata,
}

impl Display for ProjectLevelDiskCache {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProjectLevelDiskCache::BuildOutput => "Build output files",
            ProjectLevelDiskCache::GradleMetadata => "Gradle metadata",
            ProjectLevelDiskCache::IdeaMetadata => "Idea metadata",
        };

        formatter.write_str(name)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum DiskCached {
    Shared(UserLevelDiskCache),
    Standalone(ProjectLevelDiskCache),
}

impl Display for DiskCached {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            DiskCached::Shared(user_level) => user_level.to_string(),
            DiskCached::Standalone(project_level) => project_level.to_string(),
        };

        formatter.write_str(&name)
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

impl From<ProjectLevelDiskCache> for UseCase {
    fn from(value: ProjectLevelDiskCache) -> Self {
        UseCase::Disk(DiskCached::Standalone(value))
    }
}

impl From<UserLevelDiskCache> for UseCase {
    fn from(value: UserLevelDiskCache) -> Self {
        UseCase::Disk(DiskCached::Shared(value))
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
