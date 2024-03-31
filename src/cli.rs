// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::models::{ExecutionOutcome, MachineResource, ResourceAllocation, TidyAction};
use clap::{Args, Parser, Subcommand, ValueEnum};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use human_panic::setup_panic;

#[derive(ValueEnum, Debug, Clone)]
enum ExecutionMode {
    Analyse,
    Shallow,
    Deep,
}

#[derive(Args)]
struct WrappedArguments {
    #[arg(value_enum)]
    pub mode: ExecutionMode,
}

impl From<WrappedArguments> for TidyAction {
    fn from(value: WrappedArguments) -> Self {
        match value.mode {
            ExecutionMode::Analyse => TidyAction::EvaluateResources,
            ExecutionMode::Shallow => TidyAction::ShallowWiping,
            ExecutionMode::Deep => TidyAction::DeepWiping,
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

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        setup_panic!();
        Self {}
    }

    pub fn parsed_arguments(&self) -> (MachineResource, TidyAction) {
        let cli = CliParser::parse();
        match cli.command {
            Commands::Disk(args) => (MachineResource::DiskSpace, TidyAction::from(args)),
            Commands::Ram(args) => (MachineResource::RamMemory, TidyAction::from(args)),
        }
    }

    pub fn show_allocated_resources(&self, allocated: &[ResourceAllocation]) {
        println!();
        let rows = allocated
            .iter()
            .map(|res| vec![format!("{:?}", res.use_case), format!("{:?}", res.megabytes)])
            .collect::<Vec<_>>();

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(100)
            .set_header(vec!["Allocation", "Amount (MB)"])
            .add_rows(rows);

        println!("{table}");
        println!();
    }

    pub fn show_execution_outcome(&self, outcome: &ExecutionOutcome) {
        let resource_name = match outcome.subject {
            MachineResource::RamMemory => "system processes",
            MachineResource::DiskSpace => "files",
        };

        println!();
        println!(
            "Reclaimed {} Mb by deleting {} {}",
            outcome.reclaimed_memory, outcome.freed_entries, resource_name
        );
        println!();
    }
}
