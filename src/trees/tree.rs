// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::build;
use super::map::TreeMap;
use indexmap::IndexMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue(pub Box<str>, pub Tree);

pub fn keyval(name: &str, tree: Tree) -> KeyValue {
    KeyValue(name.into(), tree.clone())
}

pub type FlagMap = IndexMap<Box<str>, bool>;

#[derive(Debug, Clone, PartialEq)]
pub struct NodeInfo {
    pub name: Box<str>,
    pub params: Box<[Box<str>]>,
    pub flags: FlagMap,
}

pub type NodeInfoRef = Rc<NodeInfo>;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Nil,
    Bottom,

    Void,
    Text(Box<str>),
    List(Box<[Tree]>),

    Named(Box<KeyValue>),
    NamedAsList(Box<KeyValue>),

    Override(Box<Tree>),
    OverrideAsList(Box<Tree>),

    Map(Box<TreeMap>),

    Node { info: NodeInfoRef, tree: Box<Tree> },
}

impl From<Vec<Tree>> for Tree {
    fn from(v: Vec<Tree>) -> Self {
        Tree::List(v.into_boxed_slice())
    }
}

impl<const N: usize> From<[Tree; N]> for Tree {
    fn from(arr: [Tree; N]) -> Self {
        Tree::List(arr.into())
    }
}

impl Tree {
    pub fn append(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::List(list), node) => {
                let mut v = list.into_vec();
                v.push(node);
                Self::List(v.into_boxed_slice())
            }
            (s, n) => Self::List(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn append_as_list(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => Self::List(vec![n].into_boxed_slice()),
            (s, Self::Nil) => s,
            (Self::List(list), n) => {
                let mut v = list.into_vec();
                v.push(n);
                Self::List(v.into_boxed_slice())
            }
            (s, n) => Self::List(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn merge(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::List(l1), Self::List(l2)) => {
                let mut v = l1.into_vec();
                v.extend(l2.into_vec());
                Self::List(v.into_boxed_slice())
            }
            (Self::List(l1), n) => {
                let mut v = l1.into_vec();
                v.push(n);
                Self::List(v.into_boxed_slice())
            }
            (s, Self::List(l2)) => {
                let mut v = vec![s];
                v.extend(l2.into_vec());
                Self::List(v.into_boxed_slice())
            }
            (s, n) => s.append(n),
        }
    }

    pub fn normalized(self) -> Tree {
        let (tags, root, tree) = self._normalize();

        if root != Tree::Nil {
            root
        } else if !tags.is_empty() {
            Tree::Map(tags.into())
        } else {
            tree
        }
    }

    fn _normalize(self) -> (TreeMap, Tree, Tree) {
        let mut tags = TreeMap::new();
        let mut root = Tree::Nil;
        let mut tree = Tree::Nil;

        match self {
            Tree::List(elements) => {
                for node in elements {
                    let (child_tags, child_root, child_cst) = node._normalize();

                    tags.update(&child_tags);
                    root = root.merge(child_root);
                    tree = tree.merge(child_cst);
                }
            }
            Tree::Named(keyval) => {
                let KeyValue(name, val) = keyval.deref();
                tags.insert(name, val.clone());
            }
            Tree::NamedAsList(keyval) => {
                let KeyValue(name, val) = keyval.deref();
                tags.insert_as_list(name, val.clone());
            }
            Tree::Override(val) => root = root.append(*val),
            Tree::OverrideAsList(val) => root = root.append_as_list(*val),
            Tree::Nil => {}
            other => tree = tree.merge(other),
        }

        (tags, root, tree)
    }

    pub fn width(&self) -> usize {
        match self {
            Tree::Text(text) => text.len(),
            Tree::Override(inner) | Tree::OverrideAsList(inner) => inner.width(),
            Tree::Void | Tree::Nil | Tree::Bottom => 0,
            Tree::List(items) => items.iter().map(|item| item.width()).sum(),
            Tree::Map(tags) => tags.entries.values().map(|node| node.width()).sum(),
            Tree::Named(pair) | Tree::NamedAsList(pair) => {
                let KeyValue(_, val) = &**pair;
                val.width()
            }
            Tree::Node { info: _, tree } => tree.width(),
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
    fn test_node_nil_removal() {
        let raw = Tree::List([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.normalized();

        // result = tree_closed(Bottom)
        assert_eq!(result, Tree::Bottom.normalized());
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Tree::List([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.normalized();

        // If tree_closed is identity for non-lists, this is just Bottom
        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        let raw = Tree::List([Tree::Bottom, Tree::Nil, Tree::Bottom].into());
        let result = raw.normalized();

        if let Tree::List(v) = result {
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
        let raw = Tree::List([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.normalized();

        // Since it's effectively Bottom, and Bottom isn't a list,
        // it doesn't become a Closure of len 1. It just stays Bottom.
        assert_eq!(result, Tree::Bottom);
    }
}
