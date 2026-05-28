// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::map::TreeMap;
use crate::cfg::types::{Define, Str};
use crate::types::Ref;
use std::collections::LinkedList;

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
    /// A sequence of values (mergeable).
    Seq(Ref<[TreeRef]>),
    /// A non-mergeable list of values.
    List(Ref<[TreeRef]>),
    /// A mapping of named elements.
    Map(Ref<TreeMap>),
    /// The result of parsing a rule call.
    Node { typename: Str, tree: TreeRef },
    /// Parsing that didn't consume any input (internal).
    Nil,
    /// A named element for tree merging (internal).
    Named(KeyValue),
    /// A named element forced into a list (internal).
    NamedAsList(KeyValue),
    /// Override value for merged tree (internal).
    Override(TreeRef),
    /// Override value forced into a list (internal).
    OverrideAsList(TreeRef),
    /// Failure marker used in memoization (internal).
    Bottom,
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
        Tree::Seq(clean.into())
    }
}

impl<const N: usize> From<[Tree; N]> for Tree {
    fn from(arr: [Tree; N]) -> Self {
        let clean: Vec<TreeRef> = arr
            .into_iter()
            .filter(|item| *item != Tree::Nil)
            .map(|t| t.into())
            .collect();
        Tree::Seq(clean.into())
    }
}

impl From<Vec<TreeRef>> for Tree {
    fn from(v: Vec<TreeRef>) -> Self {
        Tree::Seq(v.into())
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
        Tree::Seq(clean.into())
    }
}

impl From<&[TreeRef]> for Tree {
    fn from(slice: &[TreeRef]) -> Self {
        Tree::Seq(slice.into())
    }
}

impl From<TreeList> for Tree {
    fn from(list: TreeList) -> Self {
        Tree::List(
            list.into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice()
                .into(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TreeMerge {
    pub root: Tree,
    pub map: TreeMap,
}

impl TreeMerge {
    /// Creates a new empty `TreeMerge`.
    pub fn new() -> Self {
        Self {
            root: Tree::Nil,
            map: TreeMap::new(),
        }
    }
}

impl Tree {
    /// Folds the tree by resolving Named/Override/Nil into merged Map/Seq form.
    pub fn fold(self) -> Tree {
        let mut gather = TreeMerge::new();
        let tree = self.clean_and_fold(&mut gather);

        if gather.root != Tree::Nil {
            gather.root.closed()
        } else if !gather.map.is_empty() {
            Tree::Map(gather.map.into())
        } else {
            tree.closed()
        }
    }

    pub(crate) fn define(&mut self, names: &[Define]) {
        if let Tree::Map(map) = self {
            let mut newmap = map.as_ref().clone();
            newmap.define(names);
            *map = newmap.into();
        }
    }

    pub(crate) fn closed(self) -> Self {
        match self {
            Tree::Seq(items) => Tree::List(items),
            _ => self,
        }
    }

    pub(crate) fn append(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Seq(list), node) => {
                let mut v: Vec<TreeRef> = list.to_vec();
                v.push(node.into());
                Self::Seq(v.into())
            }
            (s, n) => Self::Seq(vec![s.into(), n.into()].into()),
        }
    }

    pub(crate) fn append_as_list(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => Self::Seq(vec![n.into()].into()),
            (Self::Seq(list), Self::Nil) => Self::Seq(list),
            (Self::Seq(list), n) => {
                let mut v: Vec<TreeRef> = list.to_vec();
                v.push(n.into());
                Self::Seq(v.into())
            }
            (s, n) => Self::Seq(vec![s.into(), n.into()].into()),
        }
    }

    pub(crate) fn merge(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Seq(l1), Self::Seq(l2)) => {
                let mut v: Vec<TreeRef> = l1.to_vec();
                v.extend(l2.iter().cloned());
                Self::Seq(v.into())
            }
            (s, Self::Seq(l2)) => {
                let mut v: Vec<TreeRef> = vec![s.into()];
                v.extend(l2.iter().cloned());
                Self::Seq(v.into())
            }
            (Self::Seq(l1), n) => {
                let mut v: Vec<TreeRef> = l1.to_vec();
                v.push(n.into());
                Self::Seq(v.into())
            }
            (s, n) => Self::Seq(vec![s.into(), n.into()].into()),
        }
    }

    fn clean_and_fold(&self, gather: &mut TreeMerge) -> Tree {
        match self {
            Tree::Seq(elements) => {
                let mut out = Tree::Nil;
                for elem in elements.iter() {
                    out = out.clone().merge(elem.as_ref().clean_and_fold(gather));
                }
                out
            }
            Tree::List(elements) => {
                let clean: Vec<TreeRef> = elements
                    .iter()
                    .map(|s| s.as_ref().clean_and_fold(gather).into())
                    .collect();
                Tree::List(clean.into())
            }
            Tree::Named(keyval) => {
                let KeyValue(name, val) = keyval;
                let clean = val.as_ref().clone().clean_and_fold(gather);
                gather.map.insert(name, clean.clone());
                clean
            }
            Tree::NamedAsList(keyval) => {
                let KeyValue(name, val) = keyval;
                let clean = val.as_ref().clone().clean_and_fold(gather);
                gather.map.insert_as_list(name, clean.clone());
                clean
            }
            Tree::Override(val) => {
                let clean = val.as_ref().clone().clean_and_fold(gather);
                gather.root = gather.root.clone().append(clean.clone());
                clean
            }
            Tree::OverrideAsList(val) => {
                let clean = val.as_ref().clone().clean_and_fold(gather);
                gather.root = gather.root.clone().append_as_list(clean.clone());
                clean
            }
            Tree::Nil => Tree::Nil,
            _ => self.clone(),
        }
    }

    /// Returns the total character width of text nodes in this tree.
    pub fn width(&self) -> usize {
        match self {
            Tree::Text(text) => text.len(),
            Tree::Override(inner) | Tree::OverrideAsList(inner) => inner.width(),
            Tree::Nil | Tree::Bottom => 0,
            Tree::Seq(items) | Tree::List(items) => items.iter().map(|item| item.width()).sum(),
            Tree::Map(map) => map.iter().map(|(_, node)| node.width()).sum(),
            Tree::Named(pair) | Tree::NamedAsList(pair) => {
                let KeyValue(_, val) = pair;
                val.width()
            }
            Tree::Node { typename: _, tree } => tree.width(),
        }
    }

    /// Returns the text value of this tree or a debug representation.
    pub fn value(&self) -> Str {
        match self {
            Tree::Text(text) => text.clone(),
            _ => format!("{:#?}", self).into(),
        }
    }

    /// Returns the child elements if this is a Seq or List, or an empty slice.
    pub fn list_value(&self) -> Ref<[TreeRef]> {
        match self {
            Tree::Seq(items) | Tree::List(items) => items.clone(),
            _ => [].into(),
        }
    }

    /// Returns the child elements as text values, or an empty slice.
    pub fn str_list_value(&self) -> Ref<[Str]> {
        self.list_value().iter().map(|t| t.value()).collect()
    }

    /// Returns the inner TreeMap if this is a Map variant.
    pub fn map_value(&self) -> Option<&TreeMap> {
        match self {
            Tree::Map(map) => Some(map),
            _ => None,
        }
    }

    /// Looks up a key in the Map variant and returns the corresponding tree.
    pub fn get(&self, key: &str) -> Option<&Tree> {
        match self {
            Tree::Map(map) => map.get(key),
            _ => None,
        }
    }

    /// Looks up a key and returns its text value, or an empty string.
    pub fn get_value(&self, key: &str) -> Str {
        self.get(key)
            .map(|n| n.value())
            .unwrap_or_else(|| "".into())
    }

    /// Looks up a key and returns its list children, or an empty slice.
    pub fn get_list(&self, key: &str) -> Ref<[TreeRef]> {
        self.get(key)
            .map(|n| n.list_value().clone())
            .unwrap_or_else(|| [].into())
    }

    /// Looks up a key and returns its children as text values.
    pub fn get_str_list(&self, key: &str) -> Ref<[Str]> {
        self.get_list(key).iter().map(|t| t.value()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: usize = 32;

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
        let result = raw.fold();

        assert_eq!(result, Tree::Bottom.fold());
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Tree::from(vec![Tree::Nil, Tree::Bottom, Tree::Nil]);
        let result = raw.fold();

        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        let raw = Tree::from(vec![Tree::Bottom, Tree::Nil, Tree::Bottom]);
        let result = raw.fold();

        if let Tree::List(v) = result {
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
        let result = raw.fold();

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

        let result = tree.fold();

        assert!(matches!(result, Tree::Map(_)));
        if let Tree::Map(m) = result {
            assert!(m.get("x").is_some(), "key 'x' should be present");
            assert!(m.get("a").is_some(), "key 'a' should be present");
            assert!(m.get("b").is_some(), "key 'b' should be present");
        }
    }
}
