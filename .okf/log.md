---
type: Log
title: Project History
description: Chronological history of TieXiu development milestones.
timestamp: 2026-07-12T00:00:00Z
---

# Directory Update Log

## 2026-07-12

* **Creation**: Established `.okf/` knowledge bundle with root index and plan.
* **Update**: Rewrote README for current project state — performance reframed as Mechanical Sympathy.
* **Update**: Marked all 11 ROADMAP clusters as completed/dropped.
* **Transform**: Moved RULES.md, AGENTS.md, DESIGN.md, SYNTAX.md, ROADMAP.md, CHANGELOG.md, PYO3_BOUNDARY.md into `.okf/` bundle.
* **Concepts**: Wrote 8 concept files (cursor, tree, context, pyo3, api-rust, api-python, cli, grammar).
* **Restore**: Restored CHANGELOG.md for human consumption.
* **Restore**: Restored SYNTAX.md and README.md link for human consumption.

## 2026-07-10

* **Update**: Cluster 10 — benchmark improvements (e2e, scaling, TatSu grammar benchmarks).
* **Update**: Cluster 9 — test coverage (shared helpers, OO API tests).
* **Update**: Cluster 8 — memoization and context optimization (Arc clone removal, pos_at rewrite).

## 2026-07-09

* **Release**: v0.2.0 — Trees refactor + CLI formatting.
* **Update**: Cluster 7 — tree merge/append performance (LinkedList → Vec, accumulator pattern).
* **Update**: Cluster 6 — API surface simplification (removed _to_json wrappers).
* **Update**: Cluster 5 — ExpKind traversal helpers.
* **Update**: Cluster 4 — TreeMap optimization (TreeMapBuilder).
* **Update**: Cluster 3 — FlagMap to u8 bitfield.
* **Update**: Cluster 2 — NullTracer hot-path fix.
* **Update**: Cluster 1 — safety fixes (unsafe removal, panic conversion).
* **Update**: Cluster 0 — safe cleanup (dead code, typos).

## 2026-06-14

* **Release**: v0.1.4 — New syntax + semantics (@name, @int, @uint, @float, @bool meta syntax).

## 2026-06-06

* **Release**: v0.1.3 — Parallel run command in CLI.

## 2026-05-01

* **Creation**: Initial project structure with PyO3/Maturin configuration.
