// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::core::models::{EvaluationOutcome, ExecutionOutcome, MachineResource, WipingOutcome};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use log::info;

pub fn show_execution_outcome(resource: &MachineResource, outcome: &ExecutionOutcome) -> anyhow::Result<()> {
    match outcome {
        ExecutionOutcome::Evaluation(evaluation) => used_resources(resource, evaluation),
        ExecutionOutcome::Wiping(wipping) => cleanup_outcome(resource, wipping),
    }

    Ok(())
}

fn used_resources(resource: &MachineResource, outcome: &EvaluationOutcome) {
    info!("");

    let allocated = &outcome.resources;

    if allocated.is_empty() {
        info!("No usages of {resource} related to Gradle builds were found");
        info!("");
        return;
    }

    let rows = allocated
        .iter()
        .map(|res| vec![format!("{}", res.use_case), format!("{}", res.amount)])
        .collect::<Vec<_>>();

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(100)
        .set_header(vec!["What", "Total Size"])
        .add_rows(rows);

    println!("{table}");

    info!("");
    info!("Total resources ({}) : {:.1}", resource, &outcome.total_size);
    info!("");
}

fn cleanup_outcome(resource: &MachineResource, outcome: &WipingOutcome) {
    info!("");
    info!("Reclaimed {} : {:.1}", resource, outcome.reclaimed);
    info!("");
}
