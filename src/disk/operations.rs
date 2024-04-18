// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::models::DiskCached;
use std::fs;

pub fn cleanup(resources: &[DiskCached]) {
    let paths_to_remove = resources
        .iter()
        .flat_map(crate::disk::locations::find_paths)
        .collect::<Vec<_>>();

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
