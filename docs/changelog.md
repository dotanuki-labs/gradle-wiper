# Changelog

We follow the [Keep a Changelog](https://keepachangelog.com)
conventions for release notes. Notable product changes are documented here.
In addition, you can also check all commits that landed in a given release
directly on
[Github Releases](https://github.com/dotanuki-labs/gradle-wiper/releases/latest).

## 0.2.0

Released at **2024-08-10**

### Added

- Binary artifacts and SBOMs now have provenance powered by Github Actions
- Project is now compliant with OpenSSF Best practices

## 0.1.0

Released at **2024-04-29**

### Added

- Full implementation of evaluation of used resources, for `ram` and `disk`
- Full implementation of wiping with `shallow` and `deep` modes
- Support to detection of Gradle projects by scanning special paths in the filesystem
- Support to cleaning Konan/KMP caches
- Support to cleaning IDE-related resources from RAM memory
- Compatibility with `Linux` and `macOS`, `x86_64` and `aarch64` platforms
