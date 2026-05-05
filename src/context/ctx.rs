// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::memo::{Memo, MemoCache, MemoKey};
use crate::SYM_ETX;
use crate::cfg::Configurable;
use crate::context::state::CallStack;
use crate::context::trace::Tracer;
use crate::input::Cursor;
use crate::peg::error::Nope;
use crate::peg::error::{DisasterReport, ParseFailure};
use crate::trees::tree::Tree;
use crate::types::Str;
use crate::util::pyre::{Pattern, escape};
use std::fmt::Debug;
use std::rc::Rc;

pub trait CtxI: Configurable {
    fn cursor(&self) -> &dyn Cursor;
    fn callstack(&self) -> CallStack;
    fn mark(&self) -> usize {
        self.cursor().mark()
    }
    fn cut_seen(&self) -> bool;
}

pub trait Ctx: CtxI + Clone + Debug {
    fn id(&self) -> usize {
        self as *const Self as usize
    }

    fn cursor_mut(&mut self) -> &mut dyn Cursor;
    fn enter(&mut self, name: &str);
    fn leave(&mut self);
    fn track(&mut self, key: &MemoKey) -> usize;
    fn untrack(&mut self, key: &MemoKey) -> usize;
    fn tracer(&self) -> &dyn Tracer;

    fn intern(&mut self, s: &str) -> Str {
        s.into()
    }

    #[track_caller]
    fn failure(&mut self, start: usize, source: ParseFailure) -> Nope {
        let nope = Nope::new(self);

        if let Some(furthest) = self.furthest_failure()
            && furthest.mark >= self.mark()
        {
            return nope;
        }

        let dis = DisasterReport::new(start, self, &source);
        self.set_furthest_failure(&dis);
        nope
    }

    fn set_furthest_failure(&mut self, dis: &DisasterReport);
    fn furthest_failure(&self) -> Option<DisasterReport>;

    fn reset(&mut self, mark: usize) {
        self.cursor_mut().reset(mark);
    }

    fn at_end(&mut self) -> bool {
        self.cursor().at_end()
    }
    fn parse_eof(&mut self) -> bool {
        self.enter(SYM_ETX);
        self.tracer().trace_entry(self);

        self.next_token();
        let result = self.cursor().at_end();

        if result {
            self.tracer().trace_success(self);
        } else {
            self.tracer().trace_failure(self, SYM_ETX);
        }
        self.leave();
        result
    }

    fn dot(&mut self) -> Option<char> {
        self.next()
    }

    fn next(&mut self) -> Option<char> {
        self.cursor_mut().next()
    }

    fn peek(&mut self) -> Option<char> {
        self.cursor_mut().peek()
    }

    fn get_pattern(&mut self, pattern: &str) -> Pattern;

    fn match_token(&mut self, token: &str) -> bool {
        self.next_token();
        let result = {
            let wordlike = token.chars().all(|c| c.is_alphanumeric());
            let escaped = escape(token);
            if wordlike && *escaped == *token && self.cursor().name_guard() {
                let bound = if self.cursor().ignore_case() {
                    format!(r"{}\b", token)
                } else {
                    format!(r"(?i){}\b", token)
                };
                self.match_pattern(bound.as_str()).is_some()
            } else {
                self.cursor_mut().match_token(token)
            }
        };
        if result {
            self.tracer().trace_match(self, token, "");
        } else {
            self.tracer().trace_no_match(self, token, "");
        }
        result
    }

    fn match_pattern(&mut self, pattern: &str) -> Option<Str> {
        let re = self.get_pattern(pattern);
        let result = self.cursor_mut().match_pattern(&re);
        if let Some(matched) = result {
            self.tracer().trace_match(self, matched.as_str(), pattern);
            Some(self.intern(matched.as_str()))
        } else {
            self.tracer().trace_no_match(self, "", pattern);
            None
        }
    }

    fn match_eol(&mut self) -> bool {
        self.cursor_mut().match_eol()
    }

    fn match_void(&mut self) {
        self.next_token();
    }

    fn next_token(&mut self) {
        self.cursor_mut().next_token();
    }

    fn heartbeat_tick(&mut self) {
        let _ = self;
    }

    fn key(&mut self, name: &str, memo: bool) -> MemoKey {
        MemoCache::key(self.mark(), self.intern(name), memo)
    }

    fn memo(&mut self, key: &MemoKey) -> Option<Memo>;

    fn memoize(&mut self, key: &MemoKey, tree: &Rc<Tree>, lastmark: usize);

    fn clear_error_memos(&mut self);

    fn cut(&mut self);
    fn clear_cut(&mut self);

    fn prune_cache(&mut self);

    fn is_keyword(&self, name: &str) -> bool {
        let _ = name;
        false
    }
    fn set_keywords(&mut self, keywords: &[Str]) {
        let _ = keywords;
    }

    fn merge(self, other: Self) -> Self;

    fn push(&mut self) -> Self {
        let mut new = self.clone();
        new.clear_cut();
        new
    }

    fn done(&self) -> bool;
}

impl<C: Ctx> CtxI for Box<C> {
    #[inline]
    fn cursor(&self) -> &dyn Cursor {
        (**self).cursor()
    }

    #[inline]
    fn callstack(&self) -> CallStack {
        (**self).callstack()
    }

    #[inline]
    fn cut_seen(&self) -> bool {
        (**self).cut_seen()
    }
}

impl<C: Ctx> Configurable for Box<C> {
    fn configure(&mut self, cfg: &crate::cfg::Cfg) {
        (**self).configure(cfg)
    }
}

impl<C: Ctx> Ctx for Box<C> {
    #[inline]
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        (**self).cursor_mut()
    }

    #[inline]
    fn enter(&mut self, name: &str) {
        (**self).enter(name)
    }

    #[inline]
    fn leave(&mut self) {
        (**self).leave()
    }

    #[inline]
    fn track(&mut self, key: &MemoKey) -> usize {
        (**self).track(key)
    }

    #[inline]
    fn untrack(&mut self, key: &MemoKey) -> usize {
        (**self).untrack(key)
    }

    #[inline]
    fn tracer(&self) -> &dyn Tracer {
        (**self).tracer()
    }

    #[inline]
    fn intern(&mut self, s: &str) -> Str {
        (**self).intern(s)
    }

    #[inline]
    fn set_furthest_failure(&mut self, dis: &DisasterReport) {
        (**self).set_furthest_failure(dis)
    }

    #[inline]
    fn furthest_failure(&self) -> Option<DisasterReport> {
        (**self).furthest_failure()
    }

    #[inline]
    fn get_pattern(&mut self, pattern: &str) -> Pattern {
        (**self).get_pattern(pattern)
    }

    #[inline]
    fn heartbeat_tick(&mut self) {
        (**self).heartbeat_tick()
    }

    #[inline]
    fn memo(&mut self, key: &MemoKey) -> Option<Memo> {
        (**self).memo(key)
    }

    #[inline]
    fn memoize(&mut self, key: &MemoKey, tree: &Rc<Tree>, lastmark: usize) {
        (**self).memoize(key, tree, lastmark)
    }

    #[inline]
    fn clear_error_memos(&mut self) {
        (**self).clear_error_memos()
    }

    #[inline]
    fn cut(&mut self) {
        (**self).cut()
    }

    #[inline]
    fn clear_cut(&mut self) {
        (**self).clear_cut()
    }

    #[inline]
    fn prune_cache(&mut self) {
        (**self).prune_cache()
    }

    #[inline]
    fn is_keyword(&self, name: &str) -> bool {
        (**self).is_keyword(name)
    }

    #[inline]
    fn set_keywords(&mut self, keywords: &[Str]) {
        (**self).set_keywords(keywords)
    }

    fn merge(self, other: Self) -> Self {
        let this = *self;
        let other = *other;
        this.merge(other).into()
    }

    #[inline]
    fn done(&self) -> bool {
        (**self).done()
    }
}
