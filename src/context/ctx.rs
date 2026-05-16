// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::memo::{Memo, MemoKey};
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

pub const MAX_RECURSION_DEPTH: usize = 64;

#[derive(Debug, Clone, PartialEq)]
pub struct Snap {
    pub mark: usize,
    pub cutseen: bool,
}

impl From<&dyn CtxI> for Snap {
    fn from(ctx: &dyn CtxI) -> Self {
        ctx.click()
    }
}

impl<C: Ctx> From<C> for Snap {
    fn from(ctx: C) -> Self {
        ctx.click()
    }
}

impl Snap {
    pub fn cut_seen(&self) -> bool {
        self.cutseen
    }

    pub fn set_cut(&mut self) {
        self.cutseen = true;
    }

    pub fn or_cut(&mut self, value: bool) {
        self.cutseen |= value;
    }

    pub fn take_cut(&mut self) -> bool {
        let cutseen = self.cutseen;
        self.cutseen = false;
        cutseen
    }
}

pub trait CtxI: Configurable {
    fn cursor(&self) -> &dyn Cursor;
    fn callstack(&self) -> CallStack;
    fn mark(&self) -> usize {
        self.cursor().mark()
    }
    fn _cut_seen(&self) -> bool;

    fn click(&self) -> Snap {
        Snap {
            mark: self.mark(),
            cutseen: false,
        }
    }
}

pub trait Ctx: CtxI + Clone + Debug {
    fn id(&self) -> usize {
        self as *const Self as usize
    }

    fn cursor_mut(&mut self) -> &mut dyn Cursor;
    fn enter(&mut self, name: &str);
    fn leave(&mut self);
    fn untrack(&mut self, key: &MemoKey) -> usize;
    fn tracer(&self) -> &dyn Tracer;

    /// Checks recursion depth to prevent stack overflow.
    fn track(&mut self, key: &MemoKey) -> usize;
    fn track_recursion_depth(&mut self, key: &MemoKey) -> Result<(), Nope> {
        let depth = self.track(key);
        if depth > MAX_RECURSION_DEPTH {
            panic!("Recursion depth exceeded")
        } else {
            Ok(())
        }
    }

    fn intern(&mut self, s: &str) -> Str {
        s.into()
    }

    #[track_caller]
    fn failure(&mut self, start: usize, source: ParseFailure) -> Nope {
        let nope = Nope::new(false);
        self.cursor_mut().reset(start);
        if let Some(furthest) = self.furthest_failure()
            && furthest.start() >= start
        {
            return nope;
        }

        let dis = DisasterReport::new(start, false, self, &source);
        self.set_furthest_failure(dis.clone());
        nope
    }

    fn set_furthest_failure(&mut self, dis: DisasterReport);
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
            let m = matched.as_str();
            self.tracer().trace_match(self, m, pattern);
            Some(self.intern(m))
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

    fn key(&mut self, name: &str, can_memo: bool) -> MemoKey;

    fn memo(&mut self, key: &MemoKey) -> Option<Memo>;

    fn memoize(&mut self, key: &MemoKey, tree: &Rc<Tree>, lastmark: usize);

    fn clear_error_memos(&mut self);

    fn _cut(&mut self);
    fn _clear_cut(&mut self);

    fn prune_cache(&mut self);

    fn is_keyword(&self, name: &str) -> bool {
        let _ = name;
        false
    }
    fn set_keywords(&mut self, keywords: &[Str]) {
        let _ = keywords;
    }

    fn merge(&mut self, snap: &Snap) {
        self.reset(snap.mark);
    }

    fn push(&self) -> Self {
        let mut new = self.clone();
        // FIXME
        new._clear_cut();
        new
    }
}
