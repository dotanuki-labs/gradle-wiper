// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use anyhow::anyhow;
use directories::BaseDirs;

#[allow(dead_code)]
pub fn find_gradle_home() -> anyhow::Result<PathBuf> {
    let base_dir = BaseDirs::new().ok_or(anyhow!("Cannot access base directories"))?;
    let home_dir = base_dir.home_dir();
    Ok(home_dir.join(".gradle"))
}

