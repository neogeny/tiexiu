// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use crate::grammars::Rule;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Key(usize, String);

#[derive(Clone, Debug)]
pub struct Memo {
    pub cst: Cst,
    pub mark: usize,
}

#[derive(Debug)]
pub struct Cache<'c> {
    parsers: HashMap<&'c str, &'c Rule<'c>>,
    memos: HashMap<Key, Memo>,
}

impl<'c> Cache<'c> {
    pub fn new(parsers: HashMap<&'c str, &'c Rule<'c>>) -> Self {
        Self {
            memos: HashMap::new(),
            parsers,
        }
    }
}

impl<'c> Cache<'c> {
    pub fn key(mark: usize, name: &str) -> Key {
        Key(mark, name.into())
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

    pub fn rule(&mut self, name: &str) -> &'c Rule<'c> {
        if let Some(&parser) = self.parsers.get(name) {
            return parser;
        }
        panic!("no such parser: {}", name);
    }
    
    pub fn prune(&mut self, cutpoint: usize) {
        self.memos.retain(|key, _| key.0 >= cutpoint);
    }
}
