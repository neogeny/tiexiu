// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.map

use super::tree::{Tree, TreeRef};
use crate::Ref;
use crate::{Define, FastIndexMap, Str};

/// A reference-counted slice of key-tree entries for TreeMap.
pub type TreeMap = FastIndexMap<Str, Tree>;
pub type TreeMapRef= Ref<TreeMap>;

fn safe_key(key: &str) -> String {
    let mut k = key.to_string();
    while is_reserved(&k) {
        k.push('_');
    }
    k
}

fn is_reserved(key: &str) -> bool {
    matches!(
            key,
            "items" | "keys" | "values" | "get" | "update" | "pop" | "clear"
        )
}

pub(crate) fn append(tree: &Tree, node: &Tree) -> Tree {
    match (tree, node) {
        (Tree::Null, n) => *n,
        (s, Tree::Null) => s.clone(),
        (Tree::Seq(list), node) => {
            let mut v: Vec<TreeRef> = list.to_vec();
            v.push(node.into());
            Tree::Seq(v.into())
        }
        (s, n) => Tree::Seq(vec![s.into(), n.into()].into()),
    }
}


pub(crate) fn append_as_list(tree: &Tree, node: &Tree) -> Tree {
    match (tree, node) {
        (Tree::Null, n) => Tree::Seq(vec![n.into()].into()),
        (Tree::Seq(list), Tree::Null) => Tree::Seq(list.clone()),
        (Tree::Seq(list), n) => {
            let mut v: Vec<TreeRef> = list.to_vec();
            v.push(n.into());
            Tree::Seq(v.into())
        }
        (s, n) => Tree::Seq(vec![s.into(), n.into()].into()),
    }
}

pub fn insert(map: &mut TreeMap, key: &str, item: &Tree) {
    let key = safe_key(key);
    map.into();

    let new_item = if let Some(old_item) = map.get(key.as_ref()) {
        append(old_item, item)
    } else {
        item.into()
    };
    map.insert(key.into(), new_item.into());
}

pub fn insert_as_list(map: &mut TreeMap, key: &str, item: &Tree) {
    let key = safe_key(key);

    let new_item =if let Some(old_item) = map.get(key.as_ref()) {
        append_as_list(old_item, item)
    }
    else {
        Tree::Seq([item].into()).into()
    };
    map.insert(key.into(), new_item.into());
}

/// Ensures that the given definition keys exist in the map, inserting defaults if missing.
pub fn define(map: &mut TreeMap, keys: &[Define]) {
    for (k, aslist) in keys {
        if map.get(&k).is_some() {
            continue
        }
        if aslist {
            insert_as_list(map, k, &Tree::Null)
        }
        else {
            insert(map, k, &Tree::Null)
        }
    }
}
