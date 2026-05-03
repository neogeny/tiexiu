// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::map::TreeMap;
use super::tree::Tree;
use crate::trees::KeyValue;
use std::rc::Rc;

impl Tree {
    pub fn text(value: &str) -> Tree {
        Self::Text(value.into())
    }

    pub fn seq(items: &[Rc<Tree>]) -> Tree {
        Self::Seq(items.into())
    }

    pub fn list(items: &[Rc<Tree>]) -> Tree {
        Self::List(items.into())
    }

    pub fn map(entries: TreeMap) -> Tree {
        Self::Map(entries.into())
    }

    pub fn named(key: &str, value: Rc<Tree>) -> Tree {
        let keyval = KeyValue(key.into(), value);
        Self::Named(keyval)
    }

    pub fn named_as_list(key: &str, value: Rc<Tree>) -> Tree {
        let keyval = KeyValue(key.into(), value);
        Self::NamedAsList(keyval)
    }

    pub fn override_with(tree: Rc<Tree>) -> Tree {
        Self::Override(tree)
    }

    pub fn override_as_list(tree: Rc<Tree>) -> Tree {
        Self::OverrideAsList(tree)
    }

    pub fn node(typename: &str, tree: Rc<Tree>) -> Tree {
        Self::Node {
            typename: typename.into(),
            tree,
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
        assert_eq!(t.to_string(), "t(\"hello\")");
    }

    #[test]
    fn list_tree() {
        let t = Tree::seq(&[Tree::text("a").into(), Tree::text("b").into()]);
        assert!(matches!(t, Tree::Seq(_)));
    }

    #[test]
    fn named_tree() {
        let t = Tree::named("key", Tree::text("value").into());
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
