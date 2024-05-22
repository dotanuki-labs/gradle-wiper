# Gradle Wiper

`gradle-wiper` is a clean-up utility for all users building with the
[Gradle build tool](https://gradle.org/).

It's a fast, modern, intuitive and ergonomic alternative for existing tools like
[deep-clean](https://github.com/rock3r/deep-clean),
[AndroidDaemonKiller](https://github.com/PaulWoitaschek/DaemonHunter)
and others.

Like these tools, `gradle-wiper` might be useful in
situations like:

- you are running out of disk space on your local machine
- your builds consistently fail even after all sort of Gradle tricks
- you don't want to restart your machine just to have some RAM memory again
- etc

This tool is written in 100% safe
[Rust](https://www.rust-lang.org/)
hence it is small, fast and portable by default. In addition, it adheres to the
[best open-source practices](https://www.bestpractices.dev/en/projects/8920)
proposed by the [Open Source Security Foundation](https://openssf.org/).

Last, but not least, this code is distributed under the terms of [the MIT license](https://choosealicense.com/licenses/mit)
