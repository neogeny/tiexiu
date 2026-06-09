# Changelog

All notable changes to this project are documented in this file. The format is  based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this  project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


[Unreleased]: https://github.com/neogeny/ogopego/compare/v0.1.3...HEAD
[v0.1.3]: https://github.com/neogeny/ogopego/compare/v0.1.2...v0.1.3

## [Unreleased]

### Added

* `@name`, `@int`, `@uint`, `@float`, `@bool` meta syntax for rule expressions.
  These yield `NameMeta`, `IntMeta`, `UIntMeta`, `FloatMeta`, and `BoolMeta` AST
  nodes respectively.

### Changed

* EBNF grammar semantics are now configured at the API layer rather than deep in
  the parser internals, giving library users a clean hook to supply their own
  semantics for grammar parsing.

## [v0.1.3] 2026-06-06 Parallel run command in the CLI

### Added

* Parallel run command in the CLI. Now supports running multiple files in  parallel. The number of parallel workers can be configured with the   `--workers` (`-n`) flag.

### Changed

* Refined parallel CLI progress bar layout to pin the global file count progress bar permanently to the top of the terminal screen, preventing layout jitter.
* Re-styled concurrent sub-progress bars for individual file parses to use bracket-less continuous green horizontal lines (`━╸─`) for a modern and sleek visual style.
* Configured sub-progress bars to automatically clear themselves from the terminal upon completion or drop, preventing terminal output clutter.

### Fixed

* Fixed a deadlock in the CLI when executing with multiple files. The stall was caused by a global lock on `stderr` in `src/main.rs` that conflicted with `indicatif`'s background rendering thread.
* Fixed an intermittent unit test failure in `test_repeat` by isolating process-global environment variable modifications in `test_cfg_load_from_env`.
