// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use anyhow::anyhow;
use directories::BaseDirs;
use std::path::PathBuf;

pub fn find_gradle_home() -> anyhow::Result<PathBuf> {
    let base_dir = BaseDirs::new().ok_or(anyhow!("Cannot access base directories"))?;
    let home_dir = base_dir.home_dir();
    Ok(home_dir.join(".gradle"))
}
