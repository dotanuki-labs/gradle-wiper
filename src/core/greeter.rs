// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

pub fn greet(name: &str) -> anyhow::Result<String> {
    Ok(format!("Hello, {}!", name))
}
