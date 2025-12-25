# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
