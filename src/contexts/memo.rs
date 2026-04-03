// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::astree::cst::Cst;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Key {
    pub mark: usize,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Memo {
    pub cst: Cst,
    pub mark: usize,
}

#[derive(Debug)]
pub struct Cache {
    memos: HashMap<Key, Memo>,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Self {
        Self {
            memos: HashMap::new(),
        }
    }
}

impl Cache {
    pub fn key(mark: usize, name: &str) -> Key {
        Key {
            mark,
            name: name.to_string(),
        }
    }

    pub fn memo(&mut self, key: &Key) -> Option<Memo> {
        self.memos.get(key).cloned()
    }

    pub fn memoize(&mut self, key: &Key, cst: &Cst, mark: usize) {
        let memo = Memo {
            cst: cst.clone(),
            mark,
        };
        self.memos.insert(key.clone(), memo);
    }

    pub fn prune(&mut self, cutpoint: usize) {
        self.memos.retain(|key, _| key.mark >= cutpoint);
    }
}
