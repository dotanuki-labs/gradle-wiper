// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::core::wiper;

mod cli;
mod core;

fn main() -> anyhow::Result<()> {
    let (target_resource, wipe_action, verbose_mode) = cli::parsed_arguments();
    cli::setup_logging(verbose_mode);

    let outcome = wiper::execute(&target_resource, wipe_action)?;
    cli::show_execution_outcome(&target_resource, &outcome)
}
