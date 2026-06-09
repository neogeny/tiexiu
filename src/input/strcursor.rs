// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Cursor;
use super::error::Error;
use super::tokenizing::TokenizingPatterns;
use crate::cfg::keys::config;
use crate::cfg::*;
use crate::types::{Ref, Str};
use crate::util::newlines::{take_linebreak_len, take_non_newline_whitespace_len};
use crate::util::pyre::Pattern;

/// Shared cursor configuration (CoW handle).
#[derive(Debug, Clone)]
pub struct CursorHeavy {
    ignorecase: bool,
    nameguard: bool,
    namechars: String,
    source: String,
    patterns: Ref<TokenizingPatterns>,
}

/// A cursor that parses an in-memory string.
#[derive(Debug, Clone)]
pub struct StrCursor {
    text: Str,
    offset: usize,
    heavy: Ref<CursorHeavy>,
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
    fn as_ref(&self) -> Str {
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

    fn match_name(&mut self) -> Option<String> {
        let start = self.offset;
        let mut chars = self.text[self.offset..].chars();
        let first = chars.next()?;
        if !first.is_alphabetic() && first != '_' && !self.heavy.namechars.contains(first) {
            return None;
        }
        self.offset += first.len_utf8();
        loop {
            match self.text[self.offset..].chars().next() {
                Some(c) if c.is_alphanumeric() || c == '_' || self.heavy.namechars.contains(c) => {
                    self.offset += c.len_utf8();
                }
                _ => break,
            }
        }
        Some(self.text[start..self.offset].to_string())
    }

    fn match_int(&mut self) -> Option<i64> {
        let start = self.offset;
        // optional sign
        match self.text[self.offset..].chars().next() {
            Some('+') | Some('-') => {
                self.offset += 1;
            }
            _ => {}
        }
        // at least one digit (with optional internal underscores)
        let mut has_digit = false;
        while let Some(c) = self.text[self.offset..].chars().next() {
            if c.is_ascii_digit() {
                has_digit = true;
                self.offset += c.len_utf8();
            } else if c == '_' {
                self.offset += c.len_utf8();
            } else {
                break;
            }
        }
        if !has_digit {
            self.offset = start;
            return None;
        }
        let s: String = self.text[start..self.offset]
            .chars()
            .filter(|c| *c != '_')
            .collect();
        s.parse::<i64>().ok().or_else(|| {
            self.offset = start;
            None
        })
    }

    fn match_uint(&mut self) -> Option<u64> {
        let start = self.offset;
        // at least one digit (with optional internal underscores)
        let mut has_digit = false;
        while let Some(c) = self.text[self.offset..].chars().next() {
            if c.is_ascii_digit() {
                has_digit = true;
                self.offset += c.len_utf8();
            } else if c == '_' {
                self.offset += c.len_utf8();
            } else {
                break;
            }
        }
        if !has_digit {
            return None;
        }
        let s: String = self.text[start..self.offset]
            .chars()
            .filter(|c| *c != '_')
            .collect();
        s.parse::<u64>().ok().or_else(|| {
            self.offset = start;
            None
        })
    }

    fn match_float(&mut self) -> Option<f64> {
        let start = self.offset;
        // optional sign
        match self.text[self.offset..].chars().next() {
            Some('+') | Some('-') => {
                self.offset += 1;
            }
            _ => {}
        }
        // digits before decimal point (optional)
        let mut has_digit = false;
        while let Some(c) = self.text[self.offset..].chars().next() {
            if c.is_ascii_digit() {
                has_digit = true;
                self.offset += c.len_utf8();
            } else {
                break;
            }
        }
        // optional decimal point and digits
        if self.text[self.offset..].starts_with('.') {
            self.offset += 1;
            while let Some(c) = self.text[self.offset..].chars().next() {
                if c.is_ascii_digit() {
                    has_digit = true;
                    self.offset += c.len_utf8();
                } else {
                    break;
                }
            }
        }
        if !has_digit {
            self.offset = start;
            return None;
        }
        // optional exponent
        if let Some(c) = self.text[self.offset..].chars().next() {
            if c == 'e' || c == 'E' {
                self.offset += c.len_utf8();
                // optional sign after e/E
                match self.text[self.offset..].chars().next() {
                    Some('+') | Some('-') => {
                        self.offset += 1;
                    }
                    _ => {}
                }
                // at least one digit in exponent
                let exp_start = self.offset;
                while let Some(d) = self.text[self.offset..].chars().next() {
                    if d.is_ascii_digit() {
                        self.offset += d.len_utf8();
                    } else {
                        break;
                    }
                }
                if self.offset == exp_start {
                    // no exponent digits - backtrack
                    self.offset = start;
                    return None;
                }
            }
        }
        let s = &self.text[start..self.offset];
        s.parse::<f64>().ok().or_else(|| {
            self.offset = start;
            None
        })
    }

    fn match_bool(&mut self) -> Option<bool> {
        let start = self.offset;
        let rest = &self.text[self.offset..];
        let result = if rest.starts_with("true") {
            self.offset += 4;
            Some(true)
        } else if rest.starts_with("false") {
            self.offset += 5;
            Some(false)
        } else if rest.starts_with("True") {
            self.offset += 4;
            Some(true)
        } else if rest.starts_with("False") {
            self.offset += 5;
            Some(false)
        } else {
            None
        };
        if result.is_none() {
            self.offset = start;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use crate::cfg::Cfg;

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

    // --- Meta expression cursor tests ---

    #[test]
    fn test_match_name_simple() {
        let mut c = StrCursor::new("hello_world");
        assert_eq!(c.match_name().as_deref(), Some("hello_world"));
        assert_eq!(c.offset, 11);
    }

    #[test]
    fn test_match_name_with_underscore() {
        let mut c = StrCursor::new("_foo");
        assert_eq!(c.match_name().as_deref(), Some("_foo"));
        assert_eq!(c.offset, 4);
    }

    #[test]
    fn test_match_name_stops_at_non_name() {
        let mut c = StrCursor::new("abc123 xyz");
        assert_eq!(c.match_name().as_deref(), Some("abc123"));
        assert_eq!(c.offset, 6);
    }

    #[test]
    fn test_match_name_fail_on_digit_start() {
        let mut c = StrCursor::new("123abc");
        assert!(c.match_name().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_name_fail_on_symbol() {
        let mut c = StrCursor::new("@foo");
        assert!(c.match_name().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_name_empty() {
        let mut c = StrCursor::new("");
        assert!(c.match_name().is_none());
    }

    #[test]
    fn test_match_name_with_namechars() {
        let mut c = StrCursor::new("foo-bar");
        c.configure(&Cfg::new(&[CfgKey::NameChars("-".into())]));
        assert_eq!(c.match_name().as_deref(), Some("foo-bar"));
    }

    #[test]
    fn test_match_int_positive() {
        let mut c = StrCursor::new("42");
        assert_eq!(c.match_int(), Some(42));
    }

    #[test]
    fn test_match_int_negative() {
        let mut c = StrCursor::new("-17");
        assert_eq!(c.match_int(), Some(-17));
    }

    #[test]
    fn test_match_int_with_plus() {
        let mut c = StrCursor::new("+99");
        assert_eq!(c.match_int(), Some(99));
    }

    #[test]
    fn test_match_int_with_underscore() {
        let mut c = StrCursor::new("1_234");
        assert_eq!(c.match_int(), Some(1234));
    }

    #[test]
    fn test_match_int_fail_on_alpha() {
        let mut c = StrCursor::new("abc");
        assert!(c.match_int().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_int_fail_on_float() {
        let mut c = StrCursor::new("3.14");
        assert_eq!(c.match_int(), Some(3));
    }

    #[test]
    fn test_match_int_negative_fail() {
        let mut c = StrCursor::new("-");
        assert!(c.match_int().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_uint_simple() {
        let mut c = StrCursor::new("42");
        assert_eq!(c.match_uint(), Some(42));
    }

    #[test]
    fn test_match_uint_with_underscore() {
        let mut c = StrCursor::new("5_000");
        assert_eq!(c.match_uint(), Some(5000));
    }

    #[test]
    fn test_match_uint_fail_on_negative() {
        let mut c = StrCursor::new("-1");
        assert!(c.match_uint().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_uint_fail_on_alpha() {
        let mut c = StrCursor::new("abc");
        assert!(c.match_uint().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_uint_zero() {
        let mut c = StrCursor::new("0");
        assert_eq!(c.match_uint(), Some(0));
    }

    #[test]
    fn test_match_uint_empty() {
        let mut c = StrCursor::new("");
        assert!(c.match_uint().is_none());
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_match_float_simple() {
        let mut c = StrCursor::new("3.14");
        let result = c.match_float();
        assert!(result.is_some());
        let val = result.unwrap();
        assert!((val - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_match_float_negative() {
        let mut c = StrCursor::new("-2.5");
        assert_eq!(c.match_float(), Some(-2.5));
    }

    #[test]
    fn test_match_float_exponent() {
        let mut c = StrCursor::new("1.5e-2");
        assert_eq!(c.match_float(), Some(0.015));
    }

    #[test]
    fn test_match_float_exponent_upper() {
        let mut c = StrCursor::new("3E+10");
        assert!(c.match_float().is_some());
    }

    #[test]
    fn test_match_float_no_fraction() {
        let mut c = StrCursor::new("42.");
        let result = c.match_float();
        assert!(result.is_some(), "42. should match as float");
        assert!((result.unwrap() - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_match_float_fail_on_alpha() {
        let mut c = StrCursor::new("abc");
        assert!(c.match_float().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_float_integer_only() {
        let mut c = StrCursor::new("42");
        // floats match integers too (integer is a valid float)
        assert_eq!(c.match_float(), Some(42.0));
    }

    #[test]
    fn test_match_bool_true() {
        let mut c = StrCursor::new("true");
        assert_eq!(c.match_bool(), Some(true));
    }

    #[test]
    fn test_match_bool_false() {
        let mut c = StrCursor::new("false");
        assert_eq!(c.match_bool(), Some(false));
    }

    #[test]
    fn test_match_bool_true_capitalized() {
        let mut c = StrCursor::new("True");
        assert_eq!(c.match_bool(), Some(true));
    }

    #[test]
    fn test_match_bool_false_capitalized() {
        let mut c = StrCursor::new("False");
        assert_eq!(c.match_bool(), Some(false));
    }

    #[test]
    fn test_match_bool_fail_on_wrong_case() {
        let mut c = StrCursor::new("TRUE");
        assert!(c.match_bool().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_bool_fail_on_random() {
        let mut c = StrCursor::new("xyz");
        assert!(c.match_bool().is_none());
        assert_eq!(c.offset, 0);
    }

    #[test]
    fn test_match_bool_fail_on_partial() {
        let mut c = StrCursor::new("tru");
        assert!(c.match_bool().is_none());
        assert_eq!(c.offset, 0);
    }
}
