// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod disk;
mod filesystem;
mod models;
mod wiper;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::new();
    let (target_resource, wipe_action) = cli.parsed_arguments();
    let outcome = wiper::execute(&target_resource, wipe_action)?;
    cli.show_execution_outcome(&target_resource, &outcome)
}
