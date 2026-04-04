// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::tree::Tree;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Key {
    pub mark: usize,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Memo {
    pub tree: Tree,
    pub mark: usize,
}

#[derive(Clone, Debug)]
pub struct MemoCache {
    memos: HashMap<Key, Memo>,
}

impl Default for MemoCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoCache {
    pub fn new() -> Self {
        Self {
            memos: HashMap::new(),
        }
    }
}

impl MemoCache {
    pub fn key(mark: usize, name: &str) -> Key {
        Key {
            mark,
            name: name.to_string(),
        }
    }

    pub fn memo(&mut self, key: &Key) -> Option<Memo> {
        self.memos.get(key).cloned()
    }

    pub fn memoize(&mut self, key: &Key, tree: &Tree, mark: usize) {
        let memo = Memo {
            tree: tree.clone(),
            mark,
        };
        self.memos.insert(key.clone(), memo);
    }

    pub fn prune(&mut self, cutpoint: usize) {
        self.memos.retain(|key, _| key.mark >= cutpoint);
    }
}
