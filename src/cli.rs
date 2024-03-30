// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum TidyMode {
    Analyse,
    Shallow,
    Deep,
}

#[derive(Args)]
pub struct TidyArguments {
    #[arg(value_enum)]
    pub mode: TidyMode,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Disk(TidyArguments),
    Ram(TidyArguments),
}
