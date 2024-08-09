# Development Guidelines

## Requirements

> [!INFO]
> We officially support development over `macOS` and `Linux` boxes for now.

This project is written in Rust, and explicitly depends on:

- [rustup](https://rustup.rs/)
- [docker](https://www.docker.com/)

Please ensure you have those installed on your system.

## Project setup

This project uses the companion `krabby.sh` as a task runner and
defines a few recipes to make things straightforward. You can check them by running:

```bash
./krabby.sh

Available recipes:
    setup     # Performs setup for this project
    lint      # Checks code formatting and smells
    .
    .
    .
```

We definitely recommend getting started by setting up the latest version of Rust along with
all required Cargo subcommands by running:

```bash
./krabby.sh setup
```

## Code Style

This project adotps a few customizations on top of the standard
[rustfmt](https://rust-lang.github.io/rustfmt)
conventions. In addition, it also provides a
[.editorconfig](https://editorconfig.org/)
file to make it straightforward to get code formatting right on you editor or IDE.

In addition to that, this project uses
[Clippy](https://rust-lang.github.io/rust-clippy)
to catch the most straightforward code smells, some additional warnings on
specific patterns, and denying any warnings emitted.

## Commit Conventions

This project does not adopt/enforce any specific commit conventions, e.g.
[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)
for now.

## Code Conventions

This project encourages
[easy-mode Rust](https://llogiq.github.io/2024/03/28/easy.html)
by default, focusing on simplicity and code readability.

In addition, this project leverages
[anyhow](https://docs.rs/anyhow/latest/anyhow/)
for better error signaling, error transformations and error propagation.

## Continuous Integration

According to our policies, all code contributions to this project must go through a Pull Request,
and all required status checks must pass.

This project adopts
[GiHub Actions](https://github.com/dotanuki-labs/gradle-wiper/actions)
as it CI system. Most of the verifications we'll run on CI are provided by the `just` recipes,
as previously mentioned.

In addition to that, we also run a specific `Job` to enforce code quality standards for docs,
Bash scripts and others. In particular, this project enforces the proper open-source license
tracking on all Rust and Bash files.

Last, but not least, this project runs additional Quality checks for Rust with
[deepsource.io](https://app.deepsource.com/gh/dotanuki-labs/gradle-wiper/)
