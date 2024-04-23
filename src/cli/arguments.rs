// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::models::{MachineResource, WipeAction};
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
enum ExecutionMode {
    Evaluate,
    Shallow,
    Deep,
}

#[derive(Args)]
struct WrappedArguments {
    #[arg(value_enum)]
    pub mode: ExecutionMode,

    #[arg(short, long)]
    pub verbose: bool,
}

impl From<&WrappedArguments> for WipeAction {
    fn from(value: &WrappedArguments) -> Self {
        match value.mode {
            ExecutionMode::Evaluate => WipeAction::Evaluate,
            ExecutionMode::Shallow => WipeAction::ShallowWipe,
            ExecutionMode::Deep => WipeAction::DeepWipe,
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CliParser {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Disk(WrappedArguments),
    Ram(WrappedArguments),
}

pub fn parsed_arguments() -> (MachineResource, WipeAction, bool) {
    let cli = CliParser::parse();
    match cli.command {
        Commands::Disk(args) => (MachineResource::DiskSpace, WipeAction::from(&args), args.verbose),
        Commands::Ram(args) => (MachineResource::RamMemory, WipeAction::from(&args), args.verbose),
    }
}
