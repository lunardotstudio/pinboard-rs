# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0](https://github.com/lunardotstudio/pinboard-rs/compare/v1.0.0...v2.0.0) - 2024-10-30

### Added

- *(example)* Add examples: results and suggest
- *(example)* Add example that gets recent posts
- *(api)* Async client added
- *(types)* Add Serialization support to all v1 types
- *(api)* Add Limit trait to all v1 endpoints

### Fixed

- Update code to remove warnings; format settings
- *(types)* Update the PostsList struct

### Other

- Update Cargo; add release action; add cliff.toml
- Add a github workflow for testing
- Update documentation and README.md
- Ignore emacs temporary files
- Add in `cargo-husky`
- Cargo formatted version
