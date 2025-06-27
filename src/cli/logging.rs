// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use human_panic::{metadata, setup_panic};
use log::LevelFilter;

pub fn setup_logging(verbose: bool) {
    let homepage = "https://github.io/dotanuki-labs/gradle-wiper";
    let support_message = format!("For issues, reach out to {homepage}/issues");
    setup_panic!(metadata!().support(support_message.clone()).homepage(homepage));

    better_panic::install();

    let filter = if verbose { LevelFilter::Debug } else { LevelFilter::Info };

    env_logger::Builder::new()
        .filter_level(filter)
        .format_timestamp(None)
        .format_level(false)
        .format_target(false)
        .init();
}
