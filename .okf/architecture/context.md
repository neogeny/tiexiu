---
type: Architecture
title: Context
description: Mutable parsing context with shared heavyweight state for memoization, patterns, and tracing.
tags: [context, parsing, state, memoization]
timestamp: 2026-07-12T00:00:00Z
---

# Context

The context is the mutable state container for parsing operations. It wraps the cursor position, memo tables, pattern caches, and tracing infrastructure.

## Architecture

The context is split into two parts:

| Part | Type | Purpose |
|------|------|---------|
| `ParseState<U>` | Mutable | Cursor position, recursion tracking, lookahead depth |
| `HeavyState<'t>` | Shared | Memo tables, patterns, keywords, tracer, heartbeat, semantics |

## CoreCtx

`CoreCtx<'c, U>` is the primary parsing context:

```rust
pub struct CoreCtx<'c, U: Cursor> {
    pub state: ParseState<U>,
    pub heavy: HeavyState<'c>,
}
```

### ParseState

| Field | Type | Purpose |
|-------|------|---------|
| `cursor` | `U: Cursor` | The input cursor |
| `keytrack` | `KeyTrack` | Tracks memo key recursion depth |
| `last_cut_mark` | `usize` | Last cut position |
| `lookahead_depth` | `usize` | Current lookahead nesting depth |

### HeavyState

| Field | Type | Purpose |
|-------|------|---------|
| `memos` | `MemoCache` | Memoization cache for rule results |
| `patterns` | `PatternCache` | Compiled regex patterns |
| `keywords` | `Box<[Str]>` | Sorted reserved keywords |
| `furthest_failure` | `Option<DisasterReport>` | Furthest failure position |
| `tracer` | `&dyn Tracer` | Active parse tracer |
| `heartbeat` | `Option<HeartbeatRef>` | Progress callback |
| `semantics` | `Option<SemanticsRef>` | Post-rule transformations |
| `input_len` | `usize` | Total input length |
| `instant` | `Instant` | Last heartbeat timestamp |
| `callstack` | `CallStack` | Nested rule invocations |
| `cutstack` | `Vec<bool>` | Cut operator tracking |

## Traits

### Ctx (Immutable)

The `Ctx` trait provides read-only access to parser state:

```rust
pub trait Ctx: Configurable {
    fn cursor(&self) -> &dyn Cursor;
    fn callstack(&self) -> CallStack;
    fn mark(&self) -> usize;
    fn cut_seen(&self) -> bool;
}
```

### CtxSem (Mutable)

The `CtxSem` trait provides mutable parsing operations:

| Method | Description |
|--------|-------------|
| `cursor_mut()` | Mutable cursor access |
| `enter(name)` / `leave()` | Call stack management |
| `track(key)` / `untrack(key)` | Recursion depth tracking |
| `enter_lookahead()` / `leave_lookahead()` | Lookahead depth management |
| `cut()` | Cut operator handling with memo pruning |
| `push_cut()` / `take_cut()` | Cut stack management |
| `get_pattern(pattern)` | Compiled pattern caching |
| `intern(s)` | String interning |
| `apply_semantics(node, rule, params)` | Post-rule transformations |
| `key(name, can_memo)` / `memo(key)` / `memoize(key, tree, lastmark)` | Memoization |

## StrCtx

`StrCtx<'c>` is a type alias for `CoreCtx<'c, StrCursor>`:

```rust
pub type StrCtx<'c> = CoreCtx<'c, StrCursor>;
```

## Usage Pattern

The context is created once and passed by `&mut` reference through the parse:

```rust
let cursor = StrCursor::new("input text");
let mut ctx = CoreCtx::new(cursor, &[]);
let tree = ctx.parse(&grammar, "rule_name")?;
```

## Key Operations

### Memoization

```rust
fn memoize_rule(ctx: &mut impl CtxSem, key: &MemoKey, tree: &TreeRef, lastmark: usize) {
    ctx.memoize(key, tree, lastmark);
}
```

### Cut Operator

The cut operator (`!`) prevents backtracking and prunes memo tables:

```rust
fn cut(ctx: &mut impl CtxSem) {
    ctx.cut();
    // Prunes memo entries before the cut point
}
```

### Pattern Caching

Patterns are compiled once and cached in `HeavyState`:

```rust
fn match_pattern(ctx: &mut impl CtxSem, pattern: &str) -> Option<Str> {
    let re = ctx.get_pattern(pattern);
    ctx.cursor_mut().match_pattern(&re)
}
```

## Source Files

- `src/context/corectx.rs` — `CoreCtx` implementation
- `src/context/ctx.rs` — `Ctx` and `CtxSem` traits
- `src/context/state.rs` — `ParseState`, `HeavyState`
- `src/context/strctx.rs` — `StrCtx` type alias
- `src/context/memo.rs` — Memoization cache
- `src/context/trace.rs` — Tracing infrastructure
