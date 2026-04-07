// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;
use indexmap::IndexMap;

pub type TagMap = IndexMap<Box<str>, Tree>;

/// A structured mapping for AST nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeTags {
    pub tags: TagMap,
}

impl TreeTags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Tree> {
        self.tags.get(key)
    }

    pub fn update(&mut self, other: &TreeTags) {
        for (key, value) in &other.tags {
            self.tags.insert(key.clone(), value.clone());
        }
    }

    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safekey(k);
            self.tags.entry(key).or_insert(Tree::Nil);
        }

        for &k in list_keys {
            let key = self.safekey(k);
            self.tags.entry(key).or_insert(Tree::Branches([].into()));
        }
    }

    pub fn set(&mut self, key: &str, item: Tree) {
        let key = self.safekey(key);
        let mut new = item;
        if let Some(current) = self.tags.get(&key) {
            new = current.clone().add_leaf(new);
        }
        self.tags.insert(key, new);
    }

    pub fn set_list(&mut self, key: &str, item: Tree) {
        let key = self.safekey(key);
        let mut new = item;
        if let Some(current) = self.tags.get(&key) {
            new = current.clone().add_branching(new);
        }
        self.tags.insert(key, new);
    }

    fn safekey(&self, key: &str) -> Box<str> {
        let mut k = key.to_string();
        while self.is_unsafe(&k) {
            k.push('_');
        }
        k.into()
    }

    fn is_unsafe(&self, key: &str) -> bool {
        matches!(
            key,
            "items" | "keys" | "values" | "get" | "update" | "pop" | "clear"
        )
    }
}
