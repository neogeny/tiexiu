// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::map::TreeMap;
use super::tree::{KeyValue, Tree};

pub const NIL: Tree = Tree::Nil;
pub const BOTTOM: Tree = Tree::Bottom;

pub fn t(value: &str) -> Tree {
    Tree::Text(value.into())
}

pub fn s(items: &[Tree]) -> Tree {
    Tree::Seq(items.into())
}

pub fn c(items: &[Tree]) -> Tree {
    Tree::Closed(items.into())
}

pub fn m(entries: &[(&str, Tree)]) -> Tree {
    let mut map = TreeMap::new();
    for (key, value) in entries {
        map.insert(key, value.clone());
    }
    Tree::Map(map.into())
}

pub fn k(key: &str, value: Tree) -> Tree {
    let keyval = KeyValue(key.into(), value.into());
    Tree::Named(keyval)
}

pub fn kl(key: &str, value: Tree) -> Tree {
    let keyval = KeyValue(key.into(), value.into());
    Tree::NamedAsList(keyval)
}

pub fn o(tree: Tree) -> Tree {
    Tree::Override(tree.into())
}

pub fn ol(tree: Tree) -> Tree {
    Tree::OverrideAsList(tree.into())
}

pub fn n(typename: &str, tree: Tree) -> Tree {
    Tree::Node {
        typename: typename.into(),
        tree: tree.into(),
    }
}

pub fn bottom() -> Tree {
    Tree::Bottom
}

pub fn nil() -> Tree {
    Tree::Nil
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_tree() {
        let t = t("hello");
        assert_eq!(t.to_string(), "hello");
    }

    #[test]
    fn list_tree() {
        let t = s(&[t("a"), t("b")]);
        assert!(matches!(t, Tree::Seq(_)));
    }

    #[test]
    fn named_tree() {
        let t = k("key", t("value"));
        assert!(matches!(t, Tree::Named(_)));
    }

    #[test]
    fn nil_tree() {
        let t = NIL;
        assert_eq!(t.to_string(), "∅");
    }

    #[test]
    fn bottom_tree() {
        let t = BOTTOM;
        assert_eq!(t.to_string(), "⊥");
    }
}
