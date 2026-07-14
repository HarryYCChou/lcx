# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-07-14

### Added

- Interactive TUI (default with no subcommand): login modal with browser
  cookie auto-detection, problem search, profile stats, and a two-pane solve
  view with test/submit.
- CLI subcommands: `login`, `whoami`, `list`, `show`, `pick`, `edit`, `test`,
  `submit`, `daily`, `cache`, `config`.
- Local SQLite cache of the problem list with difficulty/tag/status/search
  filters.
- Session-cookie authentication stored at `~/.config/lcx/config.toml` (mode
  `600`).
- Solution file generation with a metadata header for automatic
  problem/language resolution on `test`/`submit`.

[Unreleased]: https://github.com/HarryYCChou/lcx/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/HarryYCChou/lcx/releases/tag/v0.1.0
