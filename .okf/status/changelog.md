---
type: Status
title: Changelog
description: Version history of TieXiu.
tags: [changelog, versions, history]
timestamp: 2026-07-12T00:00:00Z
---

# Changelog

All notable changes to TieXiu. Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), adhering to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## v0.2.0 — 2026-07-09

Trees refactor + CLI formatting.

### Changed

- Refactored tree API to use functional style with `&Ref` arguments.
- Updated to latest TatSu grammars (EBNF and JSON boot grammar).
- Updated dependencies.
- Refined CLI summary format and run command output formatting.

### Fixed

- Fixed `Alt`/`Option` nodes incorrectly omitted from JSON grammar exports.
- Fixed bugs in test fixtures and test assertions.

## v0.1.4 — 2026-06-14

New syntax + semantics.

### Added

- `@name`, `@int`, `@uint`, `@float`, `@bool` meta syntax for rule expressions.

### Changed

- EBNF grammar semantics now configured at the API layer rather than parser internals.

## v0.1.3 — 2026-06-06

Parallel run command in CLI.

### Added

- Parallel run command with configurable workers (`-n` flag).

### Changed

- Refined parallel CLI progress bar layout.
- Re-styled concurrent sub-progress bars.
- Configured sub-progress bars to auto-clear on completion.

### Fixed

- Fixed deadlock in CLI when executing with multiple files.
- Fixed intermittent test failure in `test_repeat`.
