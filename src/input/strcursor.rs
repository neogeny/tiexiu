// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Cursor;
use super::error::Error;
use super::tokenizing::TokenizingPatterns;
use crate::cfg::keys::config;
use crate::cfg::*;
use crate::util::newlines::{take_linebreak_len, take_non_newline_whitespace_len};
use crate::util::pyre::Pattern;
use std::rc::Rc;

/// Shared cursor configuration (CoW handle).
#[derive(Debug, Clone)]
pub struct CursorHeavy {
    ignorecase: bool,
    nameguard: bool,
    namechars: String,
    source: String,
    patterns: Rc<TokenizingPatterns>,
}

/// A cursor that parses an in-memory string.
#[derive(Debug, Clone)]
pub struct StrCursor {
    text: Rc<str>,
    offset: usize,
    heavy: Rc<CursorHeavy>,
}

impl From<&str> for StrCursor {
    #[inline]
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl StrCursor {
    /// Create a new `StrCursor` from a text string.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            offset: 0,
            heavy: CursorHeavy {
                ignorecase: false,
                nameguard: false,
                namechars: String::new(),
                source: "some input".into(),
                patterns: TokenizingPatterns::default().into(),
            }
            .into(),
        }
    }
    /// Create a cursor with a named source and optional start offset.
    pub fn from_source(source: &str, text: &str, mut start: usize) -> Self {
        start = start.min(text.len());
        while start < text.len() && !text.is_char_boundary(start) {
            start += 1;
        }
        Self {
            text: text.into(),
            offset: start.min(text.len()),
            heavy: CursorHeavy {
                ignorecase: false,
                nameguard: false,
                namechars: String::new(),
                source: source.into(),
                patterns: TokenizingPatterns::default().into(),
            }
            .into(),
        }
    }

    /// Create a cursor with custom tokenizing patterns.
    pub fn with_patterns(text: &str, patterns: TokenizingPatterns) -> Result<Self, Error> {
        Ok(Self {
            text: text.into(),
            offset: 0,
            heavy: CursorHeavy {
                ignorecase: false,
                nameguard: false,
                namechars: String::new(),
                source: "some input".into(),
                patterns: patterns.into(),
            }
            .into(),
        })
    }

    #[inline]
    fn eat_pattern(&mut self, pat: &Pattern) -> bool {
        if self.at_end() || pat.pattern().is_empty() {
            return false;
        }

        let text = &self.text[self.offset..];
        if let Some(mat) = pat.match_(text) {
            self.offset += mat.end(None) as usize;
            return true;
        }
        false
    }

    /// Consumes whitespace characters that are not newlines.
    pub fn eat_spaces_no_newlines(&mut self) {
        let mut p = usize::MAX;
        let eol = self.heavy.patterns.eol.clone();
        let cmt = self.heavy.patterns.cmt.clone();
        while self.offset != p {
            p = self.offset;

            self.offset += take_non_newline_whitespace_len(&self.text[self.offset..]);

            if self.eat_pattern(&eol) {
                self.offset += take_non_newline_whitespace_len(&self.text[self.offset..]);
            }

            self.eat_pattern(&cmt);
        }
    }
}

impl Configurable for StrCursor {
    fn configure(&mut self, cfg: &Cfg) {
        let cfg = config(cfg);

        let mut patterns = (*self.heavy.patterns).clone();
        patterns.configure(&cfg);

        let source = cfg
            .iter()
            .filter_map(|k| {
                if let CfgKey::Source(s) = k {
                    Some(s)
                } else {
                    None
                }
            })
            .next()
            .unwrap_or(&self.heavy.source);

        let namechars: String = cfg
            .iter()
            .filter_map(|k| {
                if let CfgKey::NameChars(s) = k {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .next()
            .unwrap_or_default();

        let nameguard = !cfg.contains(&CfgKey::NameGuard(false))
            && (cfg.contains(&CfgKey::NameGuard(true))
                || patterns.not_default && !patterns.wsp.pattern().is_empty()
                || !namechars.is_empty());
        self.heavy = CursorHeavy {
            ignorecase: cfg.contains(&CfgKey::IgnoreCase),
            nameguard,
            namechars,
            source: source.into(),
            patterns: patterns.into(),
        }
        .into()
    }
}

impl Cursor for StrCursor {
    fn input_source(&self) -> &str {
        self.heavy.source.as_str()
    }

    fn mark(&self) -> usize {
        self.offset
    }

    fn reset(&mut self, mark: usize) {
        self.offset = mark;
    }

    fn as_str(&self) -> &str {
        &self.text
    }
    fn as_ref(&self) -> Rc<str> {
        self.text.clone()
    }

    fn ignore_case(&self) -> bool {
        self.heavy.ignorecase
    }

    fn name_guard(&self) -> bool {
        self.heavy.nameguard
    }

    fn at_end(&self) -> bool {
        self.offset >= self.text.len()
    }

    fn next(&mut self) -> Option<char> {
        self.peek().inspect(|c| {
            self.offset += c.len_utf8();
        })
    }

    fn peek(&mut self) -> Option<char> {
        self.text.get(self.offset..)?.chars().next()
    }

    fn peek_token(&mut self, token: &str) -> bool {
        if let Some(text_slice) = self.text[self.offset..].get(..token.len())
            && (self.ignore_case() && text_slice.eq_ignore_ascii_case(token)
                || !self.ignore_case() && text_slice == token)
        {
            true
        } else {
            false
        }
    }

    fn is_name_char(&self, c: char) -> bool {
        c.is_alphanumeric() || self.heavy.namechars.contains(c)
    }

    fn is_name(&self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        if !first.is_alphabetic() && !self.heavy.namechars.contains(first) {
            return false;
        }
        chars.all(|c| self.is_name_char(c))
    }

    fn match_token(&mut self, token: &str) -> bool {
        if !self.peek_token(token) {
            return false;
        }
        let mark = self.offset;
        self.offset += token.len();
        if self.name_guard() && self.is_name(token) {
            if let Some(c) = self.text[self.offset..].chars().next() {
                if self.is_name_char(c) {
                    self.offset = mark;
                    return false;
                }
            }
        }
        true
    }

    fn match_pattern(&mut self, pat: &Pattern) -> Option<String> {
        let text = &self.text[self.offset..];
        let m = pat.match_(text)?;

        self.offset += m.end(None) as usize;
        m.group(1).or(m.group(0)).map(|s| s.to_string())
    }

    fn match_eol(&mut self) -> bool {
        let mark = self.offset;
        self.eat_spaces_no_newlines();

        // Look for the line terminator at the current position
        match take_linebreak_len(&self.text[self.offset..]) {
            Some(eol_len) => {
                self.offset += eol_len;
                self.eat_spaces_no_newlines();
                true
            }
            None => {
                // Backtrack if no line break is found
                self.offset = mark;
                false
            }
        }
    }

    fn next_token(&mut self) {
        let p = self.heavy.patterns.clone();

        let mut last_offset = usize::MAX;
        while self.offset != last_offset {
            last_offset = self.offset;

            self.eat_pattern(&p.wsp);

            if self.eat_pattern(&p.eol) {
                self.eat_pattern(&p.wsp);
            }

            self.eat_pattern(&p.cmt);

            if self.at_end() {
                break;
            }
        }
    }

    fn set_patterns(&mut self, patterns: &TokenizingPatterns) {
        self.heavy = CursorHeavy {
            ignorecase: self.heavy.ignorecase,
            nameguard: self.heavy.nameguard,
            namechars: self.heavy.namechars.clone(),
            source: self.heavy.source.clone(),
            patterns: patterns.clone().into(),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn whitespace_pattern_cannot_match_empty() {
        assert!(TokenizingPatterns::try_new(r"[ \t]*", "/* */", "//.*$").is_err());
    }

    #[test]
    fn comment_pattern_cannot_match_empty() {
        assert!(TokenizingPatterns::try_new(r"\s+", ".*", "//.*$").is_err());
    }

    #[test]
    fn eol_pattern_cannot_match_empty() {
        assert!(TokenizingPatterns::try_new(r"\s+", "/* */", r"\w?").is_err());
    }

    #[test]
    fn default_patterns_are_valid() -> Result<()> {
        let patterns = TokenizingPatterns::default();
        assert!(patterns.wsp.search("").is_none());
        assert!(patterns.cmt.search("").is_none());
        assert!(patterns.eol.search("").is_none());
        Ok(())
    }
}
