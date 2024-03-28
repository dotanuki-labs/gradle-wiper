// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use predicates::str::contains;

fn sut() -> Command {
    Command::cargo_bin("gradle-tidy").unwrap()
}

#[test]
fn should_parse_arguments() {
    let execution = sut().args(["--name", "John"]).assert();

    let expected = "Hello, John!\n";
    execution.stdout(expected);
}

#[test]
fn should_show_help() {
    let description = "An opinionated way to kick-off CLI apps powered by Rust";

    let execution = sut().arg("--help").assert();
    execution.stdout(contains(description));
}

#[test]
fn should_fail_without_arguments() {
    let instruction = "required arguments were not provided";

    let execution = sut().assert();
    execution.failure().stderr(contains(instruction));
}
