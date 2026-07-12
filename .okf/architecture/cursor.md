---
type: Architecture
title: Cursor
description: 24-byte parsing cursor with copy-on-write semantics and object-safe trait.
tags: [cursor, memory, parsing, input]
timestamp: 2026-07-12T00:00:00Z
---

# Cursor

The cursor is the input abstraction for parsing. It tracks position in the input text and provides matching operations for tokens, patterns, and meta-expressions.

## Memory Layout

`StrCursor` uses 24 bytes:

| Field | Type | Size | Purpose |
|-------|------|------|---------|
| `text` | `Arc<str>` | 8 bytes | Shared reference to input text |
| `offset` | `usize` | 8 bytes | Current byte position |
| `heavy` | `Arc<CursorHeavy>` | 8 bytes | Shared configuration |

The `CursorHeavy` struct holds heavyweight, rarely-changing state:

```rust
struct CursorHeavy {
    ignorecase: bool,
    nameguard: bool,
    namechars: String,
    source: String,
    patterns: Ref<TokenizingPatterns>,
}
```

## Copy-on-Write Semantics

During a parse, cursors that don't advance over the input share the same underlying `Arc<str>` text and `Arc<CursorHeavy>` configuration. Only the `offset` changes. This means:

- Creating a cursor for a branch point is essentially a copy of 24 bytes
- No text duplication occurs until a branch modifies the cursor
- Grammar elements that don't advance share the same cursor instance

## Cursor Trait

The `Cursor` trait (`src/input/cursor.rs`) is object-safe and provides:

### Core Operations

| Method | Description |
|--------|-------------|
| `mark()` | Save current position (returns `usize`) |
| `reset(mark)` | Restore a saved position |
| `next()` | Advance and return next char |
| `peek()` | Look at next char without advancing |
| `at_end()` | Check if input is exhausted |

### Matching Operations

| Method | Description |
|--------|-------------|
| `match_token(token)` | Match literal string with name guard |
| `match_pattern(pattern)` | Match regex pattern |
| `match_eol()` | Match end-of-line |
| `next_token()` | Skip whitespace and comments |

### Meta-Expression Matchers

| Method | Description |
|--------|-------------|
| `match_name()` | Match identifier (`@name`) |
| `match_int()` | Match signed integer (`@int`) |
| `match_uint()` | Match unsigned integer (`@uint`) |
| `match_float()` | Match floating-point (`@float`) |
| `match_bool()` | Match boolean literal (`@bool`) |

## Position Tracking

`pos_at(mark)` computes line and column from a byte offset in a single pass:

```rust
fn pos_at(&self, mut mark: usize) -> (usize, usize) {
    mark = mark.min(self.as_str().len());
    let head = &self.as_str()[0..mark];
    let mut line = 1;
    let mut col = 0;
    for ch in head.chars() {
        if ch == '\n' { line += 1; col = 0; } else { col += 1; }
    }
    (line, col)
}
```

## Configuration

`CursorHeavy` is configured via `Configurable::configure()`:

- `ignorecase` — case-insensitive token matching
- `nameguard` — prevent partial name matches (e.g., `IN` not matching `INITIALIZE`)
- `namechars` — additional characters valid in names
- `patterns` — tokenizing patterns (whitespace, comments, EOL)

## Source Files

- `src/input/cursor.rs` — `Cursor` trait definition
- `src/input/strcursor.rs` — `StrCursor` implementation
