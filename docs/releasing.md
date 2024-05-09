# Releasing new versions

## Preparing the release

- Understand what changes we are shipping by inspecting unreleased commits

```bash
git co main
git pull origin main --rebase
git log $(git describe --abbrev=0 --tags)...HEAD --oneline
```

- Determine next release version according to [SemVer](https://semver.org/)
- Create a short-lived branch

```bash
git checkout -b ufs/release-x.y.z
```

- Add **notable changes** the to [changelog file](https://github.com/dotanuki-labs/gradle-wiper/blob/main/docs/changelog.md)
- Bump version at [Cargo.toml](https://github.com/dotanuki-labs/gradle-wiper/blob/main/Cargo.toml#L3)
- Raise a PR preparing the release

## Creating a release (GitHub admins-only)

- Ensure the next release is prepared (as described above)
- Execute the [CD Workflow](https://github.com/dotanuki-labs/gradle-wiper/actions/workflows/cd.yml)
- Go to the [releases page](https://github.com/dotanuki-labs/gradle-wiper/releases)
- Review the release draft and add any final touches (for instance, updating `RenovateBot` identity name)
- Publish the release 🚀

## Updating distributions

- Clone [dotanuki-labs/homebrew-taps](https://github.com/dotanuki-labs/homebrew-taps)
- Create a branch like `ufs/gradle-wiper-x.y.z`
- Update the `gradle-wiper.rb` formula with proper version and checksums
- Raise a PR
