// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use predicates::str::contains;

fn sut() -> Command {
    Command::cargo_bin("gradle-tidy").unwrap()
}

#[test]
fn should_parse_arguments() {
    let execution = sut().args(["disk", "shallow"]).assert();

    let expected = "Tidying disk resources";
    execution.stdout(contains(expected));
}

#[test]
fn should_show_help() {
    let description = "Easily tidy machine resources (RAM, Disk) attached to Gradle builds";

    let execution = sut().arg("help").assert();
    execution.stdout(contains(description));
}
