// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::fold::TreeMap;
use super::tree::{Tree, TreeBuild};
use crate::types::Str;
use serde_json::{json, Value};

pub static NIL: Tree = Value::Null;
pub static BOTTOM: Tree = Value::Null;

impl TreeBuild {
    pub fn bottom() -> Tree {
        BOTTOM.clone()
    }

    /// Creates a Text tree node.
    pub fn text(value: Str) -> Tree {
        json!(value)
    }

    /// Creates a List tree node (non-mergeable).
    pub fn array(items: &[Tree]) -> Tree {
        json!(items)
    }

    /// Creates a Map tree node from a TreeMap.
    pub fn map(entries: TreeMap) -> Tree {
        json!(entries)
    }

    /// Creates a Nil tree node (no input consumed).
    pub fn nil() -> Tree {
        json!(null)
    }

    /// Creates a Bool tree node from a string.
    pub fn bool(value: Str) -> Tree {
        json!(value)
    }

    /// Creates a Number tree node from a string.
    pub fn number(value: Str) -> Tree {
        json!(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_tree() {
        let t = TreeBuild::text("hello".into());
        assert_eq!(t.to_string(), "t(\"hello\")");
    }

    #[test]
    fn list_tree() {
        let t = TreeBuild::seq(&[
            TreeBuild::text("a".into()).into(),
            TreeBuild::text("b".into()).into(),
        ]);
        assert!(matches!(t, TreeBuild::Seq(_)));
    }

    #[test]
    fn named_tree() {
        let t = TreeBuild::named("key".into(), TreeBuild::text("value".into()).into());
        assert!(matches!(t, TreeBuild::Named(_)));
    }

    #[test]
    fn nil_tree() {
        let t = TreeBuild::nil();
        assert_eq!(t.to_string(), "NIL");
    }

    #[test]
    fn bottom_tree() {
        let t = TreeBuild::bottom();
        assert_eq!(t.to_string(), "BOTTOM");
    }
}
