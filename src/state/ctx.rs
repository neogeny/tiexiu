// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::memo::{Key, Memo, MemoCache};
use crate::input::Cursor;
use crate::peg::error::ParseError;
use crate::peg::{Fail, ParseResult, Rule, Succ};
use crate::trees::tree::Tree;
use crate::util::pyre::Pattern as Regex;
use crate::util::tokenlist::TokenList;
use std::fmt::Debug;

pub trait Ctx: Clone + Debug {
    fn cursor(&self) -> &dyn Cursor;

    fn cursor_mut(&mut self) -> &mut dyn Cursor;

    fn stack(&self) -> TokenList;

    fn enter(&mut self, name: &str);

    fn failure(&self, start: usize, error: ParseError) -> Fail {
        Fail::new(start, self.mark(), self.cut_seen(), error, self.stack())
    }

    fn eof_check(&mut self) -> bool {
        self.cursor().at_end()
    }

    fn dot(&mut self) -> bool {
        self.next().is_some()
    }

    fn next(&mut self) -> Option<char> {
        self.cursor_mut().next()
    }

    fn token(&mut self, token: &str) -> bool {
        self.next_token();
        self.cursor_mut().token(token)
    }

    fn regex(&self, pattern: &str) -> Regex;

    fn pattern(&mut self, pattern: &str) -> Option<String> {
        self.next_token();
        let re = self.regex(pattern);
        self.cursor_mut().pattern_re(&re)
    }

    fn next_token(&mut self) {
        self.cursor_mut().next_token();
    }

    fn key(&mut self, name: &str) -> Key {
        MemoCache::key(self.mark(), name)
    }
    fn mark(&self) -> usize {
        self.cursor().mark()
    }

    fn reset(&mut self, mark: usize) {
        self.cursor_mut().reset(mark);
    }

    fn memo(&mut self, key: &Key) -> Option<Memo>;

    fn memoize(&mut self, key: &Key, tree: &Tree);

    fn cut_seen(&self) -> bool;
    fn uncut(&mut self);
    fn cut(&mut self);

    fn prune_cache(&mut self);

    fn call_rule(mut self, name: &str, rule: &Rule) -> ParseResult<Self> {
        let start = self.mark();
        // TODO: self.tracer.trace_entry(self.cursor)
        self.enter(name);

        if !rule.is_token() {
            self.next_token();
        }

        let key = self.key(name);
        if let Some(memo) = self.memo(&key) {
            return match memo.tree {
                // TODO: self.tracer.trace_failure(self.cursor, e)
                Tree::Bottom => Err(self.failure(start, ParseError::FailedParse(name.into()))),
                _ => {
                    self.reset(memo.mark);
                    Ok(Succ(self, memo.tree))
                }
            };
        }

        if rule.is_left_recursive() {
            self.call_recursive(key, rule)
        } else {
            match rule.parse(self.clone()) {
                Ok(Succ(mut ctx, tree)) => {
                    // TODO: self.tracer.trace_success(self.cursor)
                    ctx.memoize(&key, &tree);
                    Ok(Succ(ctx, tree))
                }
                Err(f) => {
                    // TODO: self.tracer.trace_failure(self.cursor, e)
                    self.memoize(&key, &Tree::Bottom);
                    Err(f)
                }
            }
        }
    }

    fn call_recursive(mut self, key: Key, rule: &Rule) -> ParseResult<Self> {
        // TODO: self.tracer.trace_recursion(self.cursor)
        if !rule.is_left_recursive() {
            panic!("Recursive call on non-LRec rule");
        }

        self.memoize(&key, &Tree::Bottom);
        let start_mark = self.mark();
        let mut best_cst: Option<Tree> = None;
        let mut high_water_mark = start_mark;
        let mut last_failure: Option<Fail> = None;

        loop {
            let mut ctx = self.clone();
            ctx.reset(start_mark);

            match rule.parse(ctx) {
                Err(f) => {
                    last_failure = Some(f);
                    break;
                }
                Ok(Succ(mut ctx, tree)) => {
                    let mark = ctx.mark();
                    if mark < high_water_mark {
                        break;
                    }

                    ctx.memoize(&key, &tree);
                    high_water_mark = mark;
                    best_cst = Some(tree);
                    self = ctx;
                }
            }
        }

        if let Some(tree) = best_cst {
            // TODO: self.tracer.trace_success(self.cursor)
            Ok(Succ(self, tree))
        } else {
            // TODO: self.tracer.trace_failure(self.cursor, e)
            Err(last_failure.unwrap_or_else(|| {
                self.failure(start_mark, ParseError::FailedParse(rule.meta.name.clone()))
            }))
        }
    }
}
