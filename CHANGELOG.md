# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-07-15

### Added

- Interactive TUI (default with no subcommand): login modal with browser
  cookie auto-detection, problem search, profile stats, and a two-pane solve
  view with test/submit.
- CLI subcommands: `login`, `whoami`, `list`, `show`, `pick`, `edit`, `test`,
  `submit`, `daily`, `cache`, `config`.
- Daily challenge pinned to the top of the TUI results when the search bar is
  empty.
- Background loading of profile stats and the daily challenge on TUI startup so
  the cached problem list paints instantly.
- Page-by-page progress while refreshing the problem list (`F5`) in the TUI.
- Local SQLite cache of the problem list with difficulty/tag/status/search
  filters.
- Session-cookie authentication stored at `~/.config/lcx/config.toml` (mode
  `600`).
- Solution file generation with an lcx banner header; the problem and language
  are resolved from the file name (`{id}.{slug}.{ext}`) on `test`/`submit`.

### Fixed

- Cookie auto-detection now reports why it failed instead of silently spinning,
  and on Windows explains that Chrome/Edge/Brave app-bound encryption requires
  running as administrator (Firefox and manual login still work).

[Unreleased]: https://github.com/HarryYCChou/lcx/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/HarryYCChou/lcx/releases/tag/v0.1.0
