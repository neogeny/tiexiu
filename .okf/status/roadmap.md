---
type: Status
title: Roadmap
description: All improvement clusters completed.
tags: [roadmap, clusters, completed]
timestamp: 2026-07-12T00:00:00Z
---

# Roadmap

All 11 improvement clusters have been completed or intentionally dropped.

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
Removed dead code, fixed documentation typos, eliminated stale artifacts. No behavioral changes.

### Cluster 1: Safety Fixes
Eliminated unsafe code and panicking paths in production code.

### Cluster 2: NullTracer Hot-Path Fix
Eliminated String allocations on every token match when tracing is off.

### Cluster 3: FlagMap to bitflags
Replaced hash-map-based flag storage with zero-allocation bitfield.

### Cluster 4: TreeMap Optimization
Eliminated O(n^2) mutation pattern in TreeMap.

### Cluster 5: ExpKind Visitor / Trait Simplification
Reduced ~35-variant match duplication across 9+ files.

### Cluster 6: API Surface Simplification
Reduced public API from ~24 functions to ~10.

### Cluster 7: Tree merge/append Performance
Eliminated quadratic allocation in tree construction.

### Cluster 8: Memoization and Context Optimization
Reduced Arc clone overhead in parsing hot path.

### Cluster 9: Test Coverage
Filled critical test gaps.

### Cluster 10: Benchmark Improvements
Added meaningful benchmarks for regression detection.

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
