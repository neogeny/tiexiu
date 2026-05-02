// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::memo::{Memo, MemoCache, MemoKey};
use crate::SYM_ETX;
use crate::cfg::Configurable;
use crate::engine::state::CallStack;
use crate::engine::trace::Tracer;
use crate::input::Cursor;
use crate::peg::Rule;
use crate::peg::error::ParseFailure;
use crate::peg::error::{Nope, ParseResult, Yeap};
use crate::trees::tree::Tree;
use crate::types::Str;
use crate::util::pyre::{Pattern, escape};
use std::fmt::Debug;

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
        Nope::new(start, self, &source)
    }

    fn furthest_failure(&self) -> Option<Nope>;
    fn set_furthest_failure(&mut self, nope: &Nope);

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

    fn dot(&mut self) -> bool {
        self.next()
    }

    fn next(&mut self) -> bool {
        self.cursor_mut().next().is_some()
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

    fn key(&mut self, name: &str, memo: bool) -> MemoKey {
        MemoCache::key(self.mark(), name, memo)
    }

    fn memo(&mut self, key: &MemoKey) -> Option<Memo>;

    fn memoize(&mut self, key: &MemoKey, tree: &Tree, lastmark: usize);

    fn clear_error_memos(&mut self);

    fn cut(&mut self); // Only cut() remains
    fn clear_cut(&mut self);

    fn prune_cache(&mut self);

    fn is_keyword(&self, name: &str) -> bool {
        let _ = name;
        false
    }
    fn set_keywords(&mut self, keywords: &[Str]) {
        let _ = keywords;
    }

    /// This is the merge in TatSu
    ///
    /// ```python
    /// def merge(self, prev: ParseState) -> Self:
    ///     self.ast = prev.ast
    ///     self.extend(prev.cst)
    ///     self.alerts.extend(prev.alerts)
    ///     self.cursor.goto(prev.cursor.pos)
    ///     return self
    /// ```
    // NOTE:
    //  * We don't construct the resulting CST/AST because Tree does it
    //    when Tree.node() is called on success of a rule call
    //  * Alerts are not implemented (no one knows what they're for)
    //  * We should reset our cursor. Copy? Just the mark? The only
    //    state kept by cursor is the mark.
    //  * Our `cutseen` remains as was
    //  * WARNING:
    //      Don't know what to do about the callstack
    //      All self.leave() does is pop it
    //      See CoreCtx for a reasonable override
    fn merge(self, other: Self) -> Self;

    // NOTE
    //  Default to .clone(), but implementors can do more work.
    //  This should work with both cloned Ctx and with a separate
    //  StateStack.
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
                    && let Tree::Text(name) = &tree
                    && ctx.is_keyword(name)
                {
                    ctx.memoize(&key, &Tree::Bottom, ctx.mark());
                    let error = ParseFailure::ReservedWord(name.clone());
                    ctx.tracer().trace_failure(&ctx, name);
                    return Err(ctx.failure(start, error));
                }
                ctx.tracer().trace_success(&ctx);
                ctx.memoize(&key, &tree, ctx.mark());
                Ok(Yeap(ctx, tree))
            }
            Err(mut nope) => {
                if rule.should_trace() {
                    self.leave();
                }
                self.tracer().trace_failure(&self, name);
                self.memoize(&key, &Tree::Bottom, self.mark());
                nope.take_cut();
                Err(nope)
            }
        }
    }

    fn do_call(mut self, name: &str, rule: &Rule) -> ParseResult<Self> {
        let start = self.mark();
        let key = self.key(name, rule.is_memoizable());
        if let Some(memo) = self.memo(&key) {
            return match memo.tree {
                Tree::Bottom => {
                    let err = ParseFailure::FailedParse(name.into());
                    Err(self.failure(start, err))
                }
                _ => {
                    self.reset(memo.mark);
                    Ok(Yeap(self, memo.tree))
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
            // Err(self.failure(
            //     key.mark,
            //     ParseFailure::UnboundLeftRecursion(depth, key.name.as_ref().into(), key.mark),
            // ))
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

        self.memoize(key, &Tree::Bottom, start);
        loop {
            // NOTE this is in TatSu and not in pegen
            //  self.clear_error_memos();
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
                    lasttree = endtree;
                    self = self.merge(ctx);
                    self.memoize(key, &lasttree, lastmark);
                }
            }
        }

        self.reset(lastmark);
        self.memoize(key, &lasttree, lastmark);

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
        Ok(Yeap(self, lasttree))
    }
}
