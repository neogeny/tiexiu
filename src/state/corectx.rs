// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Cursor;
use crate::peg::parser::TokenList;
use crate::state::memo::{Key, Memo, MemoCache};
use crate::state::trace::{NullTracer, Tracer};
use crate::state::Ctx;
use crate::trees::Tree;
use crate::util::pyre::Pattern;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type PatternCache = HashMap<String, Pattern>;

#[derive(Clone, Debug)]
pub struct State<U: Cursor> {
    pub cursor: U,
    pub cutseen: bool,
    pub callstack: TokenList,
}

#[derive(Clone, Debug)]
pub struct HeavyState<'c, T: Tracer> {
    pub memos: MemoCache,
    pub patterns: PatternCache,
    pub tracer: T,
    pub marker: std::marker::PhantomData<&'c ()>,
}

#[derive(Clone, Debug)]
pub struct CoreCtx<'c, U, T: Tracer = NullTracer>
where
    U: Cursor + Clone,
    T: Tracer,
{
    pub state: Rc<State<U>>,
    pub heavy: Rc<RefCell<HeavyState<'c, T>>>,
}

impl<'c, U, T: Tracer> CoreCtx<'c, U, T>
where
    U: Cursor + Clone,
    T: Tracer,
{
    pub fn new(cursor: U) -> Self {
        Self {
            state: State {
                cursor,
                cutseen: false,
                callstack: TokenList::new(),
            }
            .into(),
            heavy: RefCell::new(HeavyState {
                memos: MemoCache::new(),
                patterns: PatternCache::new(),
                tracer: T::default(),
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
        F: FnOnce(&mut HeavyState<'c, T>) -> R,
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

    #[inline]
    fn stack(&self) -> TokenList {
        self.state.callstack.clone()
    }

    fn enter(&mut self, name: &str) {
        let stack = self.state.callstack.clone();
        self.state_mut().callstack = stack.insert(name);
    }

    fn get_pattern(&self, pattern: &str) -> Pattern {
        self.with_heavy_mut(|heavy| {
            heavy
                .patterns
                .entry(pattern.to_string())
                .or_insert_with(|| Pattern::new(pattern).unwrap())
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

    fn setcut(&mut self) {
        // TODO: self.tracer.trace_cut(self.cursor)
        self.state_mut().cutseen = true;
        self.prune_cache();
    }

    #[inline]
    fn uncut(&mut self) {
        self.state_mut().cutseen = false;
    }

    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_heavy_mut(|heavy| heavy.memos.prune(cutpoint));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::StrCursor;

    #[test]
    fn new_context() {
        let cursor = StrCursor::new("test");
        let ctx = CoreCtx::new(cursor);

        assert!(!ctx.cut_seen());
    }

    #[test]
    fn enter_rule() {
        let cursor = StrCursor::new("test");
        let mut ctx = CoreCtx::new(cursor);

        ctx.enter("rule");

        let stack = ctx.stack();
        assert!(stack.to_vec().contains(&"rule".to_string()));
    }

    #[test]
    fn cut_and_uncut() {
        let cursor = StrCursor::new("test");
        let mut ctx = CoreCtx::new(cursor);

        ctx.setcut();
        assert!(ctx.cut_seen());

        ctx.uncut();
        assert!(!ctx.cut_seen());
    }

    #[test]
    fn get_pattern_caches() {
        let cursor = StrCursor::new("test");
        let ctx = CoreCtx::new(cursor);

        let p1 = ctx.get_pattern(r"\d+");
        let p2 = ctx.get_pattern(r"\d+");

        assert_eq!(p1.pattern(), p2.pattern());
    }
}
