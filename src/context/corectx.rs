// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::ctx::{Ctx, CtxSem};
use super::memo::{Memo, MemoKey};
use super::state::{CallStack, HeavyState, ParseState};
use super::trace::{Tracer, CONSOLE_TRACER, NULL_TRACER};
use crate::cfg::*;
use crate::input::Cursor;
use crate::peg::error::DisasterReport;
use crate::trees::Tree;
use crate::types::Str;
use crate::util::pyre::Pattern;
use std::rc::Rc;
use std::time::Instant;

/// The primary parsing context, wrapping a cursor and shared parsing state.
///
/// `CoreCtx` is the default context used by `new_ctx()`.  It owns the cursor
/// position (`ParseState`) and holds the memo tables, pattern caches, keywords
/// and tracing infrastructure in [`HeavyState`].
#[derive(Debug)]
pub struct CoreCtx<'c, U>
where
    U: Cursor,
{
    /// The mutable parse state (cursor position, key tracking).
    pub state: ParseState<U>,
    /// Heavyweight state (memos, patterns, tracer, heartbeat, etc.).
    pub heavy: HeavyState<'c>,
}

impl<'c, U> CoreCtx<'c, U>
where
    U: Cursor + 'c,
{
    /// Creates a new `CoreCtx` from a cursor and configuration array.
    pub fn new(cursor: U, cfga: &CfgA) -> Self {
        let len = cursor.as_str().len();
        let mut ctx = Self {
            state: ParseState::new(cursor),
            heavy: HeavyState::new(),
        };
        ctx.heavy.input_len = len;
        ctx.configure(&config(cfga));
        ctx
    }

    /// Sets a custom tracer for debugging parse execution.
    pub fn trace_with(&mut self, tracer: &'c dyn Tracer) {
        self.heavy.tracer = tracer
    }

    /// Enables or disables console tracing for parse execution.
    pub fn set_trace(&mut self, on: bool) {
        if on {
            self.trace_with(&CONSOLE_TRACER);
            return;
        }
        self.trace_with(&NULL_TRACER);
    }
}

impl<'c, U> Ctx for CoreCtx<'c, U>
where
    U: Cursor + 'c,
{
    #[inline]
    fn cursor(&self) -> &dyn Cursor {
        &self.state.cursor
    }

    #[inline]
    fn callstack(&self) -> CallStack {
        self.heavy.callstack.clone()
    }

    #[inline]
    fn cut_seen(&self) -> bool {
        *self.heavy.cutstack.last().unwrap_or(&false)
    }
}

impl<'c, U> Configurable for CoreCtx<'c, U>
where
    U: Cursor + 'c,
{
    fn configure(&mut self, cfg: &Cfg) {
        self.cursor_mut().configure(cfg);

        if cfg.contains(&CfgKey::Trace) {
            self.set_trace(true);
        }

        if let Some(hb) = cfg.heartbeat() {
            self.heavy.heartbeat = Some(hb.clone());
        }
    }
}

impl<'c, U> CtxSem for CoreCtx<'c, U>
where
    U: Cursor + 'c,
{
    #[inline]
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        &mut self.state.cursor
    }

    fn enter(&mut self, name: &str) {
        self.heavy.callstack.push(name);
    }

    fn leave(&mut self) {
        self.heavy.callstack = self.heavy.callstack.tail().unwrap_or_default();
    }

    fn untrack(&mut self, key: &MemoKey) -> usize {
        self.state.keytrack.untrack(key)
    }

    fn tracer(&self) -> &dyn Tracer {
        self.heavy.tracer
    }

    fn enter_lookahead(&mut self) {
        self.state.lookahead_depth += 1;
    }

    fn leave_lookahead(&mut self) {
        debug_assert!(self.state.lookahead_depth >= 1);
        self.state.lookahead_depth -= 1;
    }

    fn track(&mut self, key: &MemoKey) -> usize {
        self.state.keytrack.track(key)
    }

    fn intern(&mut self, s: &str) -> Str {
        self.heavy.memos.intern(s)
    }

    fn set_furthest_failure(&mut self, dis: DisasterReport) {
        self.heavy.set_furthest_failure(dis);
    }

    fn furthest_failure(&self) -> Option<DisasterReport> {
        self.heavy.furthest_failure.clone()
    }

    fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.heavy.get_pattern(pattern)
    }

    fn heartbeat_tick(&mut self) {
        if self.heavy.instant.elapsed().as_millis() < 128 {
            return;
        }
        if let Some(hb) = self.heavy.heartbeat.clone() {
            let mark = self.mark();
            let total = self.cursor().as_str().len();
            if total == 0 {
                return;
            }
            hb.tick(mark, total);
        }
        self.heavy.instant = Instant::now();
    }

    fn key(&mut self, name: &str, can_memo: bool) -> MemoKey {
        self.heavy.memos.key(self.mark(), name.into(), can_memo)
    }

    fn memo(&mut self, key: &MemoKey) -> Option<Memo> {
        self.heavy.memos.memo(key)
    }

    fn memoize(&mut self, key: &MemoKey, tree: &Rc<Tree>, lastmark: usize) {
        self.heavy.memos.memoize(key, tree, lastmark);
    }

    fn cut(&mut self) {
        self.tracer().trace_cut(self);
        if let Some(last) = self.heavy.cutstack.last_mut() {
            *last = true;
        }

        let mark = self.mark();
        if self.state.lookahead_depth == 0 && mark > self.state.last_cut_mark {
            // NOTE
            //  Kota Mizushima et al explain memo cache prunning over cut
            //  _
            //      https://kmizu.github.io/papers/paste513-mizushima.pdf
            //      https://ceur-ws.org/Vol-1269/paper232.pdf
            //
            let cutpoint = mark;
            self.heavy.memos.prune(cutpoint);
            self.state.last_cut_mark = mark;
        }
    }

    fn push_cut(&mut self) {
        self.heavy.cutstack.push(false);
    }

    fn take_cut(&mut self) -> bool {
        let cutseen = self.cut_seen();
        self.heavy.cutstack.pop();
        cutseen
    }

    fn is_keyword(&self, name: &str) -> bool {
        self.heavy.keywords.binary_search(&name.into()).is_ok()
    }

    fn set_keywords(&mut self, keywords: &[Str]) {
        self.heavy.keywords = keywords.into()
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
    fn get_pattern_caches() {
        let cursor = StrCursor::new("test");
        let mut ctx = CoreCtx::new(cursor, &[]);

        let p1 = ctx.get_pattern(r"\d+");
        let p2 = ctx.get_pattern(r"\d+");

        assert_eq!(p1.pattern(), p2.pattern());
    }
}
