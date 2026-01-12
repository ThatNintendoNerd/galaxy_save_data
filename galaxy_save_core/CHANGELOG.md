# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `HashCode::trunc`: Returns the least significant 16 bits of a `HashCode`.
- `HashCode::from_hex_str`: Converts a hexadecimal string into a `HashCode`.
- `HashCode::from_label`: Converts a label or hexadecimal string into a `HashCode`.
- `HashCode::to_label`: Converts a `HashCode` back to its original label, or hexadecimal if not found.
- `HashCode16`: The wrapper type for the result of the hash function, truncated to the least significant 16 bits.
- `HashCodeMap`: A container to associate hashes with their original label and vice versa.

### Changed

- Bump `serde` to 1.0.228.
- Bump `thiserror` to 2.0.17.
- Rename `OSTime` to `Time`.

## [0.1.0] - 2025-08-20

Initial release.
