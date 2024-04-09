// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use predicates::str::contains;

fn sut() -> Command {
    Command::cargo_bin("gradle-wiper").expect("Should be able to instantiate a command")
}

#[test]
fn should_parse_arguments() {
    let execution = sut().args(["disk", "unsupported"]).assert();

    let expected = "possible values: evaluate, shallow, deep";
    execution.stderr(contains(expected));
}

#[test]
fn should_show_help() {
    let description = "Reclaim machine resources (RAM, Disk) attached to Gradle builds";

    let execution = sut().arg("help").assert();
    execution.stdout(contains(description));
}

#[test]
fn should_evaluate_no_usages_of_ram_memory() {
    let execution = sut().args(["ram", "evaluate"]).assert();

    let expected = "No usages of RAM memory";
    execution.success().stdout(contains(expected));
}
