# Using this tool

## Overview

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
│ Konan/KMP Caches              ┆ 0B         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Build output files            ┆ 1.01GiB    │
╰───────────────────────────────┴────────────╯

Total resources (disk space) : 7.2GiB
```

## Reclaiming resources

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

## Troubleshooting

All subcommands accept a `verbose` flag which outputs information about the current
execution:

```bash
gradle-wiper disk evaluate --verbose
```

In addition, you also have the `--help` flag:

```bash
gradle-wiper --help

Reclaim machine resources (RAM, Disk) attached to Gradle builds

Usage: gradle-wiper <COMMAND>

Commands:
  disk
  ram
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
