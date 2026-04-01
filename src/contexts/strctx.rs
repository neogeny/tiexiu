// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::ctx::Ctx;
use super::memo::{Cache, Key, Memo};
use crate::grammars::Rule;
use crate::input::{Cursor, StrCursor};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct StrCtx<'c> {
    cursor: Box<StrCursor<'c>>,
    cutseen: bool,
    cache: Rc<RefCell<Cache<'c>>>,
}

impl<'c> StrCtx<'c> {
    pub fn new(cursor: StrCursor<'c>) -> Self {
        let map = HashMap::new();
        Self {
            cursor: cursor.into(),
            cutseen: false,
            cache: Rc::new(RefCell::new(Cache::new(map))),
        }
    }
}

impl<'c> Ctx for StrCtx<'c> {
    fn eof_check(&self) -> bool {
        self.cursor.at_end()
    }

    fn dot(&mut self) -> bool {
        self.next().is_some()
    }

    fn next(&mut self) -> Option<char> {
        self.cursor.next()
    }

    fn token(&mut self, token: &str) -> bool {
        self.cursor.token(token)
    }

    fn pattern(&mut self, pattern: &str) -> Option<&str> {
        self.cursor.pattern(pattern)
    }

    fn next_token(&mut self) {
        self.cursor.next_token();
    }

    fn mark(&self) -> usize {
        self.cursor.mark()
    }

    fn reset(&mut self, mark: usize) {
        self.cursor.reset(mark);
    }

    fn memo(&mut self, key: &Key) -> Option<Memo> {
        let mut cache = self.cache.borrow_mut();
        cache.memo(key)
    }

    fn memoize(&mut self, key: &Key, cst: &Cst) {
        let mut cache = self.cache.borrow_mut();
        cache.memoize(key, cst, self.mark());
    }

    fn cut(&mut self) {
        self.cutseen = true;
        let mut cache = self.cache.borrow_mut();
        cache.prune(self.mark())
    }

    fn uncut(&mut self) {
        self.cutseen = false;
    }

    fn cut_seen(&self) -> bool {
        self.cutseen
    }

    fn parser(&self, name: &str) -> (Self, &Rule<'_>) {
        let mut cache = self.cache.borrow_mut();
        (self.clone(), cache.rule(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    const TARGET: usize = 32;

    #[test]
    fn test_ctx_size() {
        let size = size_of::<StrCtx>();
        // 24 bytes: Box (8) + Rc (8) + bool/padding (8)
        assert!(size <= TARGET, "StrCtx size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_cursor_size() {
        let size = size_of::<StrCursor>();
        // StrCursor contains &str (16) and usize (8) = 24 bytes.
        assert!(
            size <= TARGET,
            "StrCursor size is {} > {} bytes",
            size,
            TARGET
        );
    }
}
