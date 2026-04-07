// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::build;
use super::tags::TreeTags;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue(pub Box<str>, pub Tree);

pub fn keyval(name: &str, tree: Tree) -> KeyValue {
    KeyValue(name.into(), tree.clone())
}

#[derive(Debug, Clone, PartialEq)]
pub struct PruneInfo {
    pub name: String,
    pub params: Box<[String]>,
}

pub type PruneInfoRef = Rc<PruneInfo>;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Nil,
    Bottom,

    Stump,
    Leaf(Box<str>),
    Branches(Box<[Tree]>),

    Tag(Box<KeyValue>),
    BranchingTag(Box<KeyValue>),

    Root(Box<Tree>),
    BranchingRoot(Box<Tree>),

    TreeTags(Box<TreeTags>),

    Pruned(PruneInfoRef, Box<Tree>),
}

impl From<Vec<Tree>> for Tree {
    fn from(v: Vec<Tree>) -> Self {
        Tree::Branches(v.into_boxed_slice())
    }
}

impl<const N: usize> From<[Tree; N]> for Tree {
    fn from(arr: [Tree; N]) -> Self {
        Tree::Branches(arr.into())
    }
}

impl Tree {
    pub fn named(key: &str, value: Tree) -> Self {
        Tree::tag(key, value)
    }

    pub fn named_list(key: &str, value: Tree) -> Self {
        Tree::branching_tag(key, value)
    }

    pub fn add_leaf(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Branches(list), node) => {
                let mut v = list.into_vec();
                v.push(node);
                Self::Branches(v.into_boxed_slice())
            }
            (s, n) => Self::Branches(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn add_branching(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => Self::Branches(vec![n].into_boxed_slice()),
            (s, Self::Nil) => s,
            (Self::Branches(list), n) => {
                let mut v = list.into_vec();
                v.push(n);
                Self::Branches(v.into_boxed_slice())
            }
            (s, n) => Self::Branches(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn join_nodes(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::Branches(l1), Self::Branches(l2)) => {
                let mut v = l1.into_vec();
                v.extend(l2.into_vec());
                Self::Branches(v.into_boxed_slice())
            }
            (Self::Branches(l1), n) => {
                let mut v = l1.into_vec();
                v.push(n);
                Self::Branches(v.into_boxed_slice())
            }
            (s, Self::Branches(l2)) => {
                let mut v = vec![s];
                v.extend(l2.into_vec());
                Self::Branches(v.into_boxed_slice())
            }
            (s, n) => s.add_leaf(n),
        }
    }

    pub fn trimmed(self) -> Tree {
        let (tags, root, tree) = self._trim();

        if root != Tree::Nil {
            root
        } else if !tags.is_empty() {
            Tree::TreeTags(tags.into())
        } else {
            tree
        }
    }

    fn _trim(self) -> (TreeTags, Tree, Tree) {
        let mut tags = TreeTags::new();
        let mut root = Tree::Nil;
        let mut tree = Tree::Nil;

        match self {
            Tree::Branches(elements) => {
                for node in elements {
                    let (child_tags, child_root, child_cst) = node._trim();

                    tags.update(&child_tags);
                    root = root.join_nodes(child_root);
                    tree = tree.join_nodes(child_cst);
                }
            }
            Tree::Tag(keyval) => {
                let KeyValue(name, val) = keyval.deref();
                tags.set(name, val.clone());
            }
            Tree::BranchingTag(keyval) => {
                let KeyValue(name, val) = keyval.deref();
                tags.set_list(name, val.clone());
            }
            Tree::Root(val) => root = root.add_leaf(*val),
            Tree::BranchingRoot(val) => root = root.add_branching(*val),
            Tree::Nil => {}
            other => tree = tree.join_nodes(other),
        }

        (tags, root, tree)
    }

    pub fn width(&self) -> usize {
        match self {
            Tree::Leaf(text) => text.len(),
            Tree::Root(inner) | Tree::BranchingRoot(inner) => inner.width(),
            Tree::Stump | Tree::Nil | Tree::Bottom => 0,
            Tree::Branches(items) => items.iter().map(|item| item.width()).sum(),
            Tree::TreeTags(tags) => tags.tags.values().map(|node| node.width()).sum(),
            Tree::Tag(pair) | Tree::BranchingTag(pair) => {
                let KeyValue(_, val) = &**pair;
                val.width()
            }
            Tree::Pruned(_info, tree) => tree.width(),
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
        let raw = Tree::Branches([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.trimmed();

        // result = tree_closed(Bottom)
        assert_eq!(result, Tree::Bottom.trimmed());
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Tree::Branches([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.trimmed();

        // If tree_closed is identity for non-lists, this is just Bottom
        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        let raw = Tree::Branches([Tree::Bottom, Tree::Nil, Tree::Bottom].into());
        let result = raw.trimmed();

        if let Tree::Branches(v) = result {
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
        let raw = Tree::Branches([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.trimmed();

        // Since it's effectively Bottom, and Bottom isn't a list,
        // it doesn't become a Closure of len 1. It just stays Bottom.
        assert_eq!(result, Tree::Bottom);
    }
}
