// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::memo::{Memo, MemoCache, MemoKey};
use crate::SYM_ETX;
use crate::cfg::Configurable;
use crate::context::state::CallStack;
use crate::context::trace::Tracer;
use crate::input::Cursor;
use crate::peg::Rule;
use crate::peg::error::{DisasterReport, ParseFailure};
use crate::peg::error::{Nope, ParseResult, Yeap};
use crate::trees::tree::Tree;
use crate::types::Str;
use crate::util::pyre::{Pattern, escape};
use std::fmt::Debug;
use std::rc::Rc;

const MAX_RECURSION_DEPTH: usize = 64;

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
        let dis = DisasterReport::new(start, self, &source);
        self.set_furthest_failure(&dis);
        Nope::new(self)
    }

    fn furthest_failure(&self) -> Option<DisasterReport>;
    fn set_furthest_failure(&mut self, dis: &DisasterReport);

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

    fn call(mut self, name: &str, rule: &Rule) -> ParseResult<Self> {
        let start = self.mark();
        let key = self.key(name, rule.is_memoizable());

        if !rule.is_token() {
            self.next_token();
        }
        if rule.should_trace() {
            self.enter(name);
            self.tracer().trace_entry(&self);
        }

        match self.push().do_call(name, rule) {
            Ok(Yeap(mut ctx, tree)) => {
                if rule.should_trace() {
                    ctx.leave();
                }
                if rule.is_name()
                    && let Tree::Text(name) = tree.as_ref()
                    && ctx.is_keyword(name)
                {
                    ctx.memoize(&key, &Tree::Bottom.into(), ctx.mark());
                    let error = ParseFailure::ReservedWord(name.clone());
                    ctx.tracer().trace_failure(&ctx, name);
                    return Err(ctx.failure(start, error));
                }
                ctx.tracer().trace_success(&ctx);
                ctx.memoize(&key, &tree, ctx.mark());
                self.heartbeat_tick();
                Ok(Yeap(ctx, tree))
            }
            Err(mut nope) => {
                if rule.should_trace() {
                    self.leave();
                }
                self.tracer().trace_failure(&self, name);
                self.memoize(&key, &Tree::Bottom.into(), self.mark());
                nope.take_cut();
                Err(nope)
            }
        }
    }

    fn do_call(mut self, name: &str, rule: &Rule) -> ParseResult<Self> {
        let start = self.mark();
        let key = self.key(name, rule.is_memoizable());
        if let Some(memo) = self.memo(&key) {
            return match memo.tree.as_ref() {
                Tree::Bottom => {
                    let err = ParseFailure::FailedParse(name.into());
                    Err(self.failure(start, err))
                }
                _ => {
                    self.reset(memo.mark);
                    Ok(Yeap(self.into(), memo.tree))
                }
            };
        }

        if rule.is_left_recursive() {
            self.call_recursive(&key, rule)
        } else {
            rule.parse(self)
        }
    }

    fn track_recursion_depth(&mut self, key: &MemoKey) -> Result<(), Nope> {
        let depth = self.track(key);
        if depth > MAX_RECURSION_DEPTH {
            panic!("Recursion depth exceeded")
        } else {
            Ok(())
        }
    }

    fn call_recursive(mut self, key: &MemoKey, rule: &Rule) -> ParseResult<Self> {
        self.tracer().trace_recursion(&self);
        if !rule.is_left_recursive() {
            panic!("Recursive call on non-LRec rule");
        }

        let start = self.mark();
        let mut lastmark = start;
        let mut lasttree: Tree = Tree::Nil;
        let mut lastnope: Option<Nope> = None;

        self.memoize(key, &Tree::Bottom.into(), start);
        loop {
            self.reset(start);

            self.track_recursion_depth(key)?;
            let result = rule.parse(self.push());
            self.untrack(key);

            match result {
                Err(nope) => {
                    lastnope = Some(nope);
                    break;
                }
                Ok(Yeap(ctx, tree)) => {
                    let endmark = ctx.mark();
                    let endtree = tree;
                    if endmark <= lastmark {
                        break;
                    }
                    lastmark = endmark;
                    lasttree = Rc::unwrap_or_clone(endtree);
                    self = self.merge(*ctx);
                    self.memoize(key, &lasttree.clone().into(), lastmark);
                }
            }
        }

        self.reset(lastmark);
        self.memoize(key, &lasttree.clone().into(), lastmark);

        if lasttree == Tree::Bottom {
            let nope = lastnope.unwrap_or(self.failure(
                start,
                ParseFailure::FailedRecursion(
                    key.name.clone(),
                    start,
                    lastmark,
                    lasttree.clone().into(),
                ),
            ));
            return Err(nope);
        }
        Ok(Yeap(self.into(), lasttree.into()))
    }
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
    fn furthest_failure(&self) -> Option<DisasterReport> {
        (**self).furthest_failure()
    }

    #[inline]
    fn set_furthest_failure(&mut self, dis: &DisasterReport) {
        (**self).set_furthest_failure(dis)
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
