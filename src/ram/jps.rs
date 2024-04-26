// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use rust_strings::BytesConfig;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_jvm_processes(hsperfdata_locator: fn() -> PathBuf) -> anyhow::Result<Vec<(u32, String)>> {
    let jvm_perf_data_path = hsperfdata_locator();

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

    let processes = pids
        .into_iter()
        .zip(launcher_class_names)
        .filter(|(_, class_name)| !class_name.contains("sun"))
        .collect::<Vec<_>>();

    Ok(processes)
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
