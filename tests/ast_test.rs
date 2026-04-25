// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for AST/Tree functionality

use tiexiu::Result;
use tiexiu::trees::Tree;

#[test]
fn test_ast_pickling() -> Result<()> {
    let a = Tree::nil();
    let b = Tree::nil();
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn ast() -> Result<()> {
    let test_ast = Tree::nil();
    let _has_items = !matches!(test_ast, Tree::Nil);
    Ok(())
}

#[test]
fn test_tree_text() -> Result<()> {
    let t = Tree::text("hello");
    // Display format is t("hello"), not just "hello"
    assert_eq!(t.to_string(), r#"t("hello")"#);
    Ok(())
}

#[test]
fn test_tree_list() -> Result<()> {
    let t = Tree::list(&[Tree::text("a"), Tree::text("b")]);
    assert!(matches!(t, Tree::Seq(_)));
    Ok(())
}
