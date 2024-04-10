// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::models::{EvaluationOutcome, ExecutionOutcome, MachineResource, WipeAction, WippingOutcome};
use clap::{Args, Parser, Subcommand, ValueEnum};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use human_panic::setup_panic;

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
}

impl From<WrappedArguments> for WipeAction {
    fn from(value: WrappedArguments) -> Self {
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

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        setup_panic!();
        Self {}
    }

    pub fn parsed_arguments(&self) -> (MachineResource, WipeAction) {
        let cli = CliParser::parse();
        match cli.command {
            Commands::Disk(args) => (MachineResource::DiskSpace, WipeAction::from(args)),
            Commands::Ram(args) => (MachineResource::RamMemory, WipeAction::from(args)),
        }
    }

    fn report_resources(&self, resource: &MachineResource, outcome: &EvaluationOutcome) {
        println!();

        let allocated = &outcome.resources;

        if allocated.is_empty() {
            let what = match resource {
                MachineResource::RamMemory => "RAM memory",
                MachineResource::DiskSpace => "disk space",
            };

            println!("No usages of {} related to Gradle builds were found", what);
            println!();
            return;
        }

        let rows = allocated
            .iter()
            .map(|res| vec![format!("{:?}", res.use_case), format!("{}", res.amount)])
            .collect::<Vec<_>>();

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(100)
            .set_header(vec!["Allocation", "Amount"])
            .add_rows(rows);

        table.add_row(vec!["Total", &outcome.total_size.to_string()]);

        println!("{table}");
        println!();
    }

    fn report_cleanup(&self, outcome: &WippingOutcome) {
        let resource_name = match outcome.subject {
            MachineResource::RamMemory => "system processes",
            MachineResource::DiskSpace => "files",
        };

        println!();
        println!(
            "Reclaimed {} by deleting {} {}",
            outcome.reclaimed_memory, outcome.freed_entries, resource_name
        );
        println!();
    }

    pub fn show_execution_outcome(&self, resource: &MachineResource, outcome: &ExecutionOutcome) -> anyhow::Result<()> {
        match outcome {
            ExecutionOutcome::Evaluation(evaluation) => self.report_resources(resource, evaluation),
            ExecutionOutcome::Wipping(wipping) => self.report_cleanup(wipping),
        }

        Ok(())
    }
}
