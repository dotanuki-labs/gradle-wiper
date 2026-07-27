# gradle-wiper

> [!WARNING]
>
> THIS PROJECT IS NO LONGER MAINTAINED

> Easily reclaim machine resources taken by Gradle builds

`gradle-wiper` is a clean-up utility for all users building with the
[Gradle build tool](https://gradle.org/).

```bash
gradle-wiper disk evaluate

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

```bash
gradle-wiper disk deep

Reclaimed 7.2Gib

```

Check out the
[complete documentation](https://dotanuki-labs.github.io/gradle-wiper/)
to learn more!

## License

Copyright (c) 2025 - Dotanuki Labs - [The MIT license](https://choosealicense.com/licenses/mit)
