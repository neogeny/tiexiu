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
- [Cursor](/architecture/cursor.md) — 24-byte parsing cursor with copy-on-write semantics
- [Tree](/architecture/tree.md) — 32-byte compact CST/AST nodes with TreeMap builder
- [Context](/architecture/context.md) — Mutable parsing context with memoization
- [PyO3 Boundary](/architecture/pyo3-boundary.md) — Rust-to-Python wrapper pattern
