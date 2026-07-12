---
type: Status
title: Roadmap
description: Improvement clusters for TieXiu.
tags: [roadmap, clusters, completed]
timestamp: 2026-07-12T00:00:00Z
---

# Roadmap

A high-performance port of TatSu to Rust.

## Conventions

- **Verify** means: run `just test` (which runs `cargo fix`, `cargo fmt`, `cargo clippy`, and `cargo nextest run --lib --all-features`).
- Each cluster is self-contained and must pass `just test` before proceeding.
- When a cluster references a prior cluster's changes, that cluster is a prerequisite.
- Do not skip clusters. Reorder only with User approval.

## Cluster Summary

| Cluster | Name | Status |
|---------|------|--------|
| 0 | Safe Cleanup | Completed |
| 1 | Safety Fixes | Completed |
| 2 | NullTracer Hot-Path Fix | Completed |
| 3 | FlagMap to bitflags | Completed |
| 4 | TreeMap Optimization | Completed |
| 5 | ExpKind Visitor / Trait Simplification | Completed |
| 6 | API Surface Simplification | Completed |
| 7 | Tree merge/append Performance | Completed |
| 8 | Memoization and Context Optimization | Completed |
| 9 | Test Coverage | Completed |
| 10 | Benchmark Improvements | Completed |
| 11 | Documentation and Housekeeping | Dropped |

## Cluster Details

### Cluster 0: Safe Cleanup

Remove dead code, fix documentation typos, eliminate stale artifacts. No behavioral changes. Lowest risk.

### Cluster 1: Safety Fixes

Eliminate unsafe code and panicking paths in production code.

### Cluster 2: NullTracer Hot-Path Fix

Eliminate the single largest performance waste: String allocations on every token match when tracing is off.

### Cluster 3: FlagMap to bitflags

Replace hash-map-based flag storage with a zero-allocation bitfield. Simplifies `Rule` and eliminates 6 hash lookups per flag access.

### Cluster 4: TreeMap Optimization

Eliminate O(n^2) mutation pattern in `TreeMap`. Each `insert` previously cloned the entire slice, did linear scans, and re-wrapped in `Arc`.

### Cluster 5: ExpKind Visitor / Trait Simplification

Reduce the ~35-variant match duplication across 9+ files. When a new `ExpKind` variant is added, only one place needs updating instead of 9.

### Cluster 6: API Surface Simplification

Reduce the public API from ~24 functions to ~10. Eliminate the `_to_json` / `_to_json_string` combinatorial explosion.

### Cluster 7: Tree merge/append Performance

Eliminate quadratic allocation in tree construction.

### Cluster 8: Memoization and Context Optimization

Reduce Arc clone overhead in the parsing hot path.

### Cluster 9: Test Coverage

Fill critical test gaps so future clusters have a safety net.

### Cluster 10: Benchmark Improvements

Add meaningful benchmarks for regression detection.

### Cluster 11: Documentation and Housekeeping

Dropped. The `fragments/` directory and `trees/fold.rs` lack documentation and tests, making this cluster inadvisable without first addressing those gaps.

## Execution Order

```
Cluster 0  (Safe cleanup)        -- DONE
Cluster 1  (Safety fixes)        -- DONE
Cluster 2  (NullTracer fix)      -- DONE
Cluster 3  (FlagMap -> bitflags) -- DONE
Cluster 4  (TreeMap optimization) -- DONE
Cluster 5  (ExpKind visitor)      -- DONE
Cluster 6  (API simplification)   -- DONE
Cluster 7  (Tree merge perf)      -- DONE
Cluster 8  (Memoization/context)  -- DONE
Cluster 9  (Test coverage)        -- DONE
Cluster 10 (Benchmarks)           -- DONE
Cluster 11 (Documentation)        -- DROPPED
```

**All functional clusters completed.**
