// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::PathBuf;

pub fn cleanup_resources(paths_to_remove: &[PathBuf]) {
    let errors = paths_to_remove
        .iter()
        .map(fs::remove_dir_all)
        .filter(|deletion| deletion.is_err())
        .map(|deletion| deletion.expect_err("Expecting an error").to_string())
        .collect::<Vec<String>>();

    for error in errors {
        eprintln!("{}", error);
    }
}
