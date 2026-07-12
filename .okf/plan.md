---
type: Playbook
title: OKF Bundle Population Plan
description: Step-by-step plan to populate the .okf/ knowledge bundle.
tags: [plan, okf, knowledge]
timestamp: 2026-07-12T00:00:00Z
---

# Plan: Populate `.okf/` Knowledge Bundle

Each step creates a branch, adds concept files with OKF-compliant frontmatter, commits, and waits for merge.

## Workflow

1. Create branch from `main`
2. Create concept files
3. Run `just test`
4. Commit
5. Wait for merge before next step

## Steps

### Step 1: Bootstrap (current)

**Branch:** `okf/bootstrap`
**Files:** `.okf/index.md`, `.okf/log.md`, `.okf/plan.md`

Bundle root with project overview, chronological history, and this plan.

### Step 2: Architecture — Performance

**Branch:** `okf/architecture`
**Files:** `.okf/architecture/index.md`, `.okf/architecture/performance.md`

Amdahl's Law, linear-scan lower bound, memory wall, OgoPEGo confirmation.

### Step 3: Architecture — Cursor

**Branch:** `okf/cursor`
**Files:** `.okf/architecture/cursor.md`

24-byte StrCursor, Arc<str> + usize + Arc<CursorHeavy>, copy-on-write, pos_at single-pass.

### Step 4: Architecture — Tree

**Branch:** `okf/tree`
**Files:** `.okf/architecture/tree.md`

32-byte Tree enum, TreeMapBuilder, clean_and_fold accumulator, nil purging.

### Step 5: Architecture — Context

**Branch:** `okf/context`
**Files:** `.okf/architecture/context.md`

CoreCtx, &mut CtxSem, memoization cache, RuleFlags bitfield, NullTracer override.

### Step 6: Architecture — PyO3 Boundary

**Branch:** `okf/pyo3`
**Files:** `.okf/architecture/pyo3-boundary.md`

TieXiuPy/GrammarPy wrappers, Arc::make_mut CoW, pykwargs_to_cfg, pythonize_json_value.

### Step 7: API — Rust

**Branch:** `okf/api-rust`
**Files:** `.okf/api/index.md`, `.okf/api/rust.md`

15 fnapi.rs functions, TieXiu struct, pegapi() entry point.

### Step 8: API — Python

**Branch:** `okf/api-python`
**Files:** `.okf/api/python.md`

OO API (pegapi), Functional API (parse, compile), Grammar class, JSON type mapping.

### Step 9: CLI Reference

**Branch:** `okf/cli`
**Files:** `.okf/cli/reference.md`

tiexiu run, tiexiu boot, tiexiu grammar with arguments and examples.

### Step 10: Grammar — Non-Features

**Branch:** `okf/grammar`
**Files:** `.okf/grammar/index.md`, `.okf/grammar/non-features.md`

TatSu features deliberately excluded from TieXiu.

### Step 11: Status — Roadmap

**Branch:** `okf/status`
**Files:** `.okf/status/index.md`, `.okf/status/roadmap.md`

All 11 improvement clusters completed.
