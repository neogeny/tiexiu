---
type: Architecture
title: Rust API
description: Functional and OO APIs for grammar compilation and parsing.
tags: [api, rust, grammar, parsing]
timestamp: 2026-07-12T00:00:00Z
---

# Rust API

TieXiu provides two API styles for Rust consumers: functional (free functions) and OO (struct-based with caching).

## Module Structure

```
src/api/
  mod.rs    — Re-exports
  fnapi.rs  — Functional API (free functions)
  ooapi.rs  — OO API (TieXiu struct)
```

## Functional API

Free functions in `fnapi.rs`:

### Grammar Operations

| Function | Description |
|----------|-------------|
| `parse_grammar(grammar, cfg)` | Parse EBNF grammar text to tree |
| `parse_grammar_with(cursor, cfg)` | Parse grammar from cursor |
| `compile(grammar, cfg)` | Compile grammar string to Grammar |
| `compile_with(cursor, cfg)` | Compile grammar from cursor |
| `load_grammar_from_json(json)` | Load grammar from JSON string |
| `load_tree_from_json(json)` | Load parse tree from JSON |
| `grammar_pretty(grammar, cfg)` | Pretty-print grammar |

### Parsing Operations

| Function | Description |
|----------|-------------|
| `parse(grammar, text, cfg)` | Parse input with grammar string |
| `parse_input(parser, text, cfg)` | Parse input with compiled Grammar |
| `pegapi()` | Create default TieXiu instance |

### Boot Grammar

| Function | Description |
|----------|-------------|
| `boot_grammar()` | Load boot grammar |
| `load_boot()` | Alias for `boot_grammar` |
| `boot_grammar_pretty()` | Pretty-print boot grammar |

## OO API

The `TieXiu` struct provides a stateful API with grammar caching:

```rust
pub struct TieXiu {
    cfg: Box<[CfgKey]>,
    cache: RwLock<HashMap<u64, Grammar>>,
}
```

### Creation

```rust
// Default (empty config)
let parser = TieXiu::new(&[]);

// With config
let parser = TieXiu::new(&[CfgKey::Trace]);
```

### Methods

| Method | Description |
|--------|-------------|
| `new(cfg)` | Create with config keys |
| `update_cfg(cfg)` | Replace config at runtime |
| `get(grammar)` | Retrieve cached grammar |
| `get_or_compile(grammar)` | Get or compile and cache |
| `compile(grammar)` | Compile (with caching) |
| `compile_with(cursor)` | Compile from cursor |
| `parse_grammar(grammar)` | Parse grammar to tree |
| `parse_grammar_with(cursor)` | Parse grammar from cursor |
| `load(json)` | Load grammar from JSON |
| `load_tree(json)` | Load tree from JSON |
| `grammar_pretty(grammar)` | Pretty-print grammar |
| `parse(grammar, text)` | Parse input |
| `parse_input(parser, text)` | Parse with compiled Grammar |
| `boot_grammar()` | Load boot grammar |
| `load_boot()` | Alias for `boot_grammar` |
| `boot_grammar_pretty()` | Pretty-print boot grammar |

## Grammar Caching

The OO API caches compiled grammars using a `RwLock<HashMap<u64, Grammar>>`:

```rust
pub fn get_or_compile(&self, grammar: &str) -> Result<Grammar> {
    let hash = compute_hash(grammar);
    
    // Check cache first
    {
        let cache = self.cache.read()?;
        if let Some(existing) = cache.get(&hash) {
            return Ok(existing.clone());
        }
    }
    
    // Compile and cache
    let compiled = Grammar::compile(&tree, &self.cfg)?;
    let mut cache = self.cache.write()?;
    cache.insert(hash, compiled.clone());
    Ok(compiled)
}
```

## Configuration

Both APIs accept `CfgA` (configuration array) or `&[CfgKey]`:

```rust
use tiexiu::{CfgKey, compile};

let grammar = compile("expr = @int", &[CfgKey::Trace])?;
```

### Common Config Keys

| Key | Purpose |
|-----|---------|
| `Trace` | Enable parse tracing |
| `Semantics` | Set post-rule transformations |
| `Heartbeat` | Set progress callback |

## Re-exports

The API module re-exports commonly used types:

```rust
pub use crate::cfg::*;
pub use crate::context::new_ctx;
pub use crate::input::{Cursor, StrCursor};
pub use crate::peg::grammar::PrettyPrint;
pub use crate::peg::*;
pub use crate::trees::Tree;
pub use crate::{Error, Result};
```

## Source Files

- `src/api/mod.rs` — Re-exports
- `src/api/fnapi.rs` — Functional API
- `src/api/ooapi.rs` — OO API
