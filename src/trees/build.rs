// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::cst::TreeMap;
use super::tree::Tree;
use crate::trees::TreeRef;
use crate::types::Str;

impl Tree {
    /// Creates a Text tree node.
    pub fn text(value: Str) -> Tree {
        Self::Text(value)
    }

    /// Creates a Seq tree node.
    pub fn seq(items: &[TreeRef]) -> Tree {
        Self::Seq(items.to_vec())
    }

    /// Creates a List tree node (non-mergeable).
    pub fn list(items: &[TreeRef]) -> Tree {
        Self::List(items.to_vec())
    }

    /// Creates a Map tree node from a TreeMap.
    pub fn map(entries: TreeMap) -> Tree {
        Self::Map(entries)
    }

    /// Creates a Bottom tree node (memoization failure marker).
    pub fn bottom() -> Tree {
        Self::Bottom
    }

    /// Creates a Nil tree node (no input consumed).
    pub fn nil() -> Tree {
        Self::Nil
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_tree() {
        let t = Tree::text("hello".into());
        assert_eq!(t.to_string(), "t(\"hello\")");
    }

    #[test]
    fn list_tree() {
        let t = Tree::seq(&[Tree::text("a".into()).into(), Tree::text("b".into()).into()]);
        assert!(matches!(t, Tree::Seq(_)));
    }

    #[test]
    fn named_tree() {
        let t = Tree::named("key".into(), Tree::text("value".into()).into());
        assert!(matches!(t, Tree::Named(_)));
    }

    #[test]
    fn nil_tree() {
        let t = Tree::nil();
        assert_eq!(t.to_string(), "NIL");
    }

    #[test]
    fn bottom_tree() {
        let t = Tree::bottom();
        assert_eq!(t.to_string(), "BOTTOM");
    }
}
