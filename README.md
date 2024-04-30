# gradle-wiper

[![rustfmt](https://img.shields.io/badge/code%20style-%E2%9D%A4-FF4081.svg)](https://rust-lang.github.io/rustfmt)
[![DeepSource](https://app.deepsource.com/gh/dotanuki-labs/gradle-wiper.svg/?label=active+issues&token=_GtR-4HW2ttd966CiJOOYEw6)](https://app.deepsource.com/gh/dotanuki-labs/gradle-wiper/)
[![CI](https://github.com/dotanuki-labs/gradle-wiper/actions/workflows/ci.yml/badge.svg)](https://github.com/dotanuki-labs/gradle-wiper/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/gradle-wiper)](https://crates.io/crates//gradle-wiper)
[![License](https://img.shields.io/github/license/dotanuki-labs/gradle-wiper)](https://choosealicense.com/licenses/mit)

> Easily reclaim machine resources (RAM, Disk) taken by Gradle builds

`gradle-wiper` is a clean-up utility for all users building with the
[Gradle build tool](https://gradle.org/).

It's a fast, modern and ergonomic alternative for existing tools like
[deep-clean](https://github.com/rock3r/deep-clean),
[AndroidDaemonKiller](https://github.com/PaulWoitaschek/DaemonHunter)
and others.

Like these tools, `gradle-wiper` might be useful in
situations where build executions consistently fail even after
trying all sort of tricks or when you don't want to pay to
price of restarting your work machine to be in a clean state of your
system.

## Installing

> [!NOTE]
> This tool is compatible with `macOS` and `Linux` boxes, running over `x86_64` or `aarch64` hardware

Installing from [crates.io](https://crates.io/crates/gradle-wiper) (requires [Rust](https://rustup.rs/))

```bash
cargo install gradle-wiper
```

Installing with [homebrew](https://brew.sh/) (macOS/Linux)

```bash
brew tap dotanuki-labs/taps
brew install gradle-wiper
```

More installation methods to come! Stay tuned! 🔥

## Using

The general way to use this tool is

```bash
gradle-wiper <resource> <action> (-v | --verbose)
```

where:

- resource: `disk` or `ram`
- action: `evaluate` (dry-run), `shallow` (wipe) or `deep` (wipe)

For instance, to evaluate used disk space related to previous Gradle builds:

```bash
gradle-wiper disk evaluate
```

You should see something like:

```text
╭───────────────────────────────┬────────────╮
│ What                          ┆ Total Size │
╞═══════════════════════════════╪════════════╡
│ Gradle Build Caches           ┆ 4.41GiB    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Gradle Daemon Logs            ┆ 343.67MiB  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Gradle JDK toolchains         ┆ 307.02MiB  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Gradle Distributions          ┆ 556.21MiB  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Gradle Temporary Files        ┆ 124.47MiB  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Gradle platform-native caches ┆ 2.09MiB    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Gradle build-scans data       ┆ 3.53MiB    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Maven local repository        ┆ 536.22MiB  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Konan/KMP Caches             ┆ 0B         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Build output files             ┆ 1.01GiB    │
╰───────────────────────────────┴────────────╯

Total resources (disk space) : 7.2GiB
```

To wipe out all build-related Daemons (Gradle Workers, Kotin compiler, etc.)
from you RAM memory:

```bash
gradle-wiper ram shallow
```

To wipe out all JVM processes from your RAM memory (including running IDEs):

```bash
gradle-wiper ram deep
```

To wipe out potentially corrupted build caches from disk, including:

- `$HOME/.gradle/caches`
- `$HOME/.gradle/configuration-cache`

along with

- Maven dependency caches (`$HOME/.m2`)
- Konan dependency caches (`$HOME/.konan`)
- Logs from Gradle builds (`$HOME/.gradle/daemon`)
- Gradle temporary files (`$HOME/.gradle/.tmp`)
- All `build` output folders from any Gradle projects in your system

```bash
gradle-wiper disk shallow
```

> [!NOTE]
> This tool does not uninstall any existing software from your system, and
> it also preserves custom configuration hosted at `$HOME/.gradle`, like
> `$HOME/.gradle/gradle.properties` file and `$HOME/.gradle/init.d` build scripts

To also scan your disk for Gradle/IDE metadata files per project, removing

- all `<my-project>/.gradle/*` Gradle files/caches
- all `<my-project>/.idea/*` IDE metadata files/caches

```bash
gradle-wiper disk deep
```

## License

Copyright (c) 2024 - Dotanuki Labs - [The MIT license](https://choosealicense.com/licenses/mit)
