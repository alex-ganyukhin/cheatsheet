# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased] (0.2.0)

### Added

### Fixed

### Changed

### Removed



## [0.1.0] - Initial release - 2025-12-07

First release of the cheatsheet CLI tool.

### Added

- CLI commands: `add`, `remove`, `list`, `search`, `show-config`
- TOML-based storage (default: `~/.config/cheatsheet/cheatsheet.toml`)
  - Configuration via `-c/--config` flag or `CHEATSHEET_CONFIG_PATH` env var
- Fuzzy search across entry fields with scoring and result limiting
- Entry model: title (unique), command, description (optional)
- Verbose logging support (-v, -vv, -vvv)

### Fixed

N/A

### Changed

N/A

### Removed

N/A
