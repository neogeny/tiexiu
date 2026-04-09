// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Cursor;
use crate::state::Ctx;
use crate::state::memo::{Key, Memo, MemoCache};
use crate::trees::Tree;
use crate::util::pyre::Pattern as Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type RegexCache = HashMap<String, Regex>;

#[derive(Clone, Debug)]
pub struct State<U: Cursor> {
    pub cursor: U,
    pub cutseen: bool,
}

#[derive(Clone, Debug)]
pub struct HeavyState<'c> {
    pub memos: MemoCache,
    pub regex: RegexCache,
    pub marker: std::marker::PhantomData<&'c ()>,
}

#[derive(Clone, Debug)]
pub struct CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub state: Rc<State<U>>,
    pub heavy: Rc<RefCell<HeavyState<'c>>>,
}

impl<'c, U> CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub fn new(cursor: U) -> Self {
        Self {
            state: State {
                cursor,
                cutseen: false,
            }
            .into(),
            heavy: RefCell::new(HeavyState {
                memos: MemoCache::new(),
                regex: RegexCache::new(),
                marker: std::marker::PhantomData,
            })
            .into(),
        }
    }

    #[inline]
    fn state_mut(&mut self) -> &mut State<U> {
        Rc::make_mut(&mut self.state)
    }

    #[inline]
    fn with_heavy_mut<F, R>(&self, f: F) -> R
    where
        // We define the closure to take &mut HeavyState
        F: FnOnce(&mut HeavyState<'c>) -> R,
    {
        let mut heavy = self.heavy.borrow_mut();
        f(&mut heavy)
    }
}

impl<'c, U> Ctx for CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    #[inline]
    fn cursor(&self) -> &dyn Cursor {
        &self.state.cursor
    }

    #[inline]
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        &mut self.state_mut().cursor
    }

    fn regex(&self, pattern: &str) -> Regex {
        self.with_heavy_mut(|heavy| {
            heavy
                .regex
                .entry(pattern.to_string())
                .or_insert_with(|| Regex::new(pattern).unwrap())
                .clone()
        })
    }

    fn memo(&mut self, key: &Key) -> Option<Memo> {
        self.with_heavy_mut(|heavy| heavy.memos.memo(key))
    }

    fn memoize(&mut self, key: &Key, tree: &Tree) {
        let mark = self.mark();
        self.with_heavy_mut(|heavy| {
            heavy.memos.memoize(key, tree, mark);
        });
    }

    #[inline]
    fn cut_seen(&self) -> bool {
        self.state.cutseen
    }
    #[inline]
    fn uncut(&mut self) {
        self.state_mut().cutseen = false;
    }

    fn cut(&mut self) {
        self.state_mut().cutseen = true;
        self.prune_cache();
    }

    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_heavy_mut(|heavy| heavy.memos.prune(cutpoint));
    }
}
