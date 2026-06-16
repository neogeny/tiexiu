// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::{Tree, TreeRef};
use crate::Ref;
use crate::trees::KeyValue;
use crate::{Define, FastIndexMap, Str};

/// A reference-counted slice of key-tree entries for TreeMap.
pub type TreeMap = FastIndexMap<Str, TreeRef>;
pub type TreeMapRef = Ref<TreeMap>;

#[derive(Debug, Clone, PartialEq)]
struct TreeGather {
    pub root: Tree,
    pub map: TreeMap,
}

impl TreeGather {
    /// Creates a new empty `TreeMerge`.
    pub fn new() -> Self {
        Self {
            root: Tree::Null,
            map: TreeMap::default(),
        }
    }
}

impl Tree {
    /// Folds the tree by resolving Named/Override/Nil into merged Map/Seq form.
    pub fn fold(tree: Self) -> Self {
        let mut gather = TreeGather::new();
        let tree = Self::clean_and_fold(&tree, &mut gather);

        if gather.root != Tree::Null {
            Self::closed(gather.root)
        } else if !gather.map.is_empty() {
            Tree::Map(gather.map)
        } else {
            Self::closed(tree)
        }
    }

    fn clean_and_fold(tree: &Self, gather: &mut TreeGather) -> Tree {
        match tree {
            Tree::Seq(elements) => {
                let mut out = Tree::Null;
                for elem in elements.iter() {
                    out = Self::merge(&out, &Self::clean_and_fold(elem, gather));
                }
                out
            }
            Tree::List(elements) => {
                let clean: Vec<TreeRef> = elements
                    .iter()
                    .map(|s| Self::clean_and_fold(s, gather).into())
                    .collect();
                Tree::List(clean)
            }
            Tree::Named(keyval) => {
                let KeyValue(name, val) = keyval;
                let clean = Self::clean_and_fold(val, gather);
                Self::insert(&mut gather.map, name, &clean);
                clean
            }
            Tree::NamedAsList(keyval) => {
                let KeyValue(name, val) = keyval;
                let clean = Self::clean_and_fold(val, gather);
                Self::insert_as_list(&mut gather.map, name, &clean);
                clean
            }
            Tree::Override(val) => {
                let clean = Self::clean_and_fold(val, gather);
                gather.root = Self::append(&gather.root, &clean);
                clean
            }
            Tree::OverrideAsList(val) => {
                let clean = Self::clean_and_fold(val, gather);
                gather.root = Self::append_as_list(&gather.root, &clean);
                clean
            }
            Tree::Null => Tree::Null,
            _ => tree.clone(),
        }
    }

    pub(crate) fn closed(tree: Self) -> Self {
        match tree {
            Tree::Seq(items) => Tree::List(items),
            _ => tree,
        }
    }

    /// Creates a Named tree node for key-value merging.
    pub fn named(key: Str, value: TreeRef) -> Self {
        let keyval = KeyValue(key, value);
        Self::Named(keyval)
    }

    /// Creates a NamedAsList tree node forcing list merging.
    pub fn named_as_list(key: Str, value: TreeRef) -> Self {
        let keyval = KeyValue(key, value);
        Self::NamedAsList(keyval)
    }

    /// Creates an Override tree node for root merging.
    pub fn override_with(tree: TreeRef) -> Self {
        Self::Override(tree)
    }

    /// Creates an OverrideAsList tree node for list root merging.
    pub fn override_as_list(tree: TreeRef) -> Self {
        Self::OverrideAsList(tree)
    }

    /// Creates a Node tree node from a rule call result.
    pub fn node(typename: Str, tree: TreeRef) -> Self {
        Self::Node { typename, tree }
    }

    pub fn safe_key(key: &str) -> String {
        let mut k = key.to_string();
        while Self::is_reserved(&k) {
            k.push('_');
        }
        k
    }

    pub fn is_reserved(key: &str) -> bool {
        matches!(
            key,
            "items" | "keys" | "values" | "get" | "update" | "pop" | "clear"
        )
    }

    pub(crate) fn append(tree: &Self, node: &Self) -> Self {
        match (tree, node) {
            (Self::Null, n) => n.clone(),
            (s, Self::Null) => s.clone(),
            (Self::Seq(list), node) => {
                let mut v: Vec<TreeRef> = list.to_vec();
                v.push(node.clone().into());
                Self::Seq(v)
            }
            (s, n) => Self::Seq(vec![s.clone().into(), n.clone().into()]),
        }
    }

    pub(crate) fn append_as_list(tree: &Self, node: &Self) -> Self {
        match (tree, node) {
            (Self::Null, n) => Self::Seq(vec![n.clone().into()]),
            (Self::Seq(list), Self::Null) => Self::Seq(list.clone()),
            (Self::Seq(list), n) => {
                let mut v: Vec<TreeRef> = list.to_vec();
                v.push(n.clone().into());
                Self::Seq(v)
            }
            (s, n) => Self::Seq(vec![s.clone().into(), n.clone().into()]),
        }
    }

    pub fn insert(map: &mut TreeMap, key: &str, item: &Self) {
        let key: String = Self::safe_key(key);

        let new_item = if let Some(old_item) = map.get(key.as_str()) {
            Self::append(old_item.as_ref(), item)
        } else {
            item.clone()
        };
        map.insert(key, new_item.into());
    }

    pub fn insert_as_list(map: &mut TreeMap, key: &str, item: &Self) {
        let key: String = Self::safe_key(key);

        let new_item = if let Some(old_item) = map.get(key.as_str()) {
            Self::append_as_list(old_item.as_ref(), item)
        } else {
            Self::Seq(vec![item.clone().into()])
        };
        map.insert(key, new_item.into());
    }

    /// Ensures that the given definition keys exist in the map, inserting defaults if missing.
    pub fn define(map: &mut TreeMap, keys: &[Define]) {
        for (k, aslist) in keys {
            if map.get(k.as_str()).is_some() {
                continue;
            }
            if *aslist {
                Self::insert_as_list(map, k, &Self::Null);
            } else {
                Self::insert(map, k, &Self::Null);
            }
        }
    }

    pub(crate) fn merge(tree: &Self, node: &Self) -> Self {
        match (tree, node) {
            (Self::Null, n) => n.clone(),
            (s, Self::Null) => s.clone(),
            (Self::Seq(l1), Self::Seq(l2)) => {
                let mut v: Vec<TreeRef> = l1.to_vec();
                v.extend(l2.iter().cloned());
                Self::Seq(v)
            }
            (s, Self::Seq(l2)) => {
                let mut v: Vec<TreeRef> = vec![s.clone().into()];
                v.extend(l2.iter().cloned());
                Self::Seq(v)
            }
            (Self::Seq(l1), n) => {
                let mut v: Vec<TreeRef> = l1.to_vec();
                v.push(n.clone().into());
                Self::Seq(v)
            }
            (s, n) => Self::Seq(vec![s.clone().into(), n.clone().into()]),
        }
    }
}
