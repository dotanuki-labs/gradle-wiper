// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::core::models::{AllocatedResource, MemoryCached, UseCase};
use crate::core::ram::jps::find_jvm_processes;
use itertools::Itertools;
use std::path::PathBuf;
use sysinfo::Pid;
use ubyte::ByteUnit;

pub fn find_resources_used_by_jvm(
    hsperfdata_locator: fn() -> PathBuf,
    resources_converter: fn(u32, String) -> Option<(MemoryCached, u64)>,
) -> anyhow::Result<Vec<AllocatedResource>> {
    let jvm_processes = find_jvm_processes(hsperfdata_locator)?;

    let resources = jvm_processes
        .into_iter()
        .filter_map(|(pid, class_name)| resources_converter(pid, class_name))
        .map(|(cached, memory)| (UseCase::from(cached), memory))
        .sorted_by_key(|item| item.0)
        .chunk_by(|item| item.0)
        .into_iter()
        .map(|(use_case, group)| {
            (
                use_case,
                group.fold(ByteUnit::from(0), |total, (_, entry_size)| total + entry_size),
            )
        })
        .map(|(use_case, total_memory)| AllocatedResource::new(use_case, total_memory))
        .collect::<Vec<_>>();

    Ok(resources)
}

pub fn convert_to_allocated_resources(pid: u32, launcher_class_name: String) -> Option<(MemoryCached, u64)> {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    system.process(Pid::from_u32(pid)).map(|process| {
        (
            memory_type_from_jvm_launcher_class(&launcher_class_name),
            process.memory(),
        )
    })
}

pub fn memory_type_from_jvm_launcher_class(launcher_class_name: &str) -> MemoryCached {
    match launcher_class_name {
        _ if launcher_class_name.to_lowercase().contains("kotlin") => MemoryCached::KotlinCompilerDaemon,
        _ if launcher_class_name.to_lowercase().contains("gradle") => MemoryCached::GradleBuildDaemon,
        _ => MemoryCached::OtherJavaProcess,
    }
}

#[cfg(test)]
mod tests {
    use crate::core::models::{AllocatedResource, MemoryCached, UseCase};
    use crate::core::ram::find_resources_used_by_jvm;
    use std::path::PathBuf;
    use ubyte::{ByteUnit, ToByteUnit};

    fn locate_fake_hsperdata() -> PathBuf {
        let root_dir = std::env::current_dir().expect("Cannot find current dir");
        root_dir.join("test-data").join("hsperf")
    }

    fn fake_resources_converter(_: u32, launcher_name: String) -> Option<(MemoryCached, u64)> {
        let converted = match &launcher_name {
            _ if launcher_name.to_lowercase().contains("kotlin") => {
                (MemoryCached::KotlinCompilerDaemon, 1.gibibytes().as_u64())
            },
            _ if launcher_name.to_lowercase().contains("gradle") => {
                (MemoryCached::GradleBuildDaemon, 2.gibibytes().as_u64())
            },
            _ => (MemoryCached::OtherJavaProcess, 3.gibibytes().as_u64()),
        };

        Some(converted)
    }

    #[test]
    fn should_evaluate_ram_memory_resources() {
        let resources = find_resources_used_by_jvm(locate_fake_hsperdata, fake_resources_converter)
            .expect("Cannot evaluate resources");

        let grouped = vec![
            (MemoryCached::GradleBuildDaemon, 6442450944u64),
            (MemoryCached::KotlinCompilerDaemon, 2147483648u64),
            (MemoryCached::OtherJavaProcess, 3221225472u64),
        ];

        let expected = grouped
            .into_iter()
            .map(|(cached, amount)| AllocatedResource::new(UseCase::from(cached), ByteUnit::from(amount)))
            .collect::<Vec<_>>();

        assert_eq!(resources, expected)
    }
}
