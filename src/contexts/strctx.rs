// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::grammars::Parser;
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
    _cursor: StrCursor<'c>,
    cutseen: bool,
    rulemap: &'c HashMap<&'c str, Rule<'c>>,
    cache: Rc<RefCell<Cache>>,
}

impl<'c> StrCtx<'c> {
    pub fn new(
        cursor: StrCursor<'c>,
        rulemap: &'c HashMap<&'c str, Rule<'c>>,
    ) -> Self {
        Self {
            _cursor: cursor,
            cutseen: false,
            rulemap,
            cache: Rc::new(RefCell::new(Cache::new())),
        }
    }
}

impl<'c> Ctx for StrCtx<'c> {
    fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        for<'a> F: FnOnce(&mut Cache) -> R,
    {
        let mut cache = self.cache.borrow_mut();
        f(&mut cache)
    }

    fn cursor(&mut self) -> &mut dyn Cursor {
        &mut self._cursor
    }

    fn cut_seen(&self) -> bool {
        self.cutseen
    }

    fn cut(&mut self) {
        self.cutseen = true;
        self.prune_cache();
    }

    fn uncut(&mut self) {
        self.cutseen = false;
    }
    
    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_cache_mut(|cache| cache.prune(cutpoint));
    }

    fn parser(self, name: &str) -> (Self, &dyn Parser<Self>) {
        let rule = self.rulemap
            .get(name)
            .unwrap_or_else(|| panic!("rule '{}' not found", name));
        (self, rule.rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    const TARGET: usize = 16;

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
