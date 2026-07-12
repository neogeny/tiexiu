---
type: Playbook
title: OKF Bundle Gap Plan
description: Fill missing concept files and fix quality issues in the .okf/ knowledge bundle.
tags: [plan, okf, knowledge, gaps]
timestamp: 2026-07-12T00:00:00Z
---

# Plan: Fill `.okf/` Gaps

Previous plan (11 steps) archived in `log.md`. This plan fills missing concept files and fixes quality issues.

## Workflow

1. Create branch from `main`
2. Create or edit concept files with OKF-compliant frontmatter
3. Run `just test`
4. Commit
5. Wait for merge before next step

## Steps

### Phase 1: Quality Fixes

| Step | Name | Files | Status |
|------|------|-------|--------|
| 1 | Fix CHANGELOG URLs | `CHANGELOG.md` | Pending |
| 2 | Fix SYNTAX stale references | `SYNTAX.md` | Pending |
| 3 | Fix architecture link paths | `.okf/architecture/*.md` | Pending |
| 4 | Remove stale fragments | `fragments/atomlist.md` | Pending |
| 5 | Normalize frontmatter types | `.okf/architecture/*.md` | Pending |

### Phase 2: Missing Concept Files — Core

| Step | Name | Files | Status |
|------|------|-------|--------|
| 6 | Error handling | `.okf/architecture/errors.md` | Pending |
| 7 | Semantics / Transformations | `.okf/architecture/semantics.md` | Pending |
| 8 | Configuration system | `.okf/architecture/config.md` | Pending |

### Phase 3: Missing Concept Files — Infrastructure

| Step | Name | Files | Status |
|------|------|-------|--------|
| 9 | Testing strategy | `.okf/architecture/testing.md` | Pending |
| 10 | Bootstrap grammar | `.okf/architecture/bootstrap.md` | Pending |
| 11 | Thread safety | `.okf/architecture/concurrency.md` | Pending |

### Phase 4: Missing Concept Files — Extended

| Step | Name | Files | Status |
|------|------|-------|--------|
| 12 | Code generation | `.okf/architecture/codegen.md` | Pending |
| 13 | JSON serialization | `.okf/architecture/json.md` | Pending |
| 14 | Tools | `.okf/architecture/tools.md` | Pending |
| 15 | Input abstractions | `.okf/architecture/input.md` | Pending |

### Phase 5: Bundle Update

| Step | Name | Files | Status |
|------|------|-------|--------|
| 16 | Update architecture index | `.okf/architecture/index.md` | Pending |
| 17 | Update root index | `.okf/index.md` | Pending |
| 18 | Update log | `.okf/log.md` | Pending |

## Gap Details

### Step 1: Fix CHANGELOG URLs

Root `CHANGELOG.md` lines 11-14 reference `neogeny/ogopego` instead of `neogeny/tiexiu` in compare URLs.

### Step 2: Fix SYNTAX stale references

Root `SYNTAX.md` contains Python-specific references that don't apply to TieXiu:
- "If a name collides with a Python keyword or builtin" (line 24)
- References to Python's `re` module (line 282)
- References to `ast.literal_eval()` (line 346)
- References to Python's `str.format()` (line 349)
- Outdated `Tree` enum definition (uses `Box<str>` instead of `Str`)

### Step 3: Fix architecture link paths

Architecture files use absolute paths like `/architecture/performance.md`. Convert to relative paths (e.g., `performance.md` for same-directory, `../index.md` for parent).

### Step 4: Remove stale fragments

`fragments/atomlist.md` is a stale note about a past task. Remove the file and the `fragments/` directory.

### Step 5: Normalize frontmatter types

All architecture files use `type: Architecture`. Change to more specific types:
- Design documents: `type: Design`
- API documents: `type: API`
- Component documents: `type: Component`

### Step 6: Error handling

Create `.okf/architecture/errors.md` covering:
- `ParseFailure` vs `ParseError` distinction
- `DisasterReport` (furthest failure position and message)
- Error propagation through backtracking
- The cut operator's effect on error reporting
- User-facing error formatting
- Source files: `src/error.rs`

### Step 7: Semantics / Transformations

Create `.okf/architecture/semantics.md` covering:
- Post-rule transformation pipeline
- `SemanticsRef` and `apply_semantics`
- The `Folds` trait and tree folding
- `Tree::fold()` — resolving internal nodes to final form
- How named elements, overrides, and nil purging work
- Writing custom semantics
- Source files: `src/trees/fold.rs`, `src/context/ctx.rs`

### Step 8: Configuration system

Create `.okf/architecture/config.md` covering:
- `CfgKey` enum values and their purposes
- `CfgA` (configuration array) type
- Environment variable loading (`src/cfg/`)
- Grammar directives vs runtime config
- The `Configurable` trait
- Source files: `src/cfg/`

### Step 9: Testing strategy

Create `.okf/architecture/testing.md` covering:
- `just test` pipeline: `cargo fix`, `cargo fmt`, `cargo clippy`, `cargo nextest run --lib --all-features`
- Test organization: unit tests in `src/`, integration tests in `tests/` (40+ files)
- 211 tests, all passing
- Test naming conventions
- How to run specific tests
- Test fixtures and helpers
- Policy: tests may be skipped but must compile; never delete a failing test

### Step 10: Bootstrap grammar

Create `.okf/architecture/bootstrap.md` covering:
- The boot grammar is self-hosted — TieXiu parses its own grammar format
- How the boot grammar is loaded (`src/peg/boot.rs`)
- The boot model — Rust code representation of the grammar
- Bootstrap plan: regenerating Rust code for the bootstrap grammar
- CLI commands: `tiexiu boot --model`
- Source files: `src/peg/boot.rs`

### Step 11: Thread safety

Create `.okf/architecture/concurrency.md` covering:
- Grammar models are immutable after construction — `Send + Sync`
- `RwLock<HashMap<u64, Grammar>>` in OO API for concurrent grammar caching
- `Arc<Tree>` reference counting for tree sharing
- The `TieXiu` struct is safe to share across threads
- CLI parallel parsing: `-n` flag, concurrent task execution
- jemalloc allocator on non-MSVC targets

### Step 12: Code generation

Create `.okf/architecture/codegen.md` covering:
- `src/codegen/` module purpose
- Grammar-to-Rust code generation
- The `--model` CLI flag output
- Current status: pending implementation for bootstrap regeneration
- Source files: `src/codegen/`

### Step 13: JSON serialization

Create `.okf/architecture/json.md` covering:
- `src/json/` module purpose
- Grammar to/from JSON (`Grammar::to_json_string`, `Grammar::from_json`)
- Tree to/from JSON (`Tree.to_value()` for `serde_json::Value`)
- Python API JSON return types
- CLI `--json` output flag
- Source files: `src/json/`

### Step 14: Tools

Create `.okf/architecture/tools.md` covering:
- `src/tools/` module purpose
- Railroad diagram generation (APL characters)
- Grammar pretty-printing
- Syntax highlighting via syntect (base16-eighties.dark theme)
- CLI `--railroads` flag
- Source files: `src/tools/`

### Step 15: Input abstractions

Create `.okf/architecture/input.md` covering:
- `Cursor` trait — object-safe, 24-byte `StrCursor` implementation
- Byte cursor support
- Custom cursor implementations
- `TokenizingPatterns` — whitespace, comments, EOL patterns
- The `input/` module structure
- Source files: `src/input/`

### Step 16: Update architecture index

Add new concept files to `.okf/architecture/index.md`:
- Errors, Semantics, Config, Testing, Bootstrap, Concurrency, Codegen, JSON, Tools, Input

### Step 17: Update root index

Update `.okf/index.md` key facts if version or test count changed.

### Step 18: Update log

Add entries to `.okf/log.md` for all changes in this plan.
