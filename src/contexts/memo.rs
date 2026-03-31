// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use crate::grammars::Rule;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Key(usize, String);

#[derive(Debug)]
pub struct Cache<'c> {
    parsers: HashMap<&'c str, &'c Rule<'c>>,
    memos: HashMap<Key, Cst>,
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
    pub fn memo(&mut self, mark: usize, name: &str) -> Option<Cst> {
        let key = Key(mark, name.into());
        self.memos.get(&key).cloned()
    }

    pub fn memoize(&mut self, mark: usize, name: &str, cst: &Cst) {
        let key = Key(mark, name.into());
        self.memos.insert(key, cst.clone());
    }

    pub fn rule(&mut self, name: &str) -> &'c Rule<'c> {
        if let Some(&parser) = self.parsers.get(name) {
            return parser;
        }
        panic!("no such parser: {}", name);
    }
}
