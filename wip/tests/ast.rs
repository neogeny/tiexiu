// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's ast_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::trees::Tree;

// ============================================================================
// AST Tests
// ============================================================================

#[test]
fn test_ast_pickling() {
    let a = Tree::new();
    let b = Tree::new();
    assert_eq!(a, b);
}

#[test]
fn test_ast() {
    let ast = Tree::new();
    assert!(ast.items().next().is_none());
    assert!(ast.has_method("to_json"));
}

#[test]
fn test_init() {
    let mut ast = Tree::new();
    let data = vec![(0, 0), (1, 2), (2, 4), (3, 6), (4, 8), (5, 10)];
    for (k, v) in data {
        ast.set(k, v);
    }
    let items: Vec<_> = ast.items().collect();
    assert_eq!(items.len(), data.len());
}

#[test]
fn test_empty() {
    let ast = Tree::new();
    assert!(ast.name.is_none());
}

#[test]
fn test_add() {
    let mut ast = Tree::new();
    ast.set("name", "hello");
    assert!(ast.get("name").is_some());
    assert_eq!(ast.get("name").unwrap(), &"hello");

    ast.set("name", "world");
    assert_eq!(ast.get("name").unwrap(), &"world");
}

#[test]
fn test_iter() {
    let mut ast = Tree::new();
    ast.set("name", "hello");
    ast.set("name", "world");
    ast.set("value", 1);
    let keys: Vec<_> = ast.keys().collect();
    assert!(keys.contains(&"name"));
    assert!(keys.contains(&"value"));
}
