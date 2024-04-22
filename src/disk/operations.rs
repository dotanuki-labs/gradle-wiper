// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::PathBuf;

pub fn cleanup_resources(paths_to_remove: &[PathBuf]) {
    println!();
    println!("Removing the following :");
    println!();

    for path in paths_to_remove {
        println!("{}", path.to_str().expect("Not a valid path"));
    }

    println!();

    let errors = paths_to_remove
        .iter()
        .map(fs::remove_dir_all)
        .filter(|deletion| deletion.is_err())
        .map(|deletion| deletion.expect_err("Expecting an error").to_string())
        .collect::<Vec<String>>();

    if !errors.is_empty() {
        println!("Some of the target repositories were not removed (not found)")
    }
}
