// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use log::LevelFilter;

pub fn setup(verbose: bool) {
    let filter = if verbose { LevelFilter::Debug } else { LevelFilter::Info };

    env_logger::Builder::new()
        .filter_level(filter)
        .format_timestamp(None)
        .format_level(false)
        .format_target(false)
        .init();
}
