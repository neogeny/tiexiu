---
type: Project
title: TieXiu (铁修)
description: A high-performance port of TatSu PEG parser generator to Rust.
okf_version: "0.1"
tags: [rust, peg, parser, tatsu, pyo3]
timestamp: 2026-07-12T00:00:00Z
---

# TieXiu (铁修)

A PEG (Parsing Expression Grammar) engine that implements the flexibility and power of the TatSu lineage into a memory-safe, high-concurrency architecture optimized for modern CPU caches.

TieXiu takes grammars in extended EBNF as input and outputs memoizing (Packrat) PEG parsers as a Rust model. It is available as a Rust library and a Python library via PyO3/Maturin.

## Bundle Contents

- [Agent Rules](agents.md) — Operational rules for agents
- [Architecture](architecture/) — Internal design: cursor, tree, context, performance, design, PyO3 boundary
- [Grammar](grammar/) — Grammar syntax and compatibility
- [Status](status/) — Roadmap and changelog

## Key Facts

- **Version:** 0.2.1-beta.1
- **License:** MIT OR Apache-2.0
- **Repository:** https://github.com/neogeny/tiexiu
- **Performance:** Hardware-bound — 1.08x TatSu (Amdahl's Law limit)
- **Test suite:** 211 tests, all passing
