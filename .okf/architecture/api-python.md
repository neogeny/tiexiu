---
type: Architecture
title: Python API
description: Python API with functional and OO styles, returning JSON-compatible objects.
tags: [python, api, bindings, json]
timestamp: 2026-07-12T00:00:00Z
---

# Python API

TieXiu provides Python bindings via PyO3, exposing both functional and OO APIs. Return values are JSON-compatible Python objects.

## Return Type Mapping

| JSON | Python |
|------|--------|
| object | dict |
| array | list |
| string | str |
| number (int) | int |
| number (real) | float |
| true | True |
| false | False |
| null | None |

## OO API (Recommended)

The OO API provides a `TieXiu` class with grammar caching:

```python
from tiexiu import pegapi

tx = pegapi()
tree = tx.parse("start: /hello/", "hello")
json_tree = tx.parse_to_json("start: /hello/", "hello")
grammar = tx.compile("start: /hello/")
```

### TieXiu Methods

| Method | Description |
|--------|-------------|
| `parse(grammar, text)` | Parse input with grammar |
| `parse_to_json(grammar, text)` | Parse to JSON object |
| `compile(grammar)` | Compile grammar (cached) |
| `get(grammar)` | Check grammar cache |
| `get_or_compile(grammar)` | Get or compile grammar |
| `grammar_pretty(grammar)` | Pretty-print grammar |
| `boot_grammar()` | Load boot grammar |

### Grammar Caching

The `TieXiu` instance caches compiled grammars:

```python
tx = pegapi()
# First call compiles and caches
grammar1 = tx.compile("start: /hello/")
# Second call returns from cache
grammar2 = tx.compile("start: /hello/")
assert grammar1 is grammar2  # Same object
```

## Functional API

The functional API provides free functions:

```python
from tiexiu import parse, compile, pretty, boot_grammar

tree = parse("start: /hello/", "hello")
json_tree = parse_to_json("start: /hello/", "hello")
grammar = compile("start: /hello/")
```

### Functions

| Function | Description |
|----------|-------------|
| `parse(grammar, text)` | Parse input with grammar |
| `parse_to_json(grammar, text)` | Parse to JSON object |
| `parse_to_json_string(grammar, text)` | Parse to JSON string |
| `compile(grammar)` | Compile grammar |
| `compile_to_json(grammar)` | Compile to JSON |
| `compile_to_json_string(grammar)` | Compile to JSON string |
| `pretty(grammar)` | Pretty-print grammar |
| `boot_grammar()` | Load boot grammar |
| `boot_grammar_to_json()` | Boot grammar to JSON |
| `boot_grammar_to_json_string()` | Boot grammar to JSON string |
| `boot_grammar_pretty()` | Pretty-print boot grammar |

## Compiled Grammar

Grammars can be compiled and reused:

```python
from tiexiu import compile

grammar = compile("start: /hello/")
tree = grammar.parse("hello")
json_tree = grammar.parse_to_json("hello")
```

## Configuration

Keyword arguments can be passed for runtime configuration:

```python
tree = tx.parse("start: /hello/", "hello", trace=True)
```

## JSON Variants

All parse and compile functions have `_to_json` and `_to_json_string` variants:

```python
# Return Python dict/list
json_obj = tx.parse_to_json("start: /hello/", "hello")

# Return JSON string
json_str = tx.parse_to_json_string("start: /hello/", "hello")
```

## Import

```python
from tiexiu import pegapi, parse, compile, pretty, boot_grammar
```

## Source Files

- `src/python/pymodule.rs` — Module registration
- `src/python/pyfnapi.rs` — Functional API
- `src/python/pyooapi.rs` — OO API
- `src/python/grammar.rs` — Grammar wrapper
- `src/python/tree.rs` — Tree conversion
