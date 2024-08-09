# Contribution Guidelines

## Introduction

Before getting started, we strongly recommend enough reading the
[contribution guidelines for Dotanuki Labs](https://github.com/dotanuki-labs/.github/blob/main/CONTRIBUTING.md)
projects.

This document augments these recommendations with specifics ones for this project!

When looking for something easy to contribute, we can recommend checking
[good first issues](https://github.com/dotanuki-labs/gradle-wiper/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
in our issue tracker.

## Reporting bugs

This project uses that standard
[recommendations for reporting bugs](https://github.com/dotanuki-labs/.github/blob/main/CONTRIBUTING.md#issues)
in Dotanuki Labs.

In particular, all bugs are tracked as
[GitHub issues](https://github.com/dotanuki-labs/gradle-wiper/labels/bug)
with the `bug` label applied.

When reporting a bug, ensure you follow the
[provided template](https://github.com/dotanuki-labs/.github/blob/main/.github/ISSUE_TEMPLATE/bug-report.md)
paying especial attention to instructions on how to reproduce it.

> [!WARNING]
>
> Bug reports that can't be reproduced may be marked as `wontfix` and closed.

We value a lot bug reports! We'll reply to you up to 24h after a bug is reported. For bugs we
manage to reproduce, we'll release a new version of this project as soon we have a fix in place!

## Reporting vulnerabilities

> [!WARNING]
>
> **DO NOT raise GitHub issues to report a security vulnerabilities.**

This project uses the standard
[Security Policies for Dotanuki Labs](https://github.com/dotanuki-labs/.github/blob/main/SECURITY.md).
Please check them out.

## Fixing bugs

When fixing an existing bug:

- Describe why your fix works as intended
- If applicable, ensure you've added a test case covering the fixed bug

## Adding new functionality

When adding a new feature:

- Describe briefly your solution
- Ensure you've added tests covering use cases for the functionality
- Consider using inlay Pull Request comments to highlight specific implementation details

## Pull Requests

> [!Note]
>
> For non-Dotanuki members, we'll only accept contributions from forked repositories

### General procedure

- Ensure you've read our [development guidelines](https://github.com/dotanuki-labs/gradle-wiper/blob/main/docs/development.md)
- Fork this project
- Ensure you have a proper running environment for your fork, e.g. running unit tests locally

```bash
./krabby.sh tests
```

- Code your changes 🔥
- Use our `./krabby.sh` script to verify [what we execute on CI](https://github.com/dotanuki-labs/gradle-wiper/blob/main/.github/workflows/ci.yml)
- Raise your Pull Request 🚀
- Fill the description with our [pull request template](https://github.com/dotanuki-labs/.github/blob/main/.github/PULL_REQUEST_TEMPLATE.md)
- Ensure you've your PR [linked with a proper issue](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue#linking-a-pull-request-to-an-issue-using-a-keyword)

## License

Please note that all code contributed by you will follow the
[MIT license](http://opensource.org/licenses/MIT)
without any additional terms.
