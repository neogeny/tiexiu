// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::map::TreeMap;
use indexmap::{IndexMap, IndexSet};

pub type Str = Box<str>;
pub type FlagMap = IndexMap<Str, bool>;
pub type Define = (Str, bool);
pub type StrSet = IndexSet<Str>;
pub type DefineSet = IndexSet<Define>;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue(pub Str, pub Box<Tree>);

pub fn keyval(name: &str, tree: Tree) -> KeyValue {
    KeyValue(name.into(), tree.into())
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Text(Box<str>),      // Tokens or patterns
    Seq(Box<[Tree]>),    // Sequences of values
    Closed(Box<[Tree]>), // Non-mergeable list of values
    Map(Box<TreeMap>),   // A mapping of named elements

    Node {
        // The result of parsing a rule call
        typename: Box<str>,
        tree: Box<Tree>, // The result of parsing a rule
    },

    // INTERNAL
    // The folowing variants do not appear in final trees
    Nil,                   // Parsing that doesn't consume any input
    Named(KeyValue),       // Named elements
    NamedAsList(KeyValue), // Named elements forced into a list
    Override(Box<Tree>),   // Sets the value of the whole expression
    OverrideAsList(Box<Tree>),

    Bottom, // The marker for failure used in memoization
}

impl From<Vec<Tree>> for Tree {
    fn from(v: Vec<Tree>) -> Self {
        let clean: Vec<Tree> = v
            .into_iter()
            .filter(|item| !matches!(item, Tree::Nil))
            .collect();
        Tree::Seq(clean.into_boxed_slice())
    }
}

impl<const N: usize> From<[Tree; N]> for Tree {
    fn from(arr: [Tree; N]) -> Self {
        let clean: Vec<Tree> = arr
            .into_iter()
            .filter(|item| !matches!(item, Tree::Nil))
            .collect();
        Tree::Seq(clean.into_boxed_slice())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TreeMerge {
    pub root: Tree,
    pub map: TreeMap,
}

impl TreeMerge {
    pub fn new() -> Self {
        Self {
            root: Tree::Nil,
            map: TreeMap::new(),
        }
    }
}

impl Tree {
    pub fn define(&mut self, names: &[Define]) {
        if let Tree::Map(map) = self {
            map.define(names);
        }
    }

    pub fn value(&self) -> Box<str> {
        match self {
            Tree::Text(text) => text.clone(),
            _ => format!("{:#?}", self).into(),
        }
    }

    pub fn list_value(&self) -> Box<[Tree]> {
        match self {
            Tree::Seq(items) | Tree::Closed(items) => items.clone(),
            _ => [].into(),
        }
    }

    pub fn str_list_value(&self) -> Box<[Box<str>]> {
        self.list_value().iter().map(|t| t.value()).collect()
    }

    pub fn map_value(&self) -> Option<&TreeMap> {
        match self {
            Tree::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Tree> {
        match self {
            Tree::Map(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_value(&self, key: &str) -> Box<str> {
        self.get(key)
            .map(|n| n.value())
            .unwrap_or_else(|| "".into())
    }

    pub fn get_list(&self, key: &str) -> Box<[Tree]> {
        self.get(key)
            .map(|n| n.list_value().clone())
            .unwrap_or_else(|| [].into())
    }

    pub fn get_str_list(&self, key: &str) -> Box<[Box<str>]> {
        self.get_list(key).iter().map(|t| t.value()).collect()
    }

    pub fn closed(self) -> Self {
        match self {
            Tree::Seq(items) => Tree::Closed(items),
            _ => self,
        }
    }

    pub fn append(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Seq(list), node) => {
                let mut v = list.into_vec();
                v.push(node);
                Self::Seq(v.into_boxed_slice())
            }
            (s, n) => Self::Seq(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn append_as_list(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => Self::Seq(vec![n].into_boxed_slice()),
            (s, Self::Nil) => s,
            (Self::Seq(list), n) => {
                let mut v = list.into_vec();
                v.push(n);
                Self::Seq(v.into_boxed_slice())
            }
            (s, n) => Self::Seq(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn merge(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Seq(l1), Self::Seq(l2)) => {
                let mut v = l1.into_vec();
                v.extend(l2.into_vec());
                Self::Seq(v.into_boxed_slice())
            }
            (Self::Seq(l1), n) => {
                let mut v = l1.into_vec();
                v.push(n);
                Self::Seq(v.into_boxed_slice())
            }
            (s, Self::Seq(l2)) => {
                let mut v = vec![s];
                v.extend(l2.into_vec());
                Self::Seq(v.into_boxed_slice())
            }
            (s, n) => s.append(n),
        }
    }

    pub fn into_node_tree(self) -> Tree {
        let mut gather = TreeMerge::new();
        let tree = self.clean_and_merge(&mut gather);

        if gather.root != Tree::Nil {
            gather.root
        } else if !gather.map.is_empty() {
            Tree::Map(gather.map.into())
        } else {
            tree
        }
    }

    fn clean_and_merge(&self, gather: &mut TreeMerge) -> Tree {
        match self {
            Tree::Seq(elements) => {
                // NOTE:
                //  Tree::List is the product of Exp::Sequence and the
                //  right semantics are to merge the values one by one
                //  Later, Tree::List should be renamed to Tree::Sequence
                //  to make it clear

                let mut out = Tree::Nil;
                elements
                    .into_iter()
                    .for_each(|s| out = out.clone().append(s.clean_and_merge(gather)));
                out
            }
            Tree::Closed(elements) => {
                // NOTE:
                //  Tree::Closed is the product of the Exp closure node kinds.
                //  The current semantics inherited from TatSu are to keep them
                //  intact, with no merging
                let clean: Vec<Tree> = elements
                    .into_iter()
                    .map(|s| s.clean_and_merge(gather))
                    .collect();
                Tree::Closed(clean.into())
            }
            Tree::Named(keyval) => {
                let KeyValue(name, val) = keyval;
                let clean = val.clone().clean_and_merge(gather);
                gather.map.insert(name, clean.clone());
                clean
            }
            Tree::NamedAsList(keyval) => {
                let KeyValue(name, val) = keyval;
                let clean = val.clean_and_merge(gather);
                gather.map.insert_as_list(name, clean.clone());
                clean
            }
            Tree::Override(val) => {
                let clean = val.clean_and_merge(gather);
                gather.root = gather.root.clone().append(clean.clone());
                clean
            }
            Tree::OverrideAsList(val) => {
                let clean = val.clean_and_merge(gather);
                gather.root = gather.root.clone().append_as_list(clean.clone());
                clean
            }
            Tree::Nil => Tree::Nil,
            _ => self.clone(),
        }
    }

    pub fn width(&self) -> usize {
        match self {
            Tree::Text(text) => text.len(),
            Tree::Override(inner) | Tree::OverrideAsList(inner) => inner.width(),
            Tree::Nil | Tree::Bottom => 0,
            Tree::Seq(items) | Tree::Closed(items) => items.iter().map(|item| item.width()).sum(),
            Tree::Map(map) => map.entries.values().map(|node| node.width()).sum(),
            Tree::Named(pair) | Tree::NamedAsList(pair) => {
                let KeyValue(_, val) = pair;
                val.width()
            }
            Tree::Node { typename: _, tree } => tree.width(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: usize = 32;

    #[test]
    fn test_tree_size() {
        let size = size_of::<Tree>();
        // 24 bytes: Box (8) + Rc (8) + bool/padding (8)
        assert!(size <= TARGET, "Cst size is {} > {} bytes", size, TARGET);
    }
    #[test]
    fn test_keyval_size() {
        let size = size_of::<KeyValue>();
        assert!(size <= TARGET, "KeyVal size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_node_nil_removal() {
        let raw = Tree::Seq([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.into_node_tree();

        // result = tree_closed(Bottom)
        assert_eq!(result, Tree::Bottom.into_node_tree());
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Tree::Seq([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.into_node_tree();

        // If tree_closed is identity for non-lists, this is just Bottom
        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        let raw = Tree::Seq([Tree::Bottom, Tree::Nil, Tree::Bottom].into());
        let result = raw.into_node_tree(); // normalize doesn't close

        if let Tree::Seq(v) = result {
            assert_eq!(v.len(), 2); // Nil is gone, only the two Bottoms remain
            assert_eq!(v[0], Tree::Bottom);
            assert_eq!(v[1], Tree::Bottom);
        } else {
            panic!("Expected Closure, got {:?}", result);
        }
    }

    #[test]
    fn test_node_nil_purging_preserves_count() {
        // Input: List([Nil, Bottom, Nil])
        let raw = Tree::Seq([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.into_node_tree();

        // Since it's effectively Bottom, and Bottom isn't a list,
        // it doesn't become a Closure of len 1. It just stays Bottom.
        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_named_group_with_inner_names() {
        // r: x=(a='a' b='b') - nested named expressions
        // After normalization, all of a, b, and x should be present
        let tree = Tree::named(
            "x",
            Tree::Seq(
                [
                    Tree::named("a", Tree::text("a")),
                    Tree::named("b", Tree::text("b")),
                ]
                .into(),
            ),
        );

        let result = tree.into_node_tree();

        // Result should be a Map containing "a", "b", and "x"
        assert!(matches!(result, Tree::Map(_)));
        if let Tree::Map(m) = result {
            assert!(m.get("x").is_some(), "key 'x' should be present");
            assert!(m.get("a").is_some(), "key 'a' should be present");
            assert!(m.get("b").is_some(), "key 'b' should be present");
        }
    }
}
