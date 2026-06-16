// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::types::Str;
use crate::trees::cst::*;
use crate::types::Ref;
use std::collections::LinkedList;

/// The Nil tree constant.
pub const NIL: Tree = Tree::Nil;
/// The Bottom tree constant.
pub const BOTTOM: Tree = Tree::Bottom;

/// A reference-counted tree node.
pub type TreeRef = Ref<Tree>;
/// A linked list of tree references.
pub type TreeList = LinkedList<TreeRef>;

/// A key-value pair for named elements in a tree.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue(pub Str, pub TreeRef);

/// The abstract syntax tree representation for parsed input.
#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    /// A text/leaf node from tokens or patterns.
    Text(Str),
    /// A non-mergeable list of values.
    Array(Vec<TreeRef>),
    /// A mapping of named elements.
    Object(TreeMap),

    // Use tod support True, False, Number
    Bool(bool),
    Number(f64),

    // NOTE these variants don't survive fold()
    /// Parsing that didn't consume any input (internal).
    Nil,
    /// The result of parsing a rule call.
    Node { typename: Str, tree: TreeRef },
    /// A sequence of values (mergeable).
    Seq(Vec<TreeRef>),
    /// Failure marker used in memoization (internal).
    Bottom,
    /// A named element for tree merging (internal).
    Named(KeyValue),
    /// A named element forced into a list (internal).
    NamedAsList(KeyValue),
    /// Override value for merged tree (internal).
    Override(TreeRef),
    /// Override value forced into a list (internal).
    OverrideAsList(TreeRef),
}

/// Creates a KeyValue pair from a name and a tree.
pub fn keyval(name: &str, tree: Tree) -> KeyValue {
    KeyValue(name.into(), tree.into())
}

impl From<Vec<Tree>> for Tree {
    fn from(v: Vec<Tree>) -> Self {
        let clean: Vec<TreeRef> = v
            .into_iter()
            .filter(|item| *item != Tree::Nil)
            .map(|t| t.into())
            .collect();
        Tree::Seq(clean)
    }
}

impl<const N: usize> From<[Tree; N]> for Tree {
    fn from(arr: [Tree; N]) -> Self {
        let clean: Vec<TreeRef> = arr
            .into_iter()
            .filter(|item| *item != Tree::Nil)
            .map(|t| t.into())
            .collect();
        Tree::Seq(clean)
    }
}

impl From<Vec<TreeRef>> for Tree {
    fn from(v: Vec<TreeRef>) -> Self {
        Tree::Seq(v)
    }
}

impl From<&[Tree]> for Tree {
    fn from(slice: &[Tree]) -> Self {
        let clean: Vec<TreeRef> = slice
            .iter()
            .filter(|item| **item != Tree::Nil)
            .cloned()
            .map(|t| t.into())
            .collect();
        Tree::Seq(clean)
    }
}

impl From<&[TreeRef]> for Tree {
    fn from(slice: &[TreeRef]) -> Self {
        Tree::Seq(slice.to_vec())
    }
}

impl From<TreeList> for Tree {
    fn from(list: TreeList) -> Self {
        Tree::Array(list.into_iter().collect())
    }
}

impl Tree {
    /// Returns the text value of this tree or a debug representation.
    pub fn value(&self) -> Str {
        match self {
            Tree::Text(text) => text.clone(),
            _ => format!("{:#?}", self),
        }
    }

    /// Returns the child elements if this is a Seq or List, or an empty vec.
    pub fn list_value(&self) -> Vec<TreeRef> {
        match self {
            Tree::Seq(items) | Tree::Array(items) => items.clone(),
            _ => vec![],
        }
    }

    /// Returns the child elements as text values, or an empty vec.
    pub fn str_list_value(&self) -> Vec<Str> {
        self.list_value().iter().map(|t| t.value()).collect()
    }

    /// Returns the inner TreeMap if this is a Map variant.
    pub fn map_value(&self) -> Option<&TreeMap> {
        match self {
            Tree::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Looks up a key in the Map variant and returns the corresponding tree.
    pub fn get(&self, key: &str) -> Option<&Tree> {
        match self {
            Tree::Object(map) => map.get(key).map(|arc| arc.as_ref()),
            _ => None,
        }
    }

    /// Looks up a key and returns its text value, or an empty string.
    pub fn get_value(&self, key: &str) -> Str {
        self.get(key)
            .map(|n| n.value())
            .unwrap_or_else(|| "".into())
    }

    /// Looks up a key and returns its list children, or an empty vec.
    pub fn get_list(&self, key: &str) -> Vec<TreeRef> {
        self.get(key).map(|n| n.list_value()).unwrap_or_default()
    }

    /// Looks up a key and returns its children as text values.
    pub fn get_str_list(&self, key: &str) -> Vec<Str> {
        self.get_list(key).iter().map(|t| t.value()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: usize = 96;

    #[test]
    fn test_tree_size() {
        let size = size_of::<Tree>();
        assert!(size <= TARGET, "Cst size is {} > {} bytes", size, TARGET);
    }
    #[test]
    fn test_keyval_size() {
        let size = size_of::<KeyValue>();
        assert!(size <= TARGET, "KeyVal size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_node_nil_removal() {
        let raw = Tree::from(vec![Tree::Nil, Tree::Bottom, Tree::Nil]);
        let result = Tree::fold(raw);

        assert_eq!(result, Tree::fold(Tree::Bottom));
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Tree::from(vec![Tree::Nil, Tree::Bottom, Tree::Nil]);
        let result = Tree::fold(raw);

        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        let raw = Tree::from(vec![Tree::Bottom, Tree::Nil, Tree::Bottom]);
        let result = Tree::fold(raw);

        if let Tree::Array(v) = result {
            assert_eq!(v.len(), 2);
            assert_eq!(*v[0], Tree::Bottom);
            assert_eq!(*v[1], Tree::Bottom);
        } else {
            panic!("Expected Closure, got {:?}", result);
        }
    }

    #[test]
    fn test_node_nil_purging_preserves_count() {
        let raw = Tree::from(vec![Tree::Nil, Tree::Bottom, Tree::Nil]);
        let result = Tree::fold(raw);

        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_named_group_with_inner_names() {
        let tree = Tree::named(
            "x".into(),
            Tree::Seq(
                [
                    Tree::named("a".into(), Tree::text("a".into()).into()).into(),
                    Tree::named("b".into(), Tree::text("b".into()).into()).into(),
                ]
                .into(),
            )
            .into(),
        );

        let result = Tree::fold(tree);

        assert!(matches!(result, Tree::Object(_)));
        if let Tree::Object(m) = result {
            assert!(m.get("x").is_some(), "key 'x' should be present");
            assert!(m.get("a").is_some(), "key 'a' should be present");
            assert!(m.get("b").is_some(), "key 'b' should be present");
        }
    }
}
