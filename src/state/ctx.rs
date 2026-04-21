// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::memo::{Key, Memo, MemoCache};
use crate::cfg::Configurable;
use crate::input::Cursor;
use crate::peg::error::ParseError;
use crate::peg::{Nope, ParseResult, Rule, Succ};
use crate::state::trace::Tracer;
use crate::trees::tree::Tree;
use crate::util::pyre::{Pattern, escape};
use crate::util::tokenlist::TokenList;
use std::fmt::Debug;

pub trait CtxI: Configurable {
    fn cursor(&self) -> &dyn Cursor;
    fn callstack(&self) -> TokenList;
    fn mark(&self) -> usize {
        self.cursor().mark()
    }
    fn cut_seen(&self) -> bool;
}

pub trait Ctx: CtxI + Clone + Debug {
    fn cursor_mut(&mut self) -> &mut dyn Cursor;
    fn enter(&mut self, name: &str);
    fn leave(&mut self);
    fn tracer(&self) -> &dyn Tracer;

    #[track_caller]
    fn failure(&self, start: usize, source: ParseError) -> Nope {
        Nope::new(start, self, source)
    }

    fn reset(&mut self, mark: usize) {
        self.cursor_mut().reset(mark);
    }

    fn at_end(&mut self) -> bool {
        self.cursor().at_end()
    }
    fn eof_check(&mut self) -> bool {
        self.next_token();
        self.cursor().at_end()
    }

    fn dot(&mut self) -> bool {
        self.next().is_some()
    }

    fn next(&mut self) -> Option<char> {
        self.cursor_mut().next()
    }

    fn get_pattern(&self, pattern: &str) -> Pattern;

    fn match_token(&mut self, token: &str) -> bool {
        // WARNING: this may belong in Cursor, but the Ctx chain holds the regex caching
        self.next_token();
        let result = {
            let wordlike = token.chars().all(|c| c.is_alphanumeric());
            let escaped = escape(token);
            if wordlike && *escaped == *token {
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
            self.tracer().trace_no_match(self, token);
        }
        result
    }

    fn match_pattern(&mut self, pattern: &str) -> Option<String> {
        let re = self.get_pattern(pattern);
        let result = self.cursor_mut().match_pattern(&re);
        if let Some(matched) = result {
            self.tracer().trace_match(self, matched.as_str(), pattern);
            Some(matched)
        } else {
            self.tracer().trace_no_match(self, pattern);
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

    fn key(&mut self, name: &str) -> Key {
        MemoCache::key(self.mark(), name)
    }

    fn memo(&mut self, key: &Key) -> Option<Memo>;

    fn memoize(&mut self, key: &Key, tree: &Tree);

    fn set_cut_seen(&mut self);
    fn unset_cut(&mut self);

    fn cut(&mut self) {
        self.set_cut_seen();
        self.prune_cache();
    }

    fn restore_if_was_cut(&mut self, was_cut: bool) {
        if was_cut {
            self.set_cut_seen();
        } else {
            // NOTE: Do nothinig. A cut may have been set by the caller.
        }
    }
    fn prune_cache(&mut self);

    fn is_keyword(&self, name: &str) -> bool {
        let _ = name;
        false
    }
    fn set_keywords(&mut self, keywords: &[Box<str>]) {
        let _ = keywords;
    }

    fn call(mut self, name: &str, rule: &Rule) -> ParseResult<Self> {
        let start = self.mark();
        self.enter(name);
        self.tracer().trace_entry(&self);

        if !rule.is_token() {
            self.next_token();
        }

        let key = self.key(name);
        if let Some(memo) = self.memo(&key) {
            return match memo.tree {
                Tree::Bottom => {
                    let err = ParseError::FailedParse(name.into());
                    self.tracer().trace_failure(&self, &err);
                    self.leave();
                    Err(self.failure(start, err))
                }
                _ => {
                    self.reset(memo.mark);
                    self.tracer().trace_success(&self);
                    self.leave();
                    Ok(Succ(self, memo.tree))
                }
            };
        }

        let doppelganger = self.clone();
        match if rule.is_left_recursive() {
            doppelganger.call_recursive(&key, rule)
        } else {
            rule.parse(doppelganger)
        } {
            Ok(Succ(mut neogenus, tree)) => {
                neogenus.leave();
                if rule.is_name()
                    && let Tree::Text(name) = &tree
                    && self.is_keyword(name)
                {
                    self.memoize(&key, &Tree::Bottom);
                    let error = ParseError::ReservedWord(name.clone());
                    self.tracer().trace_failure(&self, &error);
                    return Err(self.failure(start, error));
                }
                neogenus.tracer().trace_success(&neogenus);
                neogenus.memoize(&key, &tree);
                Ok(Succ(neogenus, tree))
            }
            Err(nope) => {
                self.tracer().trace_failure(&self, &nope.source);
                self.memoize(&key, &Tree::Bottom);
                Err(nope)
            }
        }
    }

    fn call_recursive(mut self, key: &Key, rule: &Rule) -> ParseResult<Self> {
        self.tracer().trace_recursion(&self);
        if !rule.is_left_recursive() {
            panic!("Recursive call on non-LRec rule");
        }

        self.memoize(key, &Tree::Bottom);
        let start_mark = self.mark();
        let mut best_cst: Option<Tree> = None;
        let mut high_water_mark = start_mark;
        let mut last_failure: Option<Nope> = None;

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

                    ctx.memoize(key, &tree);
                    high_water_mark = mark;
                    best_cst = Some(tree);
                    self = ctx;
                }
            }
        }

        if let Some(tree) = best_cst {
            self.tracer().trace_success(&self);
            Ok(Succ(self, tree))
        } else {
            let nope = last_failure.unwrap_or_else(|| {
                self.failure(start_mark, ParseError::FailedParse(rule.name.clone()))
            });
            self.tracer().trace_failure(&self, &nope.source);
            Err(nope)
        }
    }
}
