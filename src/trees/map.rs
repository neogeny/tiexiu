// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;
use indexmap::IndexMap;

pub type MapEntries = IndexMap<Box<str>, Tree>;

/// A structured mapping for AST nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeMap {
    pub entries: MapEntries,
}

impl TreeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Tree> {
        self.entries.get(key)
    }

    pub fn update(&mut self, other: &TreeMap) {
        for (key, value) in &other.entries {
            self.entries.insert(key.clone(), value.clone());
        }
    }

    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safe_key(k);
            self.entries.entry(key).or_insert(Tree::Nil);
        }

        for &k in list_keys {
            let key = self.safe_key(k);
            self.entries.entry(key).or_insert(Tree::List([].into()));
        }
    }

    pub fn insert(&mut self, key: &str, item: Tree) {
        let key = self.safe_key(key);
        let mut new = item;
        if let Some(current) = self.entries.get(&key) {
            new = current.clone().append(new);
        }
        self.entries.insert(key, new);
    }

    pub fn insert_as_list(&mut self, key: &str, item: Tree) {
        let key = self.safe_key(key);
        let mut new = item;
        if let Some(current) = self.entries.get(&key) {
            new = current.clone().append_as_list(new);
        }
        self.entries.insert(key, new);
    }

    fn safe_key(&self, key: &str) -> Box<str> {
        let mut k = key.to_string();
        while self.is_reserved(&k) {
            k.push('_');
        }
        k.into()
    }

    fn is_reserved(&self, key: &str) -> bool {
        matches!(
            key,
            "items" | "keys" | "values" | "get" | "update" | "pop" | "clear"
        )
    }
}
