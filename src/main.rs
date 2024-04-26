// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod disk;
mod models;
mod ram;
mod wiper;

fn main() -> anyhow::Result<()> {
    let (target_resource, wipe_action, verbose_mode) = cli::parsed_arguments();
    cli::setup_logging(verbose_mode);

    let outcome = wiper::execute(&target_resource, wipe_action)?;
    cli::show_execution_outcome(&target_resource, &outcome)
}
