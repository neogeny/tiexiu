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

- [Performance](/architecture/performance.md) — Hardware-bound parsing, Amdahl's Law, Mechanical Sympathy
- [Design](/architecture/design.md) — Design criteria and implementation choices
- [Cursor](/architecture/cursor.md) — 24-byte parsing cursor with copy-on-write semantics
- [Tree](/architecture/tree.md) — 32-byte compact CST/AST nodes with TreeMap builder
- [Context](/architecture/context.md) — Mutable parsing context with memoization
- [Grammar](/architecture/grammar.md) — PEG grammar representation, rules, expressions, analysis
- [PyO3](/architecture/pyo3.md) — PyO3 bindings, pymodule, pyfnapi, pyooapi
- [PyO3 Boundary](/architecture/pyo3-boundary.md) — Rust-to-Python wrapper pattern
- [API Rust](/architecture/api-rust.md) — Functional and OO Rust API
- [API Python](/architecture/api-python.md) — Python API with JSON return types
- [CLI](/architecture/cli.md) — clap CLI with Boot/Run/Grammar subcommands
- [Project Layout](/architecture/project-layout.md) — Directory structure and toolchain
