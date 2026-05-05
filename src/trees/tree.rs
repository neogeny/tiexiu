// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::map::TreeMap;
use crate::cfg::types::{Define, Str};
use std::rc::Rc;

pub type TreeRef = Rc<Tree>;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue(pub Str, pub Rc<Tree>);

#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Text(Str),            // Tokens or patterns
    Seq(Rc<[Rc<Tree>]>),  // Sequences of values
    List(Rc<[Rc<Tree>]>), // Non-mergeable list of values
    Map(Rc<TreeMap>),     // A mapping of named elements

    Node {
        // The result of parsing a rule call
        typename: Str,
        tree: Rc<Tree>,
    },

    // INTERNAL
    // The folowing variants do not appear in merged trees
    Nil,                      // Parsing that didn't consume any input
    Named(KeyValue),          // Named elements add to the merged TreeMap
    NamedAsList(KeyValue),    // Named elements forced into a list
    Override(Rc<Tree>),       // Adds value to the merged tree
    OverrideAsList(Rc<Tree>), // Adds value to the merged tree, forces list

    Bottom, // The marker for failure used in memoization
}

pub fn keyval(name: &str, tree: Tree) -> KeyValue {
    KeyValue(name.into(), tree.into())
}

impl From<Vec<Tree>> for Tree {
    fn from(v: Vec<Tree>) -> Self {
        let clean: Vec<Rc<Tree>> = v
            .into_iter()
            .filter(|item| *item != Tree::Nil)
            .map(|t| t.into())
            .collect();
        Tree::Seq(clean.into())
    }
}

impl<const N: usize> From<[Tree; N]> for Tree {
    fn from(arr: [Tree; N]) -> Self {
        let clean: Vec<Rc<Tree>> = arr
            .into_iter()
            .filter(|item| *item != Tree::Nil)
            .map(|t| t.into())
            .collect();
        Tree::Seq(clean.into())
    }
}

impl From<Vec<Rc<Tree>>> for Tree {
    fn from(v: Vec<Rc<Tree>>) -> Self {
        Tree::Seq(v.into())
    }
}

impl From<&[Tree]> for Tree {
    fn from(slice: &[Tree]) -> Self {
        let clean: Vec<Rc<Tree>> = slice
            .iter()
            .filter(|item| **item != Tree::Nil)
            .cloned()
            .map(|t| t.into())
            .collect();
        Tree::Seq(clean.into())
    }
}

impl From<&[Rc<Tree>]> for Tree {
    fn from(slice: &[Rc<Tree>]) -> Self {
        Tree::Seq(slice.into())
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

    pub fn define(&mut self, names: &[Define]) {
        if let Tree::Map(map) = self {
            let mut newmap = map.as_ref().clone();
            newmap.define(names);
            *map = newmap.into();
        }
    }

    pub fn closed(self) -> Self {
        match self {
            Tree::Seq(items) => Tree::List(items),
            _ => self,
        }
    }

    pub fn append(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Seq(list), node) => {
                let mut v: Vec<Rc<Tree>> = list.to_vec();
                v.push(node.into());
                Self::Seq(v.into())
            }
            (s, n) => Self::Seq(vec![s.into(), n.into()].into()),
        }
    }

    pub fn append_as_list(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => Self::Seq(vec![n.into()].into()),
            (s, Self::Nil) => s,
            (Self::Seq(list), n) => {
                let mut v: Vec<Rc<Tree>> = list.to_vec();
                v.push(n.into());
                Self::Seq(v.into())
            }
            (s, n) => Self::Seq(vec![s.into(), n.into()].into()),
        }
    }

    pub fn merge(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Seq(l1), Self::Seq(l2)) => {
                let mut v: Vec<Rc<Tree>> = l1.to_vec();
                v.extend(l2.iter().cloned());
                Self::Seq(v.into())
            }
            (Self::Seq(l1), n) => {
                let mut v: Vec<Rc<Tree>> = l1.to_vec();
                v.push(n.into());
                Self::Seq(v.into())
            }
            (s, Self::Seq(l2)) => {
                let mut v: Vec<Rc<Tree>> = vec![s.into()];
                v.extend(l2.iter().cloned());
                Self::Seq(v.into())
            }
            (s, n) => s.append(n),
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
                let clean: Vec<Rc<Tree>> = elements
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

    pub fn value(&self) -> Str {
        match self {
            Tree::Text(text) => text.clone(),
            _ => format!("{:#?}", self).into(),
        }
    }

    pub fn list_value(&self) -> Rc<[Rc<Tree>]> {
        match self {
            Tree::Seq(items) | Tree::List(items) => items.clone(),
            _ => [].into(),
        }
    }

    pub fn str_list_value(&self) -> Rc<[Str]> {
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

    pub fn get_value(&self, key: &str) -> Str {
        self.get(key)
            .map(|n| n.value())
            .unwrap_or_else(|| "".into())
    }

    pub fn get_list(&self, key: &str) -> Rc<[Rc<Tree>]> {
        self.get(key)
            .map(|n| n.list_value().clone())
            .unwrap_or_else(|| [].into())
    }

    pub fn get_str_list(&self, key: &str) -> Rc<[Str]> {
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
            "x",
            Tree::Seq(
                [
                    Tree::named("a", Tree::text("a").into()).into(),
                    Tree::named("b", Tree::text("b").into()).into(),
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
