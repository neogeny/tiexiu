// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::memo::{Memo, MemoKey};
use crate::cfg::Configurable;
use crate::context::state::CallStack;
use crate::context::trace::Tracer;
use crate::input::Cursor;
use crate::peg::error::Nope;
use crate::peg::error::ParseResult;
use crate::peg::error::{DisasterReport, ParseFailure};
use crate::trees::Tree;
use crate::trees::TreeRef;
use crate::types::Str;
use crate::util::pyre::Pattern;
use crate::{MAX_RECURSION_DEPTH, SYM_ETX};
use std::fmt::Debug;

/// Immutable context interface for reading parser state.
pub trait Ctx: Configurable {
    fn cursor(&self) -> &dyn Cursor;
    fn callstack(&self) -> CallStack;
    fn mark(&self) -> usize {
        self.cursor().mark()
    }
    fn cut_seen(&self) -> bool;
}

/// Mutable context interface for parsing operations.
pub trait CtxSem: Ctx + Debug + Sized {
    fn cursor_mut(&mut self) -> &mut dyn Cursor;
    fn enter(&mut self, name: &str);
    fn leave(&mut self);
    fn untrack(&mut self, key: &MemoKey) -> usize;
    fn tracer(&self) -> &dyn Tracer;

    fn enter_lookahead(&mut self);
    fn leave_lookahead(&mut self);

    /// Checks recursion depth to prevent stack overflow.
    fn track(&mut self, key: &MemoKey) -> usize;
    fn track_recursion_depth(&mut self, key: &MemoKey) -> Result<(), Nope> {
        let depth = self.track(key);
        if depth > MAX_RECURSION_DEPTH {
            Err(self.failure(self.mark(), ParseFailure::RecursionDepthExceeded))
        } else {
            Ok(())
        }
    }

    fn intern(&mut self, s: &str) -> Str {
        s.into()
    }

    #[track_caller]
    fn failure(&mut self, start: usize, source: ParseFailure) -> Nope {
        let dis = DisasterReport::new(start, false, self, &source);
        let furthest = self.furthest_failure();
        if furthest.is_none() || furthest.is_some_and(|f| f.mark() <= self.mark()) {
            self.set_furthest_failure(dis.clone());
        }
        Nope { report: dis.into() }
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

    fn next(&mut self) -> Option<char> {
        self.cursor_mut().next()
    }

    fn get_pattern(&mut self, pattern: &str) -> Pattern;

    fn match_token(&mut self, token: &str) -> bool {
        self.next_token();
        let result = self.cursor_mut().match_token(token);
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

    /// Apply semantics to a parsed rule tree.
    ///
    /// Default implementation returns `Ok(Tree::Bottom)`, meaning "not handled"
    /// — the caller should proceed with default param-based `Node` wrapping.
    fn apply_semantics(
        &mut self,
        _node: TreeRef,
        _rule_name: &str,
        _params: &[Str],
    ) -> ParseResult {
        Ok(Tree::Bottom.into())
    }

    fn key(&mut self, name: &str, can_memo: bool) -> MemoKey;
    fn memo(&mut self, key: &MemoKey) -> Option<Memo>;
    fn memoize(&mut self, key: &MemoKey, tree: &TreeRef, lastmark: usize);

    fn cut(&mut self);
    fn push_cut(&mut self);
    fn take_cut(&mut self) -> bool;

    fn is_keyword(&self, name: &str) -> bool {
        let _ = name;
        false
    }
    fn set_keywords(&mut self, keywords: &[Str]) {
        let _ = keywords;
    }

    // --- Meta expression matchers ---

    fn match_name(&mut self) -> Option<Str> {
        self.next_token();
        let result = self.cursor_mut().match_name();
        if let Some(ref name) = result {
            self.tracer().trace_match(self, name, "@name");
            Some(self.intern(name))
        } else {
            self.tracer().trace_no_match(self, "", "@name");
            None
        }
    }

    fn match_int(&mut self) -> Option<Str> {
        self.next_token();
        let result = self.cursor_mut().match_int();
        if let Some(n) = result {
            let s = self.intern(&n.to_string());
            self.tracer().trace_match(self, &s, "@int");
            Some(s)
        } else {
            self.tracer().trace_no_match(self, "", "@int");
            None
        }
    }

    fn match_uint(&mut self) -> Option<Str> {
        self.next_token();
        let result = self.cursor_mut().match_uint();
        if let Some(n) = result {
            let s = self.intern(&n.to_string());
            self.tracer().trace_match(self, &s, "@uint");
            Some(s)
        } else {
            self.tracer().trace_no_match(self, "", "@uint");
            None
        }
    }

    fn match_float(&mut self) -> Option<Str> {
        self.next_token();
        let result = self.cursor_mut().match_float();
        if let Some(f) = result {
            let s = self.intern(&f.to_string());
            self.tracer().trace_match(self, &s, "@float");
            Some(s)
        } else {
            self.tracer().trace_no_match(self, "", "@float");
            None
        }
    }

    fn match_bool(&mut self) -> Option<Str> {
        self.next_token();
        let result = self.cursor_mut().match_bool();
        if let Some(b) = result {
            let s = self.intern(&b.to_string());
            self.tracer().trace_match(self, &s, "@bool");
            Some(s)
        } else {
            self.tracer().trace_no_match(self, "", "@bool");
            None
        }
    }
}
