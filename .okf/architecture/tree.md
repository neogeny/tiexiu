---
type: Architecture
title: Tree
description: 32-byte AST node with copy-on-write semantics and merge semantics for named elements.
tags: [tree, ast, memory, merging]
timestamp: 2026-07-12T00:00:00Z
---

# Tree

The tree is the abstract syntax tree (AST) representation for parsed input. Every tree node is at most 32 bytes and is reference-counted for copy-on-write sharing.

## Memory Layout

The `Tree` enum has 11 variants, all fitting within 32 bytes:

| Variant | Size | Purpose |
|---------|------|---------|
| `Text(Str)` | 8 bytes | Leaf node from tokens or patterns |
| `Seq(Ref<[TreeRef]>)` | 8 bytes | Mergeable sequence of values |
| `List(Ref<[TreeRef]>)` | 8 bytes | Non-mergeable list of values |
| `Map(Ref<TreeMap>)` | 8 bytes | Ordered map of named elements |
| `Node { typename, tree }` | 16 bytes | Result of parsing a rule call |
| `Nil` | 0 bytes | No input consumed (internal) |
| `Named(KeyValue)` | 16 bytes | Named element for tree merging (internal) |
| `NamedAsList(KeyValue)` | 16 bytes | Named element forced into list (internal) |
| `Override(TreeRef)` | 8 bytes | Override value for merged tree (internal) |
| `OverrideAsList(TreeRef)` | 8 bytes | Override value forced into list (internal) |
| `Bottom` | 0 bytes | Failure marker for memoization (internal) |

The `KeyValue` struct is 24 bytes: `Str` (8 bytes) + `TreeRef` (8 bytes) + padding.

The `TreeRef` type is `Arc<Tree>` — a reference-counted pointer to a tree node.

## TreeMap

`TreeMap` is an ordered map of named tree elements:

```rust
pub struct TreeMap(pub Ref<[(Str, TreeRef)]>);
```

Each entry is `(Str, TreeRef)` — a key string and a tree reference. The map is stored as a reference-counted slice for copy-on-write sharing.

## TreeMapBuilder

`TreeMapBuilder` accumulates entries for a `TreeMap` without O(n²) cloning:

```rust
pub(crate) struct TreeMapBuilder {
    entries: Vec<(Str, TreeRef)>,
}
```

### Insertion Methods

| Method | Description |
|--------|-------------|
| `insert(key, item)` | Insert value, merging with existing entry for same key |
| `insert_as_list(key, item)` | Insert value into list entry for this key |
| `build()` | Convert accumulated entries into a `TreeMap` |

The builder performs a linear scan for existing keys, merging via `Tree::append()` or `Tree::append_as_list()`. This avoids the O(n²) cost of repeated `Arc::make_mut` on the immutable `TreeMap` slice.

## Merge Semantics

Trees support two kinds of merging:

### Named Merging (`Named` / `NamedAsList`)

When a tree contains `Named(key, value)` nodes, they are collected into a `TreeMap` during folding:

- `Named` — merge with existing entry (sequences are flattened)
- `NamedAsList` — force into list (no flattening)

### Override Merging (`Override` / `OverrideAsList`)

When a tree contains `Override(value)` nodes, they are collected as the root value:

- `Override` — append to root (sequences are flattened)
- `OverrideAsList` — append to root (no flattening)

## Folding

`Tree::fold()` resolves internal nodes (`Named`, `Override`, `Nil`) into the final `Map`/`Seq`/`List` form:

```rust
pub fn fold(tree: TreeRef) -> TreeRef {
    let mut gather = TreeMerge::new();
    let tree = Self::clean_and_fold(&tree, &mut gather);
    // Return root override or map, or the folded tree
}
```

The `clean_and_fold` method walks the tree recursively:

1. **Seq** — fold each element, flatten nested sequences
2. **List** — fold each element, preserve list structure
3. **Named** — fold value, insert into gather map
4. **NamedAsList** — fold value, insert into gather map as list
5. **Override** — fold value, append to gather root
6. **OverrideAsList** — fold value, append to gather root as list
7. **Nil** — return as-is
8. **Other** — return as-is

## Nil Purging

`Tree::Nil` nodes are automatically removed during construction and folding:

- `From<Vec<Tree>>` filters out `Nil` before creating `Seq`
- `From<[Tree; N]>` filters out `Nil` before creating `Seq`
- `clean_and_fold` skips `Nil` in sequences

## Accessor Methods

| Method | Description |
|--------|-------------|
| `value()` | Get text value or debug representation |
| `list_value()` | Get child elements as `Ref<[TreeRef]>` |
| `str_list_value()` | Get child elements as `Ref<[Str]>` |
| `map_value()` | Get inner `TreeMap` if Map variant |
| `get(key)` | Look up key in Map |
| `get_value(key)` | Look up key and get text value |
| `get_list(key)` | Look up key and get list children |
| `get_str_list(key)` | Look up key and get children as text |
| `width()` | Total character width of text nodes |

## Tests

The 32-byte target is enforced by tests:

```rust
const TARGET: usize = 32;

#[test]
fn test_tree_size() {
    let size = size_of::<Tree>();
    assert!(size <= TARGET, "Cst size is {} > {} bytes", size, TARGET);
}

#[test]
fn test_keyval_size() {
    let size = size_of::<KeyValue>();
    assert!(size <= TARGET, "KeyVal size is {} > {} bytes", size, TARGET);
}
```

## Source Files

- `src/trees/tree.rs` — `Tree` enum, `KeyValue`, fold logic
- `src/trees/map.rs` — `TreeMap`, `TreeMapBuilder`
- `src/trees/build.rs` — Tree constructor methods
- `src/trees/fold.rs` — `Folds` trait
