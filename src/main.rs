// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use human_panic::setup_panic;

fn main() {
    setup_panic!();

    let cli = Cli::parse();

    match cli.command {
        Commands::Disk(args) => println!("Tidying disk resources -> {:?}", args.mode),
        Commands::Ram(args) => println!("Tidying memory resources -> {:?}", args.mode),
    }
}
