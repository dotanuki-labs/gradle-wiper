// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use human_panic::setup_panic;
use log::LevelFilter;

pub fn setup_logging(verbose: bool) {
    setup_panic!();
    better_panic::install();

    let filter = if verbose { LevelFilter::Debug } else { LevelFilter::Info };

    env_logger::Builder::new()
        .filter_level(filter)
        .format_timestamp(None)
        .format_level(false)
        .format_target(false)
        .init();
}
