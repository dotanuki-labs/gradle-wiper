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

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    static TOOL: &str = "gradle-tidy";

    #[test]
    fn should_parse_arguments() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();
        let assert = cmd.args(["--name", "John"]).assert();

        let expected = "Hello, John!\n";
        assert.stdout(expected);
    }

    #[test]
    fn should_show_help() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();
        let description = "An opinionated way to kick-off CLI apps powered by Rust";

        let assert = cmd.arg("--help").assert();
        assert.stdout(contains(description));
    }

    #[test]
    fn should_fail_without_arguments() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();
        let instruction = "required arguments were not provided";

        let assert = cmd.assert();
        assert.failure().stderr(contains(instruction));
    }
}
