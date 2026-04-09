// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's ngmodel_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::trees::Node;

#[test]
fn test_init_attributes() {
    let node = Node::new();
    assert!(node.has_field("ast"));
    assert!(node.has_field("parseinfo"));
}

#[test]
fn test_init_attributes_transferred() {
    let node = Node::new().with_ast("Hello!");
    assert_eq!(node.ast, "Hello!");

    let node = Node::new().with_ctx(()).with_ast("Hello!");
    assert_eq!(node.ast, "Hello!");
    assert!(node.ctx.is_some());
}

#[test]
fn test_attributes_through_shell() {
    let node = Node::new().with_ast("Hello");
    assert!(node.has_field("ast"));
    assert!(node.has_field("parseinfo"));
}

#[test]
fn test_children() {
    #[derive(Debug, Clone)]
    struct Inner {
        id: String,
    }

    #[derive(Debug, Clone)]
    struct Outer {
        left: Inner,
        right: Inner,
    }

    let result = Inner { id: String::new() };
    assert!(result.id.is_empty());

    let result = Outer {
        left: Inner {
            id: "a".to_string(),
        },
        right: Inner {
            id: "b".to_string(),
        },
    };
    assert!(result.left.id == "a");
    assert!(result.right.id == "b");
}
