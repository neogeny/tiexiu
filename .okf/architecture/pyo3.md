---
type: Architecture
title: PyO3
description: Python bindings via PyO3 with functional and OO API styles.
tags: [python, pyo3, bindings, ffi]
timestamp: 2026-07-12T00:00:00Z
---

# PyO3

TieXiu provides Python bindings via PyO3, exposed as the `_tiexiu` module. The bindings support two API styles: functional (free functions) and OO (class-based).

## Module Structure

```
src/python/
  mod.rs        — Module root
  pymodule.rs   — PyO3 module registration
  pyfnapi.rs    — Functional API (free functions)
  pyooapi.rs    — OO API (TieXiuPy class)
  grammar.rs    — GrammarPy wrapper
  tree.rs       — Tree conversion utilities
  util.rs       — Shared utilities
```

## Module Registration

The `_tiexiu` module is registered via `#[pymodule]`:

```rust
#[pymodule(name = "_tiexiu")]
pub fn tiexiu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("ParseError", m.py().get_type::<ParseError>())?;
    // Register functions and classes...
}
```

## Exception Types

| Exception | Base | Purpose |
|-----------|------|---------|
| `ParseError` | `PyException` | Parse failures with error details |

## Functional API

Free functions in `pyfnapi.rs`:

### Grammar Operations

| Function | Description |
|----------|-------------|
| `parse_grammar(text)` | Parse EBNF grammar text |
| `parse_grammar_to_json(text)` | Parse grammar to JSON value |
| `parse_grammar_to_json_string(text)` | Parse grammar to JSON string |
| `compile_to_json(grammar_text)` | Compile grammar to JSON |
| `compile_to_json_string(grammar_text)` | Compile grammar to JSON string |
| `compile(grammar_text)` | Compile grammar to Grammar object |
| `grammar_pretty(grammar)` | Pretty-print grammar |

### Parsing Operations

| Function | Description |
|----------|-------------|
| `parse_input(grammar, input)` | Parse input with grammar |
| `parse_input_to_json(grammar, input)` | Parse input to JSON |
| `parse_input_to_json_string(grammar, input)` | Parse input to JSON string |
| `parse(grammar, rule, input)` | Parse with specific rule |
| `parse_to_json(grammar, rule, input)` | Parse to JSON |
| `parse_to_json_string(grammar, rule, input)` | Parse to JSON string |

### Boot Grammar Operations

| Function | Description |
|----------|-------------|
| `load_boot_as_json()` | Load boot grammar as JSON |
| `boot_grammar_to_json()` | Boot grammar to JSON |
| `boot_grammar_to_json_string()` | Boot grammar to JSON string |
| `boot_grammar_pretty()` | Pretty-print boot grammar |
| `boot_grammar()` | Get boot grammar object |

### Utility Operations

| Function | Description |
|----------|-------------|
| `pegapi(grammar, input)` | Low-level PEG API |
| `pretty(grammar, input)` | Pretty-print parse result |

## OO API

### TieXiuPy

The main parser class:

```python
from _tiexiu import TieXiu

parser = TieXiu(grammar_text)
tree = parser.parse(rule_name, input_text)
json = parser.parse_to_json(rule_name, input_text)
```

### GrammarPy

Grammar wrapper class:

```python
from _tiexiu import Grammar

grammar = Grammar(grammar_text)
# Use with TieXiu or functional API
```

## Grammar Serialization

Grammars can be serialized to/from JSON:

```python
# Grammar to JSON
json_value = parse_grammar_to_json(grammar_text)

# JSON to Grammar
grammar = compile(json_value)
```

## Feature Flag

PyO3 bindings are gated behind the `pyo3` feature flag:

```toml
[dependencies]
pyo3 = { version = "0.22", optional = true }

[features]
pyo3 = ["dep:pyo3"]
```

## Allocator

When PyO3 is enabled on non-MSVC targets, jemalloc is used:

```rust
#[cfg(all(feature = "pyo3", not(target_env = "msvc")))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
```

## Source Files

- `src/python/pymodule.rs` — Module registration
- `src/python/pyfnapi.rs` — Functional API
- `src/python/pyooapi.rs` — OO API
- `src/python/grammar.rs` — Grammar wrapper
- `src/python/tree.rs` — Tree conversion
- `src/python/util.rs` — Shared utilities
