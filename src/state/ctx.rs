// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::memo::{Key, Memo, MemoCache};
use crate::input::Cursor;
use crate::peg::{F, Grammar, ParseResult, Rule, S};
use crate::trees::tree::Tree;
use regex::Regex;
use std::fmt::Debug;

pub trait Ctx: Clone + Debug {
    fn grammar(&self) -> &Grammar;

    fn cursor(&self) -> &dyn Cursor;

    fn cursor_mut(&mut self) -> &mut dyn Cursor;

    fn failure(&self, msg: &str) -> F {
        F {
            mark: self.mark(),
            msg: msg.into(),
            cut: self.cut_seen(),
        }
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
        // NOTE: no next_token() here
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

    fn parser_for(&self, name: &str) -> Rule {
        let rule = self
            .grammar()
            .rulemap
            .get(name)
            .unwrap_or_else(|| panic!("rule '{}' not found", name));
        rule.clone()
    }

    fn call(mut self, name: &str) -> ParseResult<Self> {
        let rule = self.parser_for(name);

        if !rule.is_token() {
            self.next_token();
        }

        let key = self.key(name);
        if let Some(memo) = self.memo(&key) {
            return match memo.tree {
                Tree::Bottom => Err(self.failure(name)),
                _ => {
                    self.reset(memo.mark);
                    Ok(S(self, memo.tree))
                }
            };
        }

        if rule.is_left_recursive() {
            return self.recursive_call(key, &rule);
        }
        match rule.parse(self.clone()) {
            Ok(S(mut ctx, tree)) => {
                ctx.memoize(&key, &tree);
                Ok(S(ctx, tree))
            }
            Err(f) => {
                self.memoize(&key, &Tree::Bottom);
                Err(f)
            }
        }
    }

    fn recursive_call(mut self, key: Key, rule: &Rule) -> ParseResult<Self> {
        if !rule.is_left_recursive() {
            panic!("Recursive call on non-LRec rule");
        }

        self.memoize(&key, &Tree::Bottom);
        let start_mark = self.mark();
        let mut best_cst: Option<Tree> = None;
        let mut high_water_mark = start_mark;
        let mut last_failure: Option<F> = None;

        loop {
            let mut ctx = self.clone();
            ctx.reset(start_mark);

            match rule.parse(ctx) {
                Err(f) => {
                    last_failure = Some(f);
                    break;
                }
                Ok(S(mut ctx, tree)) => {
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
            Ok(S(self, tree))
        } else {
            Err(last_failure.unwrap_or_else(|| self.failure(&rule.info.name)))
        }
    }
}
