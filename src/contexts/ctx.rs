// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::memo::{Cache, Key, Memo};
use crate::grammars::{ParseResult, Rule};
use std::fmt::Debug;

pub trait Ctx: Clone + Debug {
    fn eof_check(&self) -> bool;
    fn dot(&mut self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn token(&mut self, token: &str) -> bool;
    fn pattern(&mut self, pattern: &str) -> Option<&str>;
    fn next_token(&mut self);

    fn key(&self, name: &str) -> Key {
        Cache::key(self.mark(), name)
    }
    fn mark(&self) -> usize;
    fn reset(&mut self, mark: usize);
    fn memo(&mut self, key: &Key) -> Option<Memo>;
    fn memoize(&mut self, key: &Key, cst: &Cst);

    fn cut(&mut self);
    fn uncut(&mut self);
    fn cut_seen(&self) -> bool;

    fn parser(&self, name: &str) -> (Self, &Rule<'_>);

    fn call(mut self, name: &str) -> ParseResult<Self> {
        let key = self.key(name);

        if let Some(memo) = self.memo(&key) {
            return match memo.cst {
                Cst::Bottom => Err(self),
                _ => {
                    self.reset(memo.mark);
                    Ok((self, memo.cst))
                }
            };
        }

        let (ctx, rule) = self.parser(name);
        if rule.is_lrec {
            return ctx.recursive_call(rule);
        }

        match rule.parse(ctx) {
            Ok((mut ctx, cst)) => {
                ctx.memoize(&key, &cst);
                Ok((ctx, cst))
            }
            Err(mut ctx) => {
                ctx.memoize(&key, &Cst::Bottom);
                Err(ctx)
            }
        }
    }

    fn recursive_call(mut self, rule: &Rule) -> ParseResult<Self> {
        if !rule.is_lrec {
            panic!("Recursive call on non-LRec rule {}", rule.name);
        }

        let key = self.key(rule.name);
        let start_mark = self.mark();
        self.memoize(&key, &Cst::Bottom);

        let mut best_cst: Option<Cst> = None;
        let mut high_water_mark = start_mark;

        loop {
            let mut ctx = self.clone();
            ctx.reset(start_mark);

            match rule.parse(ctx) {
                Err(_) => break,
                Ok((mut ctx, cst)) => {
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
            Ok((self, cst))
        } else {
            Err(self)
        }
    }
}
