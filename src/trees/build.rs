// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::map::TreeMap;
use super::tree::{FlagMap, KeyValue, NodeMeta, Tree};

impl Tree {
    pub fn text(value: &str) -> Tree {
        Self::Text(value.into())
    }

    pub fn list(items: &[Tree]) -> Tree {
        Self::List(items.into())
    }

    pub fn map(entries: TreeMap) -> Tree {
        Self::Map(entries.into())
    }

    pub fn named(key: &str, value: Tree) -> Tree {
        let keyval = KeyValue(key.into(), value);
        Self::Named(keyval.into())
    }

    pub fn named_as_list(key: &str, value: Tree) -> Tree {
        let keyval = KeyValue(key.into(), value);
        Self::NamedAsList(keyval.into())
    }

    pub fn override_with(tree: Tree) -> Tree {
        Self::Override(tree.into())
    }

    pub fn override_as_list(tree: Tree) -> Tree {
        Self::OverrideAsList(tree.into())
    }

    pub fn node(name: &str, params: &[String], tree: Tree) -> Tree {
        let pi = NodeMeta {
            name: name.into(),
            params: params.iter().map(|p| p.as_str().into()).collect(),
            flags: FlagMap::new(),
        };
        Self::Node {
            meta: pi.into(),
            tree: tree.into(),
        }
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
        let t = Tree::text("hello");
        assert_eq!(t.to_string(), "hello");
    }

    #[test]
    fn list_tree() {
        let t = Tree::list(&[Tree::text("a"), Tree::text("b")]);
        assert!(matches!(t, Tree::List(_)));
    }

    #[test]
    fn named_tree() {
        let t = Tree::named("key", Tree::text("value"));
        assert!(matches!(t, Tree::Named(_)));
    }

    #[test]
    fn nil_tree() {
        let t = Tree::nil();
        assert_eq!(t.to_string(), "∅");
    }

    #[test]
    fn bottom_tree() {
        let t = Tree::bottom();
        assert_eq!(t.to_string(), "⊥");
    }
}
