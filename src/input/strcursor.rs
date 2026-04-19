// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Cursor;
use super::error::Error;
use super::tokenizing::TokenizingPatterns;
use crate::cfg::*;
use crate::util::newlines::empty_line;
use crate::util::pyre::Pattern;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct StrCursor {
    text: Box<str>,
    offset: usize,
    patterns: Rc<TokenizingPatterns>,
}

impl From<&str> for StrCursor {
    #[inline]
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl StrCursor {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            offset: 0,
            patterns: TokenizingPatterns::default().into(),
        }
    }

    pub fn with_patterns(text: &str, patterns: TokenizingPatterns) -> Result<Self, Error> {
        Ok(Self {
            text: text.into(),
            offset: 0,
            patterns: patterns.into(),
        })
    }

    #[inline]
    fn eat_pattern(&mut self, pat: &Pattern) -> bool {
        let text = &self.text[self.offset..];
        if let Some(mat) = pat.match_(text) {
            self.offset += mat.end(None) as usize;
            return true;
        }
        false
    }
}

impl Configurable for StrCursor {
    fn configure(&mut self, cfg: &Cfg) {
        if let Ok(patterns) = self.tokenizing_from_cfg(cfg) {
            self.set_tokenizing(&patterns);
        }
    }
}

impl Cursor for StrCursor {
    fn mark(&self) -> usize {
        self.offset
    }

    fn reset(&mut self, mark: usize) {
        self.offset = mark;
    }

    fn textstr(&self) -> &str {
        &self.text
    }

    fn at_end(&self) -> bool {
        self.offset >= self.text.len()
    }

    fn next(&mut self) -> Option<char> {
        let rest = self.text.get(self.offset..)?;
        let c = rest.chars().next()?;
        self.offset += c.len_utf8();
        Some(c)
    }

    fn match_token(&mut self, token: &str) -> bool {
        let text = &self.text[self.offset..];
        if text.starts_with(token) {
            self.offset += token.len();
            true
        } else {
            false
        }
    }

    fn match_pattern(&mut self, pat: &Pattern) -> Option<String> {
        let text = &self.text[self.offset..];
        let m = pat.match_(text)?;

        self.offset += m.end(None) as usize;
        m.group(1).or(m.group(0)).map(|s| s.to_string())
    }

    fn match_eol(&mut self) -> bool {
        if let Some(len) = empty_line(&self.text[self.offset..]) {
            self.offset += len;
            true
        } else {
            false
        }
    }

    fn next_token(&mut self) {
        let p = self.patterns.clone();
        let mut last_offset = usize::MAX;

        while self.offset != last_offset {
            last_offset = self.offset;
            self.eat_pattern(&p.wsp);
            if self.at_end() {
                break;
            }
            if self.eat_pattern(&p.eol) {
                self.eat_pattern(&p.wsp);
            }
            let cmt_text = p.cmt.pattern();
            if !cmt_text.is_empty() {
                self.eat_pattern(&p.cmt);
            }
        }
    }

    fn set_tokenizing(&mut self, patterns: &TokenizingPatterns) {
        self.patterns = patterns.clone().into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "matches empty string")]
    fn whitespace_pattern_cannot_match_empty() {
        let _ = TokenizingPatterns::try_new("", "/* */", "//.*$");
    }

    #[test]
    #[should_panic(expected = "matches empty string")]
    fn comment_pattern_cannot_match_empty() {
        let _ = TokenizingPatterns::try_new(r"\s+", "", "//.*$");
    }

    #[test]
    #[should_panic(expected = "matches empty string")]
    fn eol_pattern_cannot_match_empty() {
        let _ = TokenizingPatterns::try_new(r"\s+", "/* */", "");
    }

    #[test]
    fn default_patterns_are_valid() {
        let patterns = TokenizingPatterns::default();
        assert!(patterns.wsp.search("").is_none());
        assert!(patterns.cmt.search("").is_none());
        assert!(patterns.eol.search("").is_none());
    }
}
