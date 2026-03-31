// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ctx::Ctx;
use crate::grammars::Parser;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Key(usize, String);

#[derive(Debug)]
pub struct Cache<'c, C: Ctx> {
    parsers: HashMap<&'c str, &'c dyn Parser<C>>,
    memos: HashMap<Key, &'c dyn Parser<C>>,
}

impl<'c, C: Ctx> Cache<'c, C> {
    pub fn new(parsers: HashMap<&'c str, &'c dyn Parser<C>>) -> Self {
        Self {
            memos: HashMap::new(),
            parsers,
        }
    }
}

impl<'c, C: Ctx> Cache<'c, C> {
    pub fn memo(&self, mark: usize, name: &str) -> Option<&'c dyn Parser<C>> {
        let key = Key(mark, name.into());
        if let Some(&p) = self.memos.get(&key) {
            Some(p)
        } else {
            None
        }
    }

    pub fn memoize(&mut self, mark: usize, name: &str, parser: &'c dyn Parser<C>) {
        let key = Key(mark, name.into());
        self.memos.insert(key, parser);
    }

    pub fn parser(&mut self, mark: usize, name: &str) -> &'c dyn Parser<C> {
        if let Some(p) = self.memo(mark, name) {
            return p;
        }
        if let Some(&p) = self.parsers.get(name) {
            self.memoize(mark, name, p);
            return p;
        }
        panic!("no such parser: {}", name);
    }
}
