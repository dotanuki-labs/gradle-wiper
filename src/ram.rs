// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::models::{AllocatedResource, MemoryCached, UseCase};
use itertools::Itertools;
use rust_strings::BytesConfig;
use std::fs;
use std::path::{Path, PathBuf};
use sysinfo::Pid;
use ubyte::ByteUnit;

pub fn locate_hsperfdata_dir() -> PathBuf {
    let system_tmp_dir = std::env::temp_dir();
    let username = whoami::username();

    let jvm_perf_data_dir = format!("hsperfdata_{}", username);
    system_tmp_dir.join(jvm_perf_data_dir)
}

pub fn find_resources_used_by_jvm(
    hsperfdata_locator: fn() -> PathBuf,
    resources_converter: fn(u32, String) -> (MemoryCached, u64),
) -> anyhow::Result<Vec<AllocatedResource>> {
    let jvm_perf_data_path = hsperfdata_locator();

    dbg!(jvm_perf_data_path.as_path());

    let jps_paths = fs::read_dir(jvm_perf_data_path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    let pids = jps_paths
        .iter()
        .map(|pathbuf| pathbuf.as_path())
        .map(pid_from_jps_file)
        .collect::<Vec<_>>();

    let launcher_class_names = jps_paths
        .iter()
        .map(|pathbuf| pathbuf.as_path())
        .map(launcher_class_from_monitored_process)
        .collect::<Vec<_>>();

    let resources = pids
        .into_iter()
        .zip(launcher_class_names)
        .filter(|(_, class_name)| !class_name.contains("sun"))
        .map(|(pid, class_name)| resources_converter(pid, class_name))
        .sorted_by_key(|item| item.0)
        .map(|(cached, memory)| (UseCase::from(cached), memory))
        .group_by(|item| item.0)
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

pub fn convert_to_allocated_resources(pid: u32, launcher_class_name: String) -> (MemoryCached, u64) {
    let memory_type = match &launcher_class_name {
        _ if launcher_class_name.to_lowercase().contains("kotlin") => MemoryCached::KotlinCompilerDaemon,
        _ if launcher_class_name.to_lowercase().contains("gradle") => MemoryCached::GradleBuildDaemon,
        _ => MemoryCached::OtherJavaProcess,
    };

    let system = sysinfo::System::new_all();
    let process = system.process(Pid::from_u32(pid)).expect("Cannot find JVM process");

    (memory_type, process.memory())
}

fn pid_from_jps_file(path_to_file: &Path) -> u32 {
    let file_path_raw_str = path_to_file.file_name().expect("Not a valid path").to_string_lossy();
    file_path_raw_str.parse::<u32>().expect("Cannot parse filename as u32")
}

fn launcher_class_from_monitored_process(path_to_file: &Path) -> String {
    let file_contents_as_bytes = fs::read(path_to_file).expect("Cannot read jps binary file");
    let parsing_config = BytesConfig::new(file_contents_as_bytes);
    let parsed_binary_info = rust_strings::strings(&parsing_config).expect("Cannot parse jps binary");
    let parsed_strings = parsed_binary_info
        .into_iter()
        .map(|(parsed, _)| parsed)
        .collect::<Vec<_>>();

    let (java_cmd_index, _) = parsed_strings
        .iter()
        .enumerate()
        .find(|(_, cmd)| cmd.contains("sun.rt.javaCommand"))
        .expect("Cannot find javaCommand");

    let launcher_class_name = parsed_strings.get(java_cmd_index + 1).expect("Cannot get javaCommand");
    let sanitized_command_name = launcher_class_name
        .split_whitespace()
        .next()
        .expect("Cannot extract sanitized process name");
    String::from(sanitized_command_name)
}

#[cfg(test)]
mod tests {
    use crate::models::{AllocatedResource, MemoryCached, UseCase};
    use crate::ram::find_resources_used_by_jvm;
    use std::path::PathBuf;
    use ubyte::{ByteUnit, ToByteUnit};

    fn locate_fake_hsperdata() -> PathBuf {
        let root_dir = std::env::current_dir().expect("Cannot find current dir");
        root_dir.join("test-data").join("hsperf")
    }

    fn fake_resources_converter(_: u32, launcher_name: String) -> (MemoryCached, u64) {
        match &launcher_name {
            _ if launcher_name.to_lowercase().contains("kotlin") => {
                (MemoryCached::KotlinCompilerDaemon, 1.gibibytes().as_u64())
            },
            _ if launcher_name.to_lowercase().contains("gradle") => {
                (MemoryCached::GradleBuildDaemon, 2.gibibytes().as_u64())
            },
            _ => (MemoryCached::OtherJavaProcess, 3.gibibytes().as_u64()),
        }
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
