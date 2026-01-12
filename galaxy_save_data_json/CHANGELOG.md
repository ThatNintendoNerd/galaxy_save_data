# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `labels` option to load newline-separated hash labels from a file.
- `strict` option to reject hash labels not found in the labels file.

### Changed

- Bump `clap` to 4.5.54.
- Bump `serde_json` to 1.0.149.
- Improve error messages.

### Fixed

- Incorrect endianness for save files from NVIDIA Shield TV.

## [0.1.0] - 2025-08-20

Initial release.
