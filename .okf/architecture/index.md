---
type: Architecture
title: Architecture
description: Internal design of TieXiu PEG parser engine.
tags: [architecture, design]
timestamp: 2026-07-12T00:00:00Z
---

# Architecture

Internal design decisions and implementation details for the TieXiu PEG parser engine.

## Concepts

- [Performance](performance.md) — Hardware-bound parsing, Amdahl's Law, Mechanical Sympathy
- [Design](design.md) — Design criteria and implementation choices
- [Cursor](cursor.md) — 24-byte parsing cursor with copy-on-write semantics
- [Tree](tree.md) — 32-byte compact CST/AST nodes with TreeMap builder
- [Context](context.md) — Mutable parsing context with memoization
- [Grammar](grammar.md) — PEG grammar representation, rules, expressions, analysis
- [PyO3](pyo3.md) — PyO3 bindings, pymodule, pyfnapi, pyooapi
- [PyO3 Boundary](pyo3-boundary.md) — Rust-to-Python wrapper pattern
- [API Rust](api-rust.md) — Functional and OO Rust API
- [API Python](api-python.md) — Python API with JSON return types
- [CLI](cli.md) — clap CLI with Boot/Run/Grammar subcommands
- [Project Layout](project-layout.md) — Directory structure and toolchain
