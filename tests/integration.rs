// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use predicates::str::contains;

fn sut() -> Command {
    Command::cargo_bin("gradle-wiper").unwrap()
}

#[test]
fn should_parse_arguments() {
    let execution = sut().args(["disk", "unsupported"]).assert();

    let expected = "possible values: analyse, shallow, deep";
    execution.stderr(contains(expected));
}

#[test]
fn should_show_help() {
    let description = "Reclaim machine resources (RAM, Disk) attached to Gradle builds";

    let execution = sut().arg("help").assert();
    execution.stdout(contains(description));
}
