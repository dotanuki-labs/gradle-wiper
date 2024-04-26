// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod jps;
mod resources;
mod wiping;

use std::path::PathBuf;

pub use resources::*;
pub use wiping::*;

pub fn locate_hsperfdata_dir() -> PathBuf {
    let system_tmp_dir = std::env::temp_dir();
    let username = whoami::username();

    let jvm_perf_data_dir = format!("hsperfdata_{}", username);
    system_tmp_dir.join(jvm_perf_data_dir)
}
