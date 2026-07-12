# 铁修 TieXiu Roadmap

A high-performance port of **TatSu** to Rust.

The CLI (`cargo run -- --help`) exercises everything currently implemented and is the best starting place to learn about the library.

## Conventions

- **Verify** means: run `just test` (which runs `cargo fix`, `cargo fmt`, `cargo clippy`, and `cargo nextest run --lib --all-features`).
- Each cluster is self-contained and must pass `just test` before proceeding.
- Cluster descriptions list files to modify. Line numbers are approximate -- confirm before editing.
- When a cluster references a prior cluster's changes, that cluster is a prerequisite.
- Do not skip clusters. Reorder only with User approval.

---

## Cluster 0: Safe Cleanup

**Goal:** Remove dead code, fix documentation typos, eliminate stale artifacts. No behavioral changes. Lowest risk.

**Verification:** `just test` -- no regressions.

### 0a. Delete dead source files

| File | Reason |
|------|--------|
| `src/context/stackctx.rs` | References non-existent traits (`CtxI`), has syntax error (extra `}` at line 116), is commented out in `mod.rs:18-20`. |
| `src/json/tryfrom.rs` | Empty file (1 blank line). |

In `src/context/mod.rs`: remove the commented-out `mod stackctx;` lines (lines 18-20).
In `src/json/mod.rs`: remove `mod tryfrom;`.

### 0b. Delete dead test files

| File | Reason |
|------|--------|
| `tests/fixtures_test.rs` | Empty file. |
| `tests/fixtures/data.rs` | Empty file. |
| `tests/json_crate_test.rs` | Tests the third-party `json` crate, not TieXiu. |

### 0c. Delete `grammar/include.tatsu`

Empty placeholder for `@@include` which will never be implemented.

### 0d. Fix DESIGN.md stale claims

| Line | Current | Correct |
|------|---------|---------|
| 8 | "16-byte stack footprint" | StrCursor is 24 bytes; CoreCtx is larger. Update to reflect current sizes. |
| 12 | "Transitios" | "Transitions" |
| 19 | "64-bytes per atomic node" | Tree is 32 bytes (verified by test at `src/trees/tree.rs:322`). |
| 52-53 | "16-byte Handle Size" / "32-byte State Changes" | These no longer apply with `&mut CtxSem` semantics. Update or remove. |

### 0e. Fix typos in source

| File | Line | Typo | Fix |
|------|------|------|-----|
| `src/api/fnapi.rs` | 12 (import) | `new_ebnf_grammar_sematics` | Rename function in `src/peg/ebnf_semantics.rs:18` to `new_ebnf_grammar_semantics` |
| `src/error.rs` | 46 | `Regex` variant message says "JSON import/export" | Change to "Regex compilation failed" |
| `grammar/java.ebnf` | 673 | `ExpressionStatment` | `ExpressionStatement` |

### 0f. Remove stray `#[allow(dead_code)]` items

| File | Line | Item |
|------|------|------|
| `src/peg/grammar.rs` | 191 | `parse_input_from()` -- either make `pub` or remove |
| `src/peg/grammar.rs` | 240 | `get_rule_mut()` -- same |
| `src/peg/fold.rs` | 9 | `Folder` trait -- either implement or remove |
| `src/context/state.rs` | 28-35 | `Alert` struct -- unused |
| `src/tools/rails.rs` | 6 | Module-level `#![allow(dead_code)]` -- audit and remove |

---

## Cluster 1: Safety Fixes

**Goal:** Eliminate unsafe code and panicking paths in production code.

**Verification:** `just test` -- no regressions.

### 1a. Remove `unsafe impl Send/Sync` for `CfgKey`

`src/cfg/keys.rs:170-171` -- `CfgKey` contains `Heartbeat(Arc<dyn Heartbeat>)` and `Semantics(Arc<dyn Semantics>)`. Both trait objects should be `Send + Sync` at their definition sites. Verify compiler auto-derives correctly. If not, add `Send + Sync` bounds to the trait definitions in `src/cfg/heartbeat.rs` and `src/context/semantics.rs` instead of `unsafe impl`.

### 1b. Remove `unsafe impl Send/Sync` for `Cfg<K>`

`src/util/cfg.rs:19-20` -- `Cfg<K>` wraps `Box<[K]>` where `K: Send + Sync`. The compiler auto-derives `Send + Sync`. Remove the `unsafe impl` blocks.

### 1c. Replace panicking constructors with `Result`

- `src/peg/build.rs:27` -- `Exp::pattern()` calls `.expect("Invalid regex pattern")`. Change signature to `pub fn pattern(pattern: &str) -> Result<Self, CompileError>`. Update callers.
- `src/peg/compiler.rs:101-103` -- Replace `.expect("...")` with `?` using appropriate `CompileError` variants.
- `src/context/ctx.rs:44-45` -- `track_recursion_depth` calls `panic!`. Return `Err(Nope::RecursionDepthExceeded)` instead.

### 1d. Fix Python binding `assume_attached()` safety

All `#[pymethods]` functions in `src/python/pyooapi.rs` and `src/python/pyfnapi.rs` use `unsafe { Python::assume_attached() }`. PyO3 provides `py: Python<'_>` as a parameter in `#[pymethods]`. Replace all 18 instances with the safe `py` parameter. Also fix `src/python/tree.rs:75`.

---

## Cluster 2: NullTracer Hot-Path Fix

**Goal:** Eliminate the single largest performance waste: String allocations on every token match when tracing is off.

**Verification:** `just test` + `cargo bench` -- benchmarks should improve noticeably.

### 2a. Override `trace_match`/`trace_no_match` on `NullTracer`

`src/context/trace.rs:111-129` -- The `Tracer` trait default methods allocate `String`s. `NullTracer` (used when tracing is off) does not override them.

Add to `NullTracer`:
```rust
impl Tracer for NullTracer {
    fn trace_match(&self, _ctx: &dyn Ctx, _token: &str, _name: &str) -> bool { true }
    fn trace_no_match(&self, _ctx: &dyn Ctx, _token: &str, _name: &str) -> bool { false }
    fn trace(&self, _ctx: &dyn Ctx, _event: &str) {}
}
```

### 2b. Short-circuit tracing in Ctx methods

`src/context/ctx.rs:97-119` -- `match_token`, `match_pattern`, etc. call `trace_event` even when `NullTracer` is in use. Add a fast-path check at the top of each match method: if tracing is off (the `Tracer` type is `NullTracer`), skip all formatting. This may require a `fn tracing_enabled(&self) -> bool` method on the `Ctx` trait or a type-level check.

---

## Cluster 3: FlagMap to bitflags

**Goal:** Replace hash-map-based flag storage with a zero-allocation bitfield. Simplifies `Rule` and eliminates 6 hash lookups per flag access.

**Verification:** `just test` + `cargo bench`.

### 3a. Define `RuleFlags` bitflags

`src/peg/rule.rs:48` -- Replace `FlagMap = IndexMap<Str, bool>` with:
```rust
bitflags::bitflags! {
    pub(crate) struct RuleFlags: u8 {
        const IS_NAME = 0b0000_0001;
        const IS_TOKN = 0b0000_0010;
        const IS_MEMO = 0b0000_0100;
        const IS_LREC = 0b0000_1000;
        const NO_MEMO = 0b0001_0000;
        const NO_STAK = 0b0010_0000;
    }
}
```

Add `bitflags` as a dependency.

### 3b. Refactor `Rule` to use `RuleFlags`

- Replace `flags: FlagMap` field with `flags: RuleFlags`.
- Change `flag(FLAG_XXX)` calls to `self.flags.contains(RuleFlags::XXX)`.
- Consolidate `has_is_name_flag()`, `has_is_tokn_flag()`, etc. into `is_name()`, `is_tokn()`, etc. directly. Remove the double indirection.
- Update `Rule::from_parts()` -- replace 6 boolean parameters with `RuleFlags`.

### 3c. Update JSON import/export

`src/json/import.rs` and `src/json/export.rs` -- Ensure flag serialization/deserialization maps correctly to/from the new bitflags representation. The JSON format should remain unchanged for backward compatibility.

---

## Cluster 4: TreeMap Optimization

**Goal:** Eliminate O(n^2) mutation pattern in `TreeMap`. Each `insert` currently clones the entire slice, does linear scans, and re-wraps in `Arc`.

**Verification:** `just test` + `cargo bench` (any sequence-parsing benchmarks should improve).

### 4a. Add builder pattern for `TreeMap`

`src/trees/map.rs` -- Add a `TreeMapBuilder` that accumulates `(Str, TreeRef)` pairs into a `Vec` without cloning on each insert:

```rust
pub(crate) struct TreeMapBuilder {
    entries: Vec<(Str, TreeRef)>,
}

impl TreeMapBuilder {
    pub fn new() -> Self { ... }
    pub fn insert(&mut self, key: Str, value: TreeRef) { ... }
    pub fn insert_as_list(&mut self, key: Str, value: TreeRef) { ... }
    pub fn build(self) -> TreeMap {
        // Sort, dedup, convert to Arc<[...]> once
    }
}
```

### 4b. Use builder in fold and merge paths

`src/trees/tree.rs` -- `clean_and_fold()`, `merge()`, `append()` should accumulate into a `TreeMapBuilder` and finalize once, rather than creating intermediate `TreeMap` clones per operation.

---

## Cluster 5: ExpKind Visitor / Trait Simplification

**Goal:** Reduce the ~35-variant match duplication across 9+ files. When a new `ExpKind` variant is added, only one place needs updating instead of 9.

**Verification:** `just test` + `cargo bench` (no regressions).

### 5a. Add `ExpKind` traversal helpers

`src/peg/exp.rs` -- Add methods to `ExpKind`:
```rust
impl ExpKind {
    pub fn is_leaf(&self) -> bool { ... }
    pub fn single_child(&self) -> Option<&Exp> { ... }
    pub fn children(&self) -> impl Iterator<Item = &Exp> { ... }
}
```

### 5b. Refactor analysis passes to use helpers

Update these files to use the new helpers instead of re-matching every variant:
- `src/peg/analysis/fold.rs` -- `children()`
- `src/peg/analysis/nullability.rs` -- `is_nullable()`, `callable_from()`, `callable_from_mut()`
- `src/peg/analysis/leftrec.rs` -- `first_calls()`
- `src/peg/analysis/defines.rs` -- `_defines()`
- `src/peg/codegen/mod.rs` -- `collect_fields()`, `is_constant_text()`

### 5c. Unify `callable_from` / `callable_from_mut`

`src/peg/analysis/nullability.rs:93-224` -- Replace the duplicated ~130 lines with a single generic implementation or a macro that parameterizes over `&` vs `&mut`.

---

## Cluster 6: API Surface Simplification

**Goal:** Reduce the public API from ~24 functions to ~10. Eliminate the `_*_to_json` / `_*_to_json_string` combinatorial explosion. Fix `&mut self` requirements.

**Verification:** `just test` + `cargo test --all-features` (includes Python tests if `pyo3` feature).

### 6a. Remove `_to_json` and `_to_json_string` functions from `fnapi.rs`

`src/api/fnapi.rs` -- Remove the 8 JSON-wrapper functions. Keep only the core operations that return `Tree` or `Grammar`. Callers use `tree.to_json()` / `tree.to_json_string()` directly. Functions to remove:

- `parse_grammar_to_json`, `parse_grammar_to_json_string`
- `compile_to_json`, `compile_to_json_string`
- `load_to_json`, `load_tree_to_json`
- `parse_to_json`, `parse_to_json_string`, `parse_input_to_json`, `parse_input_to_json_string`

### 6b. Fix `TieXiu` method signatures

`src/api/ooapi.rs` -- Change read-only methods from `&mut self` to `&self`:
- `get()`, `parse_grammar()`, `compile()`, `grammar_pretty()`, `boot_grammar()`, etc.

Fix thread-safety: wrap `cfg` field in `Arc<[CfgKey]>` or `RwLock<Box<[CfgKey]>>` so `update_cfg()` is safe.

### 6c. Fix grammar cache correctness

`src/api/ooapi.rs:28-33` -- Either:
- Use `HashMap<String, Grammar>` with the full grammar text as key (correct but more memory), or
- Use `u64` hash + verify grammar text on match (collision check), or
- Include config in the hash key.

### 6d. Remove unused `_cfg` parameters

`src/api/fnapi.rs` -- Remove `_cfg` from: `load_grammar_from_json`, `load_tree_from_json`, `load_boot`, `boot_grammar_pretty`. Update callers.

### 6e. Remove `get_rule_by_id` alias

`src/peg/grammar.rs:223-230` -- `get_rule_by_id` is a pure delegation to `get_rule_at`. Remove one.

### 6f. Consolidate Python binding duplication

- Extract `pykwargs_to_cfg()` from `src/python/pyooapi.rs:12-27` and `pyfnapi.rs:12-27` into a shared module.
- Extract `pythonize_json_value()` similarly.
- Fix the copy-paste bugs: `parse_grammar` == `parse_grammar_to_json` in `pyooapi.rs:71-98`, `compile` == `compile_to_json` (lines 100-128), `load_boot` == `load_boot_as_json` (lines 183-204). Decide correct behavior and fix.

---

## Cluster 7: Tree merge/append Performance

**Goal:** Eliminate quadratic allocation in tree construction.

**Verification:** `just test` + `cargo bench`.

### 7a. Replace `LinkedList` with `Vec`

`src/trees/tree.rs:12` -- Replace `TreeList = LinkedList<TreeRef>` with `Vec<TreeRef>`. Update `From<TreeList> for Tree`.

### 7b. Use accumulator pattern for merge/append

`src/trees/tree.rs:153-199` -- Instead of cloning `Box<[TreeRef]>` to `Vec`, pushing, and converting back:
- `merge()`: accept `impl IntoIterator<Item = TreeRef>` and collect once.
- `append()`: take ownership of the left `Seq`'s items and push in-place.

### 7c. Optimize `clean_and_fold` loop

`src/trees/tree.rs:202-243` -- Accumulate merge results into a single `Vec` instead of creating intermediate `Seq` nodes per merge call.

---

## Cluster 8: Memoization and Context Optimization

**Goal:** Reduce Arc clone overhead in the parsing hot path.

**Verification:** `just test` + `cargo bench`.

### 8a. Remove redundant `Arc` clones in call.rs

`src/peg/parsing/call.rs:127,133` -- Replace `&lasttree.clone()` with `&lasttree`.

### 8b. Avoid full Tree clone for define

`src/peg/parsing/expressions.rs:39` -- `tree.as_ref().clone()` deep-clones for every define. Consider `Arc::make_mut` or a lazy-define pattern where `define` returns a new `Tree` only when needed.

### 8c. Cache regex compilation in nullability analysis

`src/peg/analysis/nullability.rs:56-61` -- `is_nullable()` for `Pattern` recompiles regex each call. Cache the nullable result during analysis.

### 8d. Optimize `Cursor::pos_at()`

`src/input/cursor.rs:47-53` -- Currently iterates `.lines().count()` and `.lines().last()` separately. Combine into a single pass. Consider caching position at mark.

### 8e. Avoid `Pattern` clone in whitespace eating

`src/input/strcursor.rs:107-108` -- `eat_spaces_no_newlines()` clones `eol` and `cmt` `Pattern` objects (which are `Arc<Regex>`). Borrow instead.

---

## Cluster 9: Test Coverage

**Goal:** Fill critical test gaps so future clusters have a safety net.

**Verification:** `just test-all` (runs all tests including integration tests).

### 9a. Add shared test helpers

Create `tests/common/mod.rs` with:
- `compile_and_parse(grammar: &str, input: &str) -> Tree`
- `compile_and_parse_json(grammar: &str, input: &str) -> json::JsonValue`

Refactor existing tests to use these helpers.

### 9b. Add OO API tests

`tests/ooapi_test.rs` -- Test `TieXiu` struct: `get()`, `get_or_compile()` (verify cache), `update_cfg()`, `compile()`, `parse()`.

### 9c. Add Tree unit tests

`tests/tree_test.rs` -- Test `fold`, `merge`, `append`, `append_as_list`, `width`, `value`, `from_json_str`, `to_json` for all variants.

### 9d. Add TreeMap tests

`tests/treemap_test.rs` -- Test `insert`, `get`, `define`, `insert_as_list`, `safe_key`.

### 9e. Strengthen weak assertions

Replace overly broad assertions in:
- `tests/pretty_print_test.rs:13,27` -- use `assert_eq!`
- `tests/grammar_structure_test.rs:17,41` -- use exact count
- `tests/basic_grammar_test.rs:59` -- check `Tree` directly
- `tests/round_trips_test.rs:26` -- use JSON value comparison

### 9f. Remove dead tests

- Delete 4 `#[ignore]` tests in `tests/parsing_test.rs:12-50` (@@include will never be implemented)
- Delete `tests/firstfollow_test.rs:11` (tests nothing useful)
- Delete `tests/alerts_test.rs:15` (interpolation not implemented)

---

## Cluster 10: Benchmark Improvements

**Goal:** Add meaningful benchmarks for regression detection.

**Verification:** `cargo bench` produces results.

### 10a. Add end-to-end benchmark

`benches/parsing.rs` -- Add benchmark for full pipeline: `compile(grammar) -> parse(text) -> fold(tree) -> to_json()`.

### 10b. Add input-size-scaling benchmarks

Use `criterion::BenchmarkId` with varying input sizes (100, 1000, 10000 tokens) for sequence, choice, and closure parsing.

### 10c. Fix filesystem dependency

`benches/parsing.rs:92-97` -- `bench_grammar_from_json` reads from `grammar/calc.json` with `expect()`. Use `include_str!` at compile time.

### 10d. Add TatSu grammar benchmark

Benchmark parsing TatSu's own grammar (`grammar/tatsu.ebnf`) as a regression test.

---

## Cluster 11: Documentation and Housekeeping

**Goal:** Final polish after all functional changes.

**Verification:** `cargo doc` + `just book` (mdbook build/test).

### 11a. Update README.md API section

Remove the `_to_json` / `_to_json_string` variants from the documented API (after Cluster 6).

### 11b. Add examples

Populate `examples/` with at least:
- `examples/parse.rs` -- Simple grammar compilation and parsing
- `examples/tree_fold.rs` -- Custom tree transformation

### 11c. Clean up `fragments/`

Move any valuable snippets to `docs/design/` or delete. Remove:
- Wrong extensions: `cst_interner.ts`, `test_ebnf.ts`
- Personal notes: `atomlist.md`
- Irrelevant files: `vimrc.tx`, `result.go`, `test_py.py`
- Stray Vim commands: `termcolors.rs:17`

### 11d. Update SYNTAX.md

Document the `nostak` decorator (present in `tatsu.ebnf:89` but undocumented).

---

## Cluster Execution Order

```
Cluster 0  (Safe cleanup)        -- no deps
Cluster 1  (Safety fixes)        -- no deps
Cluster 2  (NullTracer fix)      -- no deps
Cluster 3  (FlagMap -> bitflags) -- no deps
    |
    v  (Clusters 0-3 are independent, can be done in parallel)
    |
Cluster 4  (TreeMap optimization) -- benefits from Cluster 0 cleanup
Cluster 5  (ExpKind visitor)      -- benefits from Cluster 0 cleanup
Cluster 6  (API simplification)   -- benefits from Clusters 1, 3
Cluster 7  (Tree merge perf)      -- benefits from Cluster 0 cleanup
    |
    v  (Clusters 4-7 are mostly independent)
    |
Cluster 8  (Memoization/context)  -- benefits from Clusters 1, 2
Cluster 9  (Test coverage)        -- benefits from all prior clusters
    |
    v  (Cluster 9 provides safety net for remaining work)
    |
Cluster 10 (Benchmarks)           -- after Clusters 2-8 for accurate measurements
Cluster 11 (Documentation)        -- after all functional changes
```

**Minimum viable path:** Clusters 0 -> 1 -> 2 -> 9 (safety + tests) before any other work.
