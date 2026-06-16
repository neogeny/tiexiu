// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;
use crate::trees::KeyValue;
use crate::trees::cst::TreeMap;

/// Shorthand for Tree::Text.
pub fn t(value: &str) -> Tree {
    Tree::Text(value.into())
}

/// Shorthand for Tree::Seq.
pub fn s(items: &[Tree]) -> Tree {
    Tree::Seq(items.iter().cloned().map(|t| t.into()).collect())
}

/// Shorthand for Tree::List.
pub fn l(items: &[Tree]) -> Tree {
    Tree::List(items.iter().cloned().map(|t| t.into()).collect())
}

/// Shorthand for Tree::Map from key-value pairs.
pub fn m(entries: &[(&str, Tree)]) -> Tree {
    let mut map: TreeMap = TreeMap::default();
    for (k, v) in entries.iter() {
        map.insert((*k).into(), v.clone().into());
    }
    Tree::Map(map)
}

/// Shorthand for Tree::Named.
pub fn k(key: &str, value: Tree) -> Tree {
    let keyval = KeyValue(key.into(), value.into());
    Tree::Named(keyval)
}

/// Shorthand for Tree::NamedAsList.
pub fn kl(key: &str, value: Tree) -> Tree {
    let keyval = KeyValue(key.into(), value.into());
    Tree::NamedAsList(keyval)
}

/// Shorthand for Tree::Override.
pub fn o(tree: Tree) -> Tree {
    Tree::Override(tree.into())
}

/// Shorthand for Tree::OverrideAsList.
pub fn ol(tree: Tree) -> Tree {
    Tree::OverrideAsList(tree.into())
}

/// Shorthand for Tree::Node.
pub fn n(typename: &str, tree: Tree) -> Tree {
    Tree::Node {
        typename: typename.into(),
        tree: tree.into(),
    }
}

/// Shorthand for Tree::Bottom.
pub fn bottom() -> Tree {
    Tree::Bottom
}

/// Shorthand for Tree::Nil.
pub fn nil() -> Tree {
    Tree::Null
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trees::tree::{BOTTOM, NULL};

    #[test]
    fn text_tree() {
        let t = t("hello");
        assert_eq!(t.to_string(), "t(\"hello\")");
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
        let t = NULL;
        assert_eq!(t.to_string(), "NIL");
    }

    #[test]
    fn bottom_tree() {
        let t = BOTTOM;
        assert_eq!(t.to_string(), "BOTTOM");
    }
}
