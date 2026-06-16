// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;
use crate::trees::TreeBuild;
use crate::{Define, Str};

/// A reference-counted slice of key-tree entries for TreeMap.
pub type TreeMap = serde_json::Map<Str, Tree>;

#[derive(Debug, Clone, PartialEq)]
struct TreeGather {
    pub root: Tree,
    pub map: TreeMap,
}

impl TreeGather {
    /// Creates a new empty `TreeMerge`.
    pub fn new() -> Self {
        Self {
            root: TreeBuild::nil(),
            map: TreeMap::default(),
        }
    }
}

pub struct TreeFold();

impl TreeFold {
    /// Folds the tree by resolving Named/Override/Nil into merged Map/Seq form.
    pub fn fold(tree: &Tree) -> Tree {
        let mut gather = TreeGather::new();
        let tree = Self::clean_and_fold(tree, &mut gather);

        if !gather.root.is_null()  {
            Self::closed(gather.root)
        } else if !gather.map.is_empty() {
            Tree::Object(gather.map)
        } else {
            Self::closed(tree)
        }
    }

    fn clean_and_fold(tree: Tree, gather: &mut TreeGather) -> Tree {
        match tree {
            Tree::Array(elements) => {
                let clean: Vec<Tree> = elements
                    .iter()
                    .map(|s| Self::clean_and_fold(s, gather).into())
                    .collect();
                Tree::Array(clean)
            }
            Tree::Object(map) => {
                Tree::Object(
                    map
                        .into_iter()
                        .map(
                            |(k, v)|
                                (
                                    k.to_string(),
                                    Self::clean_and_fold(v, gather)
                                )
                        )
                        .collect()
                )
            }
            _ => tree
        }
    }

    pub(crate) fn closed(tree: Tree) -> Tree {
        if Some(items) = TreeBuild::seq_items(tree) {
            TreeBuild::array(items)
        }
        tree
    }

    pub fn safe_key(key: &str) -> String {
        let mut k = key.to_string();
        while Self::is_reserved(k.as_str()) {
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
            (Self::Nil, n) => n.clone(),
            (s, Self::Nil) => s.clone(),
            (Self::Seq(list), node) => {
                let mut v: Vec<Tree> = list.to_vec();
                v.push(node.clone().into());
                Self::Seq(v)
            }
            (s, n) => Self::Seq(vec![s.clone().into(), n.clone().into()]),
        }
    }

    pub(crate) fn append_as_list(tree: &Self, node: &Self) -> Self {
        match (tree, node) {
            (Self::Nil, n) => Self::Seq(vec![n.clone().into()]),
            (Self::Seq(list), Self::Nil) => Self::Seq(list.clone()),
            (Self::Seq(list), n) => {
                let mut v: Vec<Tree> = list.to_vec();
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
                Self::insert_as_list(map, k, &Self::Nil);
            } else {
                Self::insert(map, k, &Self::Nil);
            }
        }
    }

    pub(crate) fn merge(tree: &Self, node: &Self) -> Self {
        match (tree, node) {
            (Self::Nil, n) => n.clone(),
            (s, Self::Nil) => s.clone(),
            (Self::Seq(l1), Self::Seq(l2)) => {
                let mut v: Vec<Tree> = l1.to_vec();
                v.extend(l2.iter().cloned());
                Self::Seq(v)
            }
            (s, Self::Seq(l2)) => {
                let mut v: Vec<Tree> = vec![s.clone().into()];
                v.extend(l2.iter().cloned());
                Self::Seq(v)
            }
            (Self::Seq(l1), n) => {
                let mut v: Vec<Tree> = l1.to_vec();
                v.push(n.clone().into());
                Self::Seq(v)
            }
            (s, n) => Self::Seq(vec![s.clone().into(), n.clone().into()]),
        }
    }
}
