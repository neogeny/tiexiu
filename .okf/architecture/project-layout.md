---
type: Architecture
title: Project Layout
description: Directory structure and toolchain for TieXiu.
tags: [layout, structure, toolchain]
timestamp: 2026-07-12T00:00:00Z
---

# Project Layout

## Source Structure

- `src/api/` — Functional and OO APIs (grammar compilation, parsing, caching)
- `src/cfg/` — Configuration key definitions and environment loading
- `src/codegen/` — Code generation for grammar models
- `src/context/` — Mutable parsing context, memoization, tracing
- `src/input/` — Input stream handling and the 24-byte Cursor logic
- `src/json/` — JSON serialization and import/export
- `src/peg/` — PEG grammar representation, analysis, parsing, boot grammar
- `src/python/` — PyO3 bindings (_tiexiu module, functional and OO APIs)
- `src/tools/` — Utility tools (railroad diagrams, formatting)
- `src/trees/` — 32-byte compact CST/AST nodes and TreeMap builder
- `src/ui/` — CLI (clap), progress reporting, syntax highlighting
- `src/util/` — Shared utilities (indentation, regex, string tools, token stack)
- `src/error.rs` — Error types
- `src/lib.rs` — Library root
- `src/main.rs` — CLI entry point
- `tests/` — Integration tests (40+ test files)

## Toolchain

- **Rust:** Latest stable toolchain
- **Python:** >=3.12
- **Build System:** `just` and `maturin` (`just build`, `just test`)

## TatSu Compatibility

Core PEG expressions are supported. Native type directives (`@name`, `@int`, `@uint`, `@float`, `@bool`) replace dynamic Python semantic actions with static annotations.

## PyO3 Boundary

- Tree data stays in Rust until explicit export to Python dicts/lists or JSON
- Custom `ParseError` exception (inheriting `PyException`) for parse failures
- `TreeRef = Arc<Tree>` and `Cursor` trait minimize allocations at the boundary
