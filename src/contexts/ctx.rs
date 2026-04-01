// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::memo::{Cache, Key, Memo};
use crate::grammars::{ParseResult, Parser, S};
use crate::input::Cursor;
use std::fmt::Debug;

pub trait Ctx: Clone + Debug {
    fn cursor(&self) -> &dyn Cursor;
    
    fn cursor_mut(&mut self) -> &mut dyn Cursor;

    fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Cache) -> R;

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
        self.cursor_mut().token(token)
    }

    fn pattern(&mut self, pattern: &str) -> Option<&str> {
        self.cursor_mut().pattern(pattern)
    }

    fn next_token(&mut self) {
        self.cursor_mut().next_token();
    }

    fn key(&mut self, name: &str) -> Key {
        Cache::key(self.mark(), name)
    }
    fn mark(&self) -> usize {
        self.cursor().mark()
    }

    fn reset(&mut self, mark: usize) {
        self.cursor_mut().reset(mark);
    }

    fn memo(&mut self, key: &Key) -> Option<Memo> {
        self.with_cache_mut(|cache| cache.memo(key))
    }

    fn memoize(&mut self, key: &Key, cst: &Cst) {
        let mark = self.mark();
        self.with_cache_mut(|cache| {
            cache.memoize(key, cst, mark);
        });
    }


    fn cut_seen(&self) -> bool;
    fn cut(&mut self);
    fn uncut(&mut self);

    fn prune_cache(&mut self);

    fn parser_for(self, name: &str) -> (Self, &dyn Parser<Self>);

    fn call(mut self, name: &str) -> ParseResult<Self> {
        let key = self.key(name);

        if let Some(memo) = self.memo(&key) {
            return match memo.cst {
                Cst::Bottom => Err(self),
                _ => {
                    self.reset(memo.mark);
                    Ok(S(self, memo.cst))
                }
            };
        }

        let (ctx, rule) = self.parser_for(name);
        if rule.is_left_recursive() {
            return ctx.recursive_call(key, rule);
        }
        match rule.parse(ctx) {
            Ok(S(mut ctx, cst)) => {
                ctx.memoize(&key, &cst);
                Ok(S(ctx, cst))
            }
            Err(mut ctx) => {
                ctx.memoize(&key, &Cst::Bottom);
                Err(ctx)
            }
        }
    }

    fn recursive_call(mut self, key: Key, rule: &dyn Parser<Self>) -> ParseResult<Self> {
        if !rule.is_left_recursive() {
            panic!("Recursive call on non-LRec rule");
        }

        self.memoize(&key, &Cst::Bottom);
        let start_mark = self.mark();
        let mut best_cst: Option<Cst> = None;
        let mut high_water_mark = start_mark;

        loop {
            let mut ctx = self.clone();
            ctx.reset(start_mark);

            match rule.parse(ctx) {
                Err(_) => break,
                Ok(S(mut ctx, cst)) => {
                    let mark = ctx.mark();
                    if mark < high_water_mark {
                        break;
                    }

                    ctx.memoize(&key, &cst);
                    high_water_mark = mark;
                    best_cst = Some(cst);
                    self = ctx;
                }
            }
        }

        if let Some(cst) = best_cst {
            Ok(S(self, cst))
        } else {
            Err(self)
        }
    }
}
