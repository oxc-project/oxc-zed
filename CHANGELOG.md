# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.4](https://github.com/oxc-project/oxc-zed/compare/v0.4.3...v0.4.4) - 2026-01-27

### Other

- *(deps)* update crate-ci/typos action to v1.42.3 ([#100](https://github.com/oxc-project/oxc-zed/pull/100))
- *(deps)* update oxc apps ([#99](https://github.com/oxc-project/oxc-zed/pull/99))
- *(deps)* update crate-ci/typos action to v1.42.2 ([#98](https://github.com/oxc-project/oxc-zed/pull/98))
- *(deps)* update github-actions ([#94](https://github.com/oxc-project/oxc-zed/pull/94))
- *(deps)* update pnpm to v10.28.1 ([#95](https://github.com/oxc-project/oxc-zed/pull/95))
- *(deps)* update dependency rust to v1.93.0 ([#92](https://github.com/oxc-project/oxc-zed/pull/92))
- *(deps)* update oxc apps ([#91](https://github.com/oxc-project/oxc-zed/pull/91))
- *(deps)* update crate-ci/typos action to v1.42.1 ([#90](https://github.com/oxc-project/oxc-zed/pull/90))
- *(deps)* update dependency oxfmt to v0.25.0 ([#89](https://github.com/oxc-project/oxc-zed/pull/89))
- *(deps)* update pnpm to v10.28.0 ([#88](https://github.com/oxc-project/oxc-zed/pull/88))
- update logo ([#86](https://github.com/oxc-project/oxc-zed/pull/86))
- Document oxlint v1.35.0+ requirement ([#85](https://github.com/oxc-project/oxc-zed/pull/85))
- update README
- *(deps)* update oxc apps ([#83](https://github.com/oxc-project/oxc-zed/pull/83))
- add renovate.json
- add example for both
- update oxfmt example
- *(deps)* update taiki-e/checkout-action action to v1.3.2 ([#77](https://github.com/oxc-project/oxc-zed/pull/77))
- Add steps on how to update the Git submodule in the Zed extensions repo ([#75](https://github.com/oxc-project/oxc-zed/pull/75))

## [0.4.3](https://github.com/oxc-project/oxc-zed/compare/v0.4.2...v0.4.3) - 2026-01-10

### Added

- add languages that oxfmt supports ([#73](https://github.com/oxc-project/oxc-zed/pull/73))

### Other

- Update the readme with supported languages ([#74](https://github.com/oxc-project/oxc-zed/pull/74))
- *(deps)* update crate-ci/typos action to v1.42.0 ([#72](https://github.com/oxc-project/oxc-zed/pull/72))
- *(deps)* update dependency oxlint to v1.38.0 ([#71](https://github.com/oxc-project/oxc-zed/pull/71))
- *(deps)* update dependency oxfmt to v0.23.0 ([#70](https://github.com/oxc-project/oxc-zed/pull/70))
- Remove configuration examples from the README and link to the examples folder ([#68](https://github.com/oxc-project/oxc-zed/pull/68))
- *(deps)* update dependency oxlint to v1.37.0 ([#65](https://github.com/oxc-project/oxc-zed/pull/65))
- *(deps)* update dependency oxfmt to v0.22.0 ([#64](https://github.com/oxc-project/oxc-zed/pull/64))
- Clean up oxfmt settings configuration ([#66](https://github.com/oxc-project/oxc-zed/pull/66))
- *(deps)* update crate-ci/typos action to v1.41.0 ([#61](https://github.com/oxc-project/oxc-zed/pull/61))

## [0.4.2](https://github.com/oxc-project/oxc-zed/compare/v0.4.1...v0.4.2) - 2026-01-03

### Added

- Additional debug logging ([#63](https://github.com/oxc-project/oxc-zed/pull/63))

### Other

- Add example projects for oxfmt and oxlint ([#62](https://github.com/oxc-project/oxc-zed/pull/62))
- update repository in extension.toml
- *(deps)* update crate-ci/typos action to v1.40.1 ([#59](https://github.com/oxc-project/oxc-zed/pull/59))
- update description in extension.toml
- change authors to Boshen so it looks legit in the extension store

## [0.4.1](https://github.com/oxc-project/oxc-zed/compare/v0.4.0...v0.4.1) - 2025-12-25

### Added

- Add justfile to make it easier for contributors to run pre PR checks ([#49](https://github.com/oxc-project/oxc-zed/pull/49))
- Allow supplying binary.env configuration without binary.arguments or binary.path ([#48](https://github.com/oxc-project/oxc-zed/pull/48))

### Fixed

- Update exe lookup to avoid node_modules/.bin due to PNPM using bash scripts ([#55](https://github.com/oxc-project/oxc-zed/pull/55))
- Add wasm extension target ([#54](https://github.com/oxc-project/oxc-zed/pull/54))
- LSP from workspace ([#53](https://github.com/oxc-project/oxc-zed/pull/53))
- Don't panic when both binary.arguments and binary.path are missing ([#50](https://github.com/oxc-project/oxc-zed/pull/50))

### Other

- auto update `extensions.yml`
- Update maintenance instructions for version file
- *(justfile)* use `just ready` across oxc
- add MAINTENANCE.md for release instructions
- Allow relative path for OXLINT_TSGOLINT_PATH env variable ([#47](https://github.com/oxc-project/oxc-zed/pull/47))
- Remove HashMap import to fix clippy disallowed_types warnings
