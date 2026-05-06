// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::map::TreeMap;
use super::tree::Tree;
use crate::trees::{KeyValue, TreeRef};
use crate::types::Str;

impl Tree {
    pub fn text(value: Str) -> Tree {
        Self::Text(value)
    }

    pub fn seq(items: &[TreeRef]) -> Tree {
        Self::Seq(items.into())
    }

    pub fn list(items: &[TreeRef]) -> Tree {
        Self::List(items.into())
    }

    pub fn map(entries: TreeMap) -> Tree {
        Self::Map(entries.into())
    }

    pub fn named(key: Str, value: TreeRef) -> Tree {
        let keyval = KeyValue(key, value);
        Self::Named(keyval)
    }

    pub fn named_as_list(key: Str, value: TreeRef) -> Tree {
        let keyval = KeyValue(key, value);
        Self::NamedAsList(keyval)
    }

    pub fn override_with(tree: TreeRef) -> Tree {
        Self::Override(tree)
    }

    pub fn override_as_list(tree: TreeRef) -> Tree {
        Self::OverrideAsList(tree)
    }

    pub fn node(typename: Str, tree: TreeRef) -> Tree {
        Self::Node { typename, tree }
    }

    pub fn bottom() -> Tree {
        Self::Bottom
    }

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
