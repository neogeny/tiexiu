// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tags::TreeTags;
use std::ops::Add;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue(pub Box<str>, pub Tree);

pub fn keyval(name: &str, tree: Tree) -> KeyValue {
    KeyValue(name.into(), tree.clone())
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Stump,
    Leaf(Box<str>),      // 16 bytes
    Node(Box<[Tree]>),   // 16 bytes
    Tags(Box<TreeTags>), // 8 bytes

    LeafTag(Box<KeyValue>), // 8 bytes
    NodeTag(Box<KeyValue>), // 8 bytes
    RootLeaf(Box<Tree>),    // 8 bytes
    RootNode(Box<Tree>),    // 8 bytes

    Bottom,
    Nil,
}

impl Add for Tree {
    type Output = Self;

    fn add(self, node: Self) -> Self {
        self.add_leaf(node)
    }
}

impl From<Vec<Tree>> for Tree {
    fn from(v: Vec<Tree>) -> Self {
        Tree::Node(v.into_boxed_slice())
    }
}

impl<const N: usize> From<[Tree; N]> for Tree {
    fn from(arr: [Tree; N]) -> Self {
        Tree::Node(arr.into())
    }
}

impl Tree {
    pub fn named(key: &str, value: Tree) -> Self {
        Tree::LeafTag(Box::new(keyval(key, value)))
    }

    pub fn named_list(key: &str, value: Tree) -> Self {
        Tree::NodeTag(Box::new(keyval(key, value)))
    }

    fn add_leaf(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Node(list), node) => {
                let mut v = list.into_vec();
                v.push(node);
                Self::Node(v.into_boxed_slice())
            }
            (s, n) => Self::Node(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn add_node(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => Self::Node(vec![n].into_boxed_slice()),
            (s, Self::Nil) => s,
            (Self::Node(list), n) => {
                let mut v = list.into_vec();
                v.push(n);
                Self::Node(v.into_boxed_slice())
            }
            (s, n) => Self::Node(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn join_nodes(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Node(l1), Self::Node(l2)) => {
                let mut v = l1.into_vec();
                v.extend(l2.into_vec());
                Self::Node(v.into_boxed_slice())
            }
            (Self::Node(l1), n) => {
                let mut v = l1.into_vec();
                v.push(n);
                Self::Node(v.into_boxed_slice())
            }
            (s, Self::Node(l2)) => {
                let mut v = vec![s];
                v.extend(l2.into_vec());
                Self::Node(v.into_boxed_slice())
            }
            (s, n) => s.add_leaf(n),
        }
    }

    pub fn trimmed(self) -> Tree {
        let (tags, root, tree) = self._trim();

        // Priority Gate: Override > AST > CST
        if root != Tree::Nil {
            root
        } else if !tags.is_empty() {
            Tree::Tags(tags.into())
        } else {
            tree
        }
    }

    fn _trim(self) -> (TreeTags, Tree, Tree) {
        let mut tags = TreeTags::new();
        let mut root = Tree::Nil;
        let mut tree = Tree::Nil;

        match self {
            Tree::Node(elements) => {
                for node in elements {
                    let (child_ast, child_ovr, child_cst) = node._trim();

                    tags.update(&child_ast);
                    root = root.join_nodes(child_ovr);
                    tree = tree.join_nodes(child_cst);
                }
            }
            Tree::LeafTag(keyval) => {
                let KeyValue(name, val) = keyval.deref();
                tags.set(name, val.clone())
            }
            Tree::NodeTag(keyval) => {
                let KeyValue(name, val) = keyval.deref();
                tags.set_list(name, val.clone())
            }
            Tree::RootLeaf(val) => root = root.add_leaf(*val),
            Tree::RootNode(val) => root = root.add_node(*val),
            Tree::Nil => {}
            other => tree = tree.join_nodes(other),
        }

        (tags, root, tree)
    }

    pub fn width(&self) -> usize {
        match self {
            Tree::Leaf(text) => text.len(),
            Tree::RootLeaf(inner) | Tree::RootNode(inner) => inner.width(),
            Tree::Stump | Tree::Nil | Tree::Bottom => 0,
            Tree::Node(items) => items.iter().map(|item| item.width()).sum(),
            Tree::Tags(tags) => tags.tags.values().map(|node| node.width()).sum(),
            Tree::LeafTag(pair) | Tree::NodeTag(pair) => {
                let KeyValue(_, val) = &**pair;
                val.width()
            }
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
        let raw = Tree::Node([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.trimmed();

        // result = tree_closed(Bottom)
        assert_eq!(result, Tree::Bottom.trimmed());
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Tree::Node([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.trimmed();

        // If tree_closed is identity for non-lists, this is just Bottom
        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        let raw = Tree::Node([Tree::Bottom, Tree::Nil, Tree::Bottom].into());
        let result = raw.trimmed();

        if let Tree::Node(v) = result {
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
        let raw = Tree::Node([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.trimmed();

        // Since it's effectively Bottom, and Bottom isn't a list,
        // it doesn't become a Closure of len 1. It just stays Bottom.
        assert_eq!(result, Tree::Bottom);
    }
}
