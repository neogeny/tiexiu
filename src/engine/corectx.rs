// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::ctx::{Ctx, CtxI};
use super::memo::{Key, Memo};
use super::state::{CallStack, HeavyState, ParseState};
use super::trace::{CONSOLE_TRACER, NULL_TRACER, Tracer};
use crate::cfg::*;
use crate::input::Cursor;
use crate::trees::Tree;
use crate::util::pyre::Pattern;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub state: Cow<'c, Box<ParseState<U>>>,
    pub heavy: Rc<RefCell<HeavyState<'c>>>,
}

impl<'c, U> Clone for CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    fn clone(&self) -> Self {
        let mut new_state = (*self.state).clone();
        // NOTE: new state, owns the cuts
        new_state.cutseen = false;
        Self {
            state: Cow::Owned(new_state),
            heavy: self.heavy.clone(),
        }
    }
}

impl<'c, U> Drop for CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    fn drop(&mut self) {
        self.undo_unmerged();
    }
}

impl<'c, U> CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub fn new(cursor: U, cfga: &CfgA) -> Self {
        let mut ctx = Self {
            state: Cow::Owned(ParseState::new(cursor).into()),
            heavy: RefCell::new(HeavyState::new()).into(),
        };
        ctx.configure(&config(cfga));
        ctx
    }

    #[inline]
    fn state_mut(&mut self) -> &mut ParseState<U> {
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
    fn callstack(&self) -> CallStack {
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
        self.state_mut().callstack.push(name);
    }
    fn leave(&mut self) {
        let stack = self.state.callstack.clone();
        self.state_mut().callstack = match stack.tail() {
            Some(tail) => tail.clone(),
            None => CallStack::new(),
        }
    }

    fn tracer(&self) -> &dyn Tracer {
        self.heavy.borrow().tracer
    }

    fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.with_heavy_mut(|heavy| heavy.get_pattern(pattern))
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

    fn cut(&mut self) {
        self.tracer().trace_cut(self);
        self.state_mut().cutseen = true;
        self.prune_cache();
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

    fn merge(mut self, mut other: Self) -> Self {
        self.state_mut().merge(other.state_mut());
        self
    }

    fn push(&mut self) -> Self {
        self.clone()
    }

    fn done(&self) -> bool {
        self.state.is_popped()
    }

    fn pop(&mut self) {
        self.state_mut().pop();
    }

    fn undo(&mut self) {
        self.state_mut().pop();
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
    }

    #[test]
    fn clone_resets_cutseen() {
        let cursor = StrCursor::new("test");
        let mut ctx = CoreCtx::new(cursor, &[]);

        ctx.cut();
        assert!(ctx.cut_seen());

        let cloned_ctx = ctx.push();
        assert!(
            !cloned_ctx.cut_seen(),
            "cloned context should have cutseen as false"
        );
        assert!(
            ctx.cut_seen(),
            "original context should still have cutseen as true"
        );
    }

    #[test]
    fn get_pattern_caches() {
        let cursor = StrCursor::new("test");
        let mut ctx = CoreCtx::new(cursor, &[]);

        let p1 = ctx.get_pattern(r"\d+");
        let p2 = ctx.get_pattern(r"\d+");

        assert_eq!(p1.pattern(), p2.pattern());
    }
}
