// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::ctx::{Ctx, CtxI};
use super::memo::{Key, Memo};
use super::state::{CallStack, HeavyState, ParseState, ParseStateStack};
use super::trace::{CONSOLE_TRACER, NULL_TRACER, Tracer};
use crate::cfg::*;
use crate::engine::ctxproxy::CtxProxy;
use crate::input::Cursor;
use crate::trees::Tree;
use crate::util::pyre::Pattern;

#[derive(Clone, Debug)]
pub struct StackCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub states: ParseStateStack<U>,
    pub heavy: HeavyState<'c>,
}

impl<'c, U> StackCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub fn new(cursor: U, cfga: &CfgA) -> Self {
        let mut ctx = Self {
            states: ParseStateStack::new(cursor),
            heavy: HeavyState::new(),
        };
        ctx.configure(&config(cfga));
        ctx
    }

    pub fn proxy(self) -> CtxProxy<Self> {
        CtxProxy::new(self)
    }

    #[inline]
    #[track_caller]
    fn state(&self) -> &ParseState<U> {
        self.states.state()
    }

    #[inline]
    fn state_mut(&mut self) -> &mut ParseState<U> {
        self.states.state_mut()
    }

    pub fn trace_with(&mut self, tracer: &'c dyn Tracer) {
        self.heavy.tracer = tracer
    }

    pub fn set_trace(&mut self, on: bool) {
        if on {
            self.trace_with(&CONSOLE_TRACER);
            return;
        }
        self.trace_with(&NULL_TRACER);
    }
}

impl<'c, U> CtxI for StackCtx<'c, U>
where
    U: Cursor + Clone,
{
    #[inline]
    #[track_caller]
    fn cursor(&self) -> &dyn Cursor {
        &self.state().cursor
    }

    #[inline]
    #[track_caller]
    fn callstack(&self) -> CallStack {
        self.state().callstack.clone()
    }

    #[inline]
    #[track_caller]
    fn cut_seen(&self) -> bool {
        self.state().cutseen
    }
}

impl<'c, U> Configurable for StackCtx<'c, U>
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

impl<'c, U> Ctx for StackCtx<'c, U>
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
        let stack = self.state().callstack.clone();
        self.state_mut().callstack = stack.tail().unwrap_or_default()
    }

    fn tracer(&self) -> &dyn Tracer {
        self.heavy.tracer
    }

    fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.heavy.get_pattern(pattern)
    }

    fn memo(&mut self, key: &Key) -> Option<Memo> {
        self.heavy.memos.memo(key)
    }

    fn memoize(&mut self, key: &Key, tree: &Tree) {
        let mark = self.cursor().mark();
        self.heavy.memos.memoize(key, tree, mark);
    }

    fn cut(&mut self) {
        self.tracer().trace_cut(self);
        self.state_mut().cutseen = true;
        self.prune_cache();
    }

    fn prune_cache(&mut self) {
        let cutpoint = self.cursor().mark();
        self.heavy.memos.prune(cutpoint);
    }

    fn is_keyword(&self, name: &str) -> bool {
        self.heavy.keywords.binary_search(&name.into()).is_ok()
    }

    fn set_keywords(&mut self, keywords: &[Str]) {
        self.heavy.keywords = keywords.into()
    }

    fn merge(mut self, _other: Self) -> Self {
        self.states.merge();
        self
    }

    fn push(&mut self) -> Self {
        unimplemented!()
    }

    fn push_state(&mut self) {
        self.states.push();
    }

    #[track_caller]
    fn done(&self) -> bool {
        self.state().is_popped()
    }

    fn pop(&mut self) {
        self.states.pop();
    }

    fn undo(&mut self) {
        self.states.undo();
    }
}
