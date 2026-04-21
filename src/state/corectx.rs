// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::ctx::{Ctx, CtxI};
use super::memo::{Key, Memo, MemoCache};
use super::trace::{CONSOLE_TRACER, NULL_TRACER, Tracer};
use crate::cfg::*;
use crate::input::Cursor;
use crate::peg::parser::TokenList;
use crate::trees::Tree;
use crate::util::pyre::Pattern;
use std::borrow::Cow;
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

#[derive(Debug)]
pub struct HeavyState<'t> {
    pub memos: MemoCache,
    pub patterns: PatternCache,
    pub keywords: Box<[Box<str>]>,
    pub tracer: &'t dyn Tracer,
}

#[derive(Clone, Debug)]
pub struct CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub state: Cow<'c, Box<State<U>>>,
    pub heavy: Rc<RefCell<HeavyState<'c>>>,
}

impl<'c, U> CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub fn new(cursor: U, cfg: &CfgA) -> Self {
        let _ = cfg;
        Self {
            state: Cow::Owned(
                State {
                    cursor,
                    cutseen: false,
                    callstack: TokenList::new(),
                }
                .into(),
            ),
            heavy: Rc::new(RefCell::new(HeavyState {
                memos: MemoCache::new(),
                patterns: PatternCache::new(),
                keywords: [].into(),
                tracer: &NULL_TRACER,
            })),
        }
    }

    #[inline]
    fn state_mut(&mut self) -> &mut State<U> {
        self.state.to_mut()
    }

    #[inline]
    fn with_heavy_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut HeavyState) -> R,
    {
        let mut heavy = self.heavy.borrow_mut();
        f(&mut heavy)
    }

    pub fn trace_with(&mut self, tracer: &'c dyn Tracer) {
        self.heavy.borrow_mut().tracer = tracer
    }

    pub fn set_trace(&mut self, on: bool) {
        if on {
            self.trace_with(&CONSOLE_TRACER);
            return;
        }
        self.trace_with(&NULL_TRACER);
    }
}

impl<'c, U> CtxI for CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    #[inline]
    fn cursor(&self) -> &dyn Cursor {
        &self.state.cursor
    }

    #[inline]
    fn callstack(&self) -> TokenList {
        self.state.callstack.clone()
    }

    #[inline]
    fn cut_seen(&self) -> bool {
        self.state.cutseen
    }
}

impl<'c, U> Configurable for CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    fn configure(&mut self, cfg: &CfgBox) {
        self.cursor_mut().configure(cfg);

        if cfg.contains(&Cfg::Trace) {
            self.set_trace(true);
        }
    }
}

impl<'c, U> Ctx for CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    #[inline]
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        &mut self.state_mut().cursor
    }

    fn enter(&mut self, name: &str) {
        let stack = self.state.callstack.clone();
        self.state_mut().callstack = stack.insert(name);
    }

    fn leave(&mut self) {
        let stack = self.state.callstack.clone();
        self.state_mut().callstack = match stack.tail() {
            Some(tail) => tail.clone(),
            None => TokenList::new(),
        }
    }

    fn tracer(&self) -> &dyn Tracer {
        self.heavy.borrow().tracer
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
        let mark = self.cursor().mark();
        self.with_heavy_mut(|heavy| {
            heavy.memos.memoize(key, tree, mark);
        });
    }

    fn set_cut_seen(&mut self) {
        self.tracer().trace_cut(self);
        self.state_mut().cutseen = true;
    }
    #[inline]
    fn unset_cut(&mut self) {
        self.state_mut().cutseen = false;
    }

    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_heavy_mut(|heavy| heavy.memos.prune(cutpoint));
    }

    fn is_keyword(&self, name: &str) -> bool {
        self.heavy
            .borrow()
            .keywords
            .binary_search(&name.into())
            .is_ok()
    }

    fn set_keywords(&mut self, keywords: &[Box<str>]) {
        self.heavy.borrow_mut().keywords = keywords.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::StrCursor;

    #[test]
    fn new_context() {
        let cursor = StrCursor::new("test");
        let ctx = CoreCtx::new(cursor, &[]);

        assert!(!ctx.cut_seen());
    }

    #[test]
    fn enter_rule() {
        let cursor = StrCursor::new("test");
        let mut ctx = CoreCtx::new(cursor, &[]);

        ctx.enter("rule");

        let stack = ctx.callstack();
        assert!(stack.to_vec().contains(&"rule"));
    }

    #[test]
    fn cut_and_uncut() {
        let cursor = StrCursor::new("test");
        let mut ctx = CoreCtx::new(cursor, &[]);

        ctx.cut();
        assert!(ctx.cut_seen());

        ctx.unset_cut();
        assert!(!ctx.cut_seen());
    }

    #[test]
    fn get_pattern_caches() {
        let cursor = StrCursor::new("test");
        let ctx = CoreCtx::new(cursor, &[]);

        let p1 = ctx.get_pattern(r"\d+");
        let p2 = ctx.get_pattern(r"\d+");

        assert_eq!(p1.pattern(), p2.pattern());
    }
}
