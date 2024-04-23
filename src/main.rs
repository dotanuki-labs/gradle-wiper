// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod disk;
mod logging;
mod models;
mod wiper;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::new();
    let (target_resource, wipe_action, verbose_mode) = cli.parsed_arguments();
    logging::setup(verbose_mode);

    let outcome = wiper::execute(&target_resource, wipe_action)?;
    cli.show_execution_outcome(&target_resource, &outcome)
}
