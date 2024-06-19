// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use ubyte::ByteUnit;

#[derive(Debug)]
pub enum MachineResource {
    RamMemory,
    DiskSpace,
}

impl Display for MachineResource {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            MachineResource::RamMemory => "RAM memory",
            MachineResource::DiskSpace => "disk space",
        };

        formatter.write_str(formatted)
    }
}

#[derive(Debug)]
pub enum WipeAction {
    Evaluate,
    ShallowWipe,
    DeepWipe,
}

impl Display for WipeAction {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            WipeAction::Evaluate => "evaluation",
            WipeAction::ShallowWipe => "shallow wiping",
            WipeAction::DeepWipe => "deep wiping",
        };

        formatter.write_str(formatted)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum MemoryCached {
    GradleBuildDaemon,
    KotlinCompilerDaemon,
    OtherJavaProcess,
}

impl Display for MemoryCached {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MemoryCached::GradleBuildDaemon => "Gradle Daemon",
            MemoryCached::KotlinCompilerDaemon => "Kotlin Daemon",
            MemoryCached::OtherJavaProcess => "Other JVM process",
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
    KonanCaches,
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
            UserLevelDiskCache::KonanCaches => "Konan/KMP Caches",
        };

        formatter.write_str(name)
    }
}

impl UserLevelDiskCache {
    pub fn path_relative_to_user_home(&self) -> Option<PathBuf> {
        let raw_path = match self {
            UserLevelDiskCache::GradleConfigurationCaching => ".gradle/configuration-cache",
            UserLevelDiskCache::GradleBuildCaching => ".gradle/caches",
            UserLevelDiskCache::GradleDaemonLogs => ".gradle/daemon",
            UserLevelDiskCache::GradleJDKToolchains => ".gradle/jdks",
            UserLevelDiskCache::GradleDistributions => ".gradle/wrapper",
            UserLevelDiskCache::GradleTemporaryFiles => ".gradle/.tmp",
            UserLevelDiskCache::GradleNativeFiles => ".gradle/native",
            UserLevelDiskCache::GradleBuildScans => ".gradle/build-scan-data",
            UserLevelDiskCache::MavenLocalRepository => ".m2",
            UserLevelDiskCache::KonanCaches => ".konan",
            UserLevelDiskCache::GradleOtherCaches => "",
        };

        if raw_path.is_empty() {
            return None;
        };

        Some(PathBuf::from(raw_path))
    }
}

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

pub struct WipingOutcome {
    pub reclaimed: ByteUnit,
}

impl WipingOutcome {
    pub fn new(reclaimed: ByteUnit) -> Self {
        Self { reclaimed }
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

pub enum ExecutionOutcome {
    Evaluation(EvaluationOutcome),
    Wiping(WipingOutcome),
}

impl ExecutionOutcome {
    pub fn as_evaluation(&self) -> &EvaluationOutcome {
        match self {
            ExecutionOutcome::Evaluation(evaluation) => evaluation,
            ExecutionOutcome::Wiping(_) => panic!("Not an evaluation"),
        }
    }
}
