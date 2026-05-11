// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::ctx::{Ctx, CtxI};
use super::memo::{Memo, MemoKey};
use super::state::{CallStack, HeavyState, ParseState};
use super::trace::{CONSOLE_TRACER, NULL_TRACER, Tracer};
use crate::cfg::*;
use crate::input::Cursor;
use crate::peg::error::DisasterReport;
use crate::trees::Tree;
use crate::types::Str;
use crate::util::pyre::Pattern;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub state: Cow<'c, Box<ParseState<U>>>,
    pub heavy: Rc<RefCell<HeavyState<'c>>>,
}

impl<'c, U> CoreCtx<'c, U>
where
    U: Cursor + Clone + 'c,
{
    pub fn new(cursor: U, cfga: &CfgA) -> Self {
        let len = cursor.as_str().len();
        let mut ctx = Self {
            state: Cow::Owned(ParseState::new(cursor).into()),
            heavy: RefCell::new(HeavyState::new()).into(),
        };
        ctx.heavy.borrow_mut().input_len = len;
        ctx.configure(&config(cfga));
        ctx
    }

    #[inline]
    fn state_mut(&mut self) -> &mut ParseState<U> {
        self.state.to_mut()
    }

    #[inline]
    fn _with_heavy_mut<F, R>(&self, f: F) -> R
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
        self.heavy.borrow_mut().callstack.clone()
    }

    #[inline]
    fn _cut_seen(&self) -> bool {
        self.state.cutseen
    }
}

impl<'c, U> Configurable for CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    fn configure(&mut self, cfg: &Cfg) {
        self.cursor_mut().configure(cfg);

        if cfg.contains(&CfgKey::Trace) {
            self.set_trace(true);
        }

        if let Some(hb) = cfg.heartbeat() {
            self.heavy.borrow_mut().heartbeat = Some(hb.clone());
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
        self.heavy.borrow_mut().callstack.push(name);
    }

    fn leave(&mut self) {
        let tail = self.heavy.borrow().callstack.tail().unwrap_or_default();
        self.heavy.borrow_mut().callstack = tail;
    }

    fn untrack(&mut self, key: &MemoKey) -> usize {
        self.state_mut().keytrack.untrack(key)
    }

    fn tracer(&self) -> &dyn Tracer {
        self.heavy.borrow().tracer
    }

    fn track(&mut self, key: &MemoKey) -> usize {
        self.state_mut().keytrack.track(key)
    }

    fn intern(&mut self, s: &str) -> Str {
        self.heavy.borrow_mut().memos.intern(s)
    }

    fn set_furthest_failure(&mut self, dis: DisasterReport) {
        self.heavy.borrow_mut().set_furthest_failure(dis);
    }

    fn furthest_failure(&self) -> Option<DisasterReport> {
        self.heavy.borrow().furthest_failure.clone()
    }

    fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.heavy.borrow_mut().get_pattern(pattern)
    }

    fn heartbeat_tick(&mut self) {
        if self.heavy.borrow().instant.elapsed().as_millis() < 128 {
            return;
        }
        if let Some(hb) = self.heavy.borrow().heartbeat.clone() {
            let mark = self.mark();
            let total = self.cursor().as_str().len();
            if total == 0 {
                return;
            }
            hb.tick(mark, total);
        }
        self.heavy.borrow_mut().instant = Instant::now();
    }

    fn key(&mut self, name: &str, can_memo: bool) -> MemoKey {
        self.heavy
            .borrow_mut()
            .memos
            .key(self.mark(), name.into(), can_memo)
    }

    fn memo(&mut self, key: &MemoKey) -> Option<Memo> {
        self.heavy.borrow_mut().memos.memo(key)
    }

    fn memoize(&mut self, key: &MemoKey, tree: &Rc<Tree>, lastmark: usize) {
        self.heavy.borrow_mut().memos.memoize(key, tree, lastmark);
    }

    fn clear_error_memos(&mut self) {
        self.heavy.borrow_mut().memos.clear_error_memos();
    }

    fn cut(&mut self) {
        self.tracer().trace_cut(self);
        self.state_mut().cutseen = true;
        // self.prune_cache();
    }

    fn clear_cut(&mut self) {
        self.state_mut().cutseen = false;
    }

    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.heavy.borrow_mut().memos.prune(cutpoint);
    }

    fn is_keyword(&self, name: &str) -> bool {
        self.heavy
            .borrow()
            .keywords
            .binary_search(&name.into())
            .is_ok()
    }

    fn set_keywords(&mut self, keywords: &[Str]) {
        self.heavy.borrow_mut().keywords = keywords.into()
    }

    // FIXME
    // fn merge(mut self, other: &Self) -> Self {
    //     self.state_mut().merge(&other.state);
    //     self
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::StrCursor;

    #[test]
    fn new_context() {
        let cursor = StrCursor::new("test");
        let ctx = CoreCtx::new(cursor, &[]);

        assert!(!ctx._cut_seen());
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
        assert!(ctx._cut_seen());
    }

    #[test]
    fn clone_resets_cutseen() {
        let cursor = StrCursor::new("test");
        let mut ctx = CoreCtx::new(cursor, &[]);

        ctx.cut();
        assert!(ctx._cut_seen());

        let cloned_ctx = ctx.push();
        assert!(
            !cloned_ctx._cut_seen(),
            "cloned context should have cutseen as false"
        );
        assert!(
            ctx._cut_seen(),
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
