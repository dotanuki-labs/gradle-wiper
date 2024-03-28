// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod core;

use clap::Parser;
use human_panic::setup_panic;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProgramArguments {
    #[arg(short, long)]
    name: String,
}

fn main() {
    setup_panic!();

    let arguments = ProgramArguments::parse();
    let greet = core::greet(&arguments.name).unwrap();
    println!("{}", greet);
}
