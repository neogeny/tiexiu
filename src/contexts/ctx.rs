// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::memo::Cache;
use crate::grammars::{ParseResult, Parser};
use crate::input::{Cursor, StrCursor};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Ctx: Clone + Debug {
    fn call(self, name: &str) -> ParseResult<Self>;
    fn mark(&self) -> usize;
    fn eof_check(&self) -> bool;
    fn dot(&mut self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn token(&mut self, token: &str) -> bool;
    fn pattern(&mut self, pattern: &str) -> Option<&str>;
    fn next_token(&mut self);

    fn cut(&mut self);
    fn uncut(&mut self);
    fn cut_seen(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct StrCtx<'c> {
    cursor: Box<StrCursor<'c>>,
    cutseen: bool,
    cache: Rc<RefCell<Cache<'c, Self>>>,
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

    pub fn parser(&mut self, name: &str) -> &'c dyn Parser<Self> {
        let mut cache = self.cache.borrow_mut();
        cache.parser(self.cursor.mark(), name)
    }
}

impl<'c> Ctx for StrCtx<'c> {
    fn call(mut self, name: &str) -> ParseResult<Self> {
        let rule = self.parser(name);
        rule.parse(self)
    }

    fn mark(&self) -> usize {
        self.cursor.mark()
    }

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

    fn cut(&mut self) {
        self.cutseen = true;
    }

    fn uncut(&mut self) {
        self.cutseen = false;
    }

    fn cut_seen(&self) -> bool {
        self.cutseen
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
        assert!(size <= TARGET, "StrCursor size is {} > {} bytes", size, TARGET);
    }
}
