// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::*;
use crate::input::tokenizing::TokenizingPatterns;
use crate::types::Str;
use crate::util::pyre::Pattern;
use std::fmt::Debug;

/// A source location (filename, line, column).
pub struct Location {
    /// Source name.
    pub source: Str,
    /// (line, column) position.
    pub pos: (usize, usize),
}

/// Trait for input cursors that drive parsing.
pub trait Cursor: Debug + Configurable {
    fn input_source(&self) -> &str;
    fn mark(&self) -> usize;
    fn reset(&mut self, mark: usize);
    fn as_str(&self) -> &str;
    fn as_ref(&self) -> Str;
    fn ignore_case(&self) -> bool;
    fn name_guard(&self) -> bool;

    fn lookahead(&self, start: usize) -> &str {
        self.as_str()[start..].lines().next().unwrap_or("")
    }

    fn at_end(&self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
    fn peek_token(&mut self, token: &str) -> bool;
    fn is_name_char(&self, c: char) -> bool;
    fn is_name(&self, s: &str) -> bool;
    fn match_token(&mut self, token: &str) -> bool;
    fn match_pattern(&mut self, pattern: &Pattern) -> Option<String>;
    fn match_eol(&mut self) -> bool;
    fn next_token(&mut self);

    fn pos(&self) -> (usize, usize) {
        self.pos_at(self.mark())
    }

    fn pos_at(&self, mut mark: usize) -> (usize, usize) {
        mark = mark.min(self.as_str().len());
        let head = &self.as_str()[0..mark];
        let mut line = 1;
        let mut col = 0;
        for ch in head.chars() {
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
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

    // --- Meta expression matchers ---

    /// Match a name/identifier (`@name`). Returns `None` on no match.
    fn match_name(&mut self) -> Option<String>;
    /// Match a signed integer (`@int`). Returns `None` on no match.
    fn match_int(&mut self) -> Option<i64>;
    /// Match an unsigned integer (`@uint`). Returns `None` on no match.
    fn match_uint(&mut self) -> Option<u64>;
    /// Match a floating-point literal (`@float`). Returns `None` on no match.
    fn match_float(&mut self) -> Option<f64>;
    /// Match a boolean literal (`@bool`). Returns `None` on no match.
    fn match_bool(&mut self) -> Option<bool>;
}
