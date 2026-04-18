// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for AST/Tree functionality

use tiexiu::trees::Tree;

#[test]
fn test_ast_pickling() {
    let a = Tree::nil();
    let b = Tree::nil();
    assert_eq!(a, b);
}

#[test]
fn ast() {
    let test_ast = Tree::nil();
    let _has_items = !matches!(test_ast, Tree::Nil);
}

#[test]
fn test_tree_text() {
    let t = Tree::text("hello");
    assert_eq!(t.to_string(), "hello");
}

#[test]
fn test_tree_list() {
    let t = Tree::list(&[Tree::text("a"), Tree::text("b")]);
    assert!(matches!(t, Tree::Seq(_)));
}
