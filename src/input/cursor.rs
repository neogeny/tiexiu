// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::*;
use crate::input::tokenizing::TokenizingPatterns;
use crate::types::Str;
use crate::util::pyre::Pattern;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Location {
    pub source: Str,
    pub pos: (usize, usize),
}

pub trait Cursor: Debug + Configurable {
    fn input_source(&self) -> &str;
    fn mark(&self) -> usize;
    fn reset(&mut self, mark: usize);
    fn as_str(&self) -> &str;
    fn as_ref(&self) -> Rc<str>;
    fn ignore_case(&self) -> bool;
    fn name_guard(&self) -> bool;

    fn lookahead(&self, start: usize) -> &str {
        self.as_str()[start..].lines().next().unwrap_or("")
    }

    fn at_end(&self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
    fn peek_token(&mut self, token: &str) -> bool;
    fn match_token(&mut self, token: &str) -> bool;
    fn match_pattern(&mut self, pattern: &Pattern) -> Option<String>;
    fn match_eol(&mut self) -> bool;
    fn next_token(&mut self);

    fn pos(&self) -> (usize, usize) {
        self.pos_at(self.mark())
    }

    fn pos_at(&self, mut mark: usize) -> (usize, usize) {
        mark = mark.min(self.as_str().len());
        let text = self.as_str();
        let head = &text[0..mark];
        let line = head.lines().count();
        let col = head.lines().last().map_or(0, |l| l.chars().count());
        (line, col)
    }

    fn location(&self) -> Location {
        self.location_at(self.mark())
    }

    fn location_at(&self, mark: usize) -> Location {
        let pos = self.pos_at(mark);
        Location {
            source: self.input_source().into(),
            pos,
        }
    }

    fn set_patterns(&mut self, patterns: &TokenizingPatterns);
}
