// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Cursor;
use super::error::Error;
use super::tokenizing::TokenizingPatterns;
use crate::cfg::keys::config;
use crate::cfg::*;
use crate::types::Str;
use crate::util::newlines::{take_linebreak_len, take_non_newline_whitespace_len};
use crate::util::pyre::Pattern;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CursorHeavy {
    ignorecase: bool,
    patterns: Rc<TokenizingPatterns>,
}

#[derive(Debug, Clone)]
pub struct StrCursor {
    text: Str,
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
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            offset: 0,
            heavy: CursorHeavy {
                ignorecase: false,
                patterns: TokenizingPatterns::default().into(),
            }
            .into(),
        }
    }

    pub fn with_patterns(text: &str, patterns: TokenizingPatterns) -> Result<Self, Error> {
        Ok(Self {
            text: text.into(),
            offset: 0,
            heavy: CursorHeavy {
                ignorecase: false,
                patterns: patterns.into(),
            }
            .into(),
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
        if let Ok(patterns) = self.tokenizing_from_cfg(&cfg) {
            self.set_tokenizing(&patterns);
        }
        self.heavy = CursorHeavy {
            ignorecase: cfg.contains(&CfgKey::IgnoreCase),
            patterns: self.heavy.patterns.clone(),
        }
        .into()
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

    fn ignore_case(&self) -> bool {
        self.heavy.ignorecase
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
        let token_len = token.len();
        if let Some(text_slice) = self.text[self.offset..].get(..token_len)
            && (self.ignore_case() && text_slice.eq_ignore_ascii_case(token)
                || !self.ignore_case() && text_slice == token)
        {
            self.offset += token_len;
            return true;
        }
        false
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
        let skip_all = self.heavy.patterns.skip_all.clone();
        if let Some(pat) = skip_all {
            self.eat_pattern(&pat);
        }
    }

    fn set_tokenizing(&mut self, patterns: &TokenizingPatterns) {
        self.heavy = CursorHeavy {
            ignorecase: self.heavy.ignorecase,
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
    #[should_panic(expected = "matches empty string")]
    fn whitespace_pattern_cannot_match_empty() {
        let _ = TokenizingPatterns::try_new(r"[ \t]*", "/* */", "//.*$");
    }

    #[test]
    #[should_panic(expected = "matches empty string")]
    fn comment_pattern_cannot_match_empty() {
        let _ = TokenizingPatterns::try_new(r"\s+", ".*", "//.*$");
    }

    #[test]
    #[should_panic(expected = "matches empty string")]
    fn eol_pattern_cannot_match_empty() {
        let _ = TokenizingPatterns::try_new(r"\s+", "/* */", r"\w?");
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
