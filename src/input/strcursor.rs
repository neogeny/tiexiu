// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Cursor;
use super::error::Error;
use crate::util::pyre::Pattern;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Patterns {
    pub wsp: Pattern,
    pub cmt: Pattern,
    pub eol: Pattern,
}

impl Patterns {
    const DEFAULT_WSP: &'static str = r"\s+";
    const DEFAULT_EOL: &'static str = r"//.*$";
    const DEFAULT_CMT: &'static str = r"/\*\*/";

    fn compile(kind: &'static str, pattern: &str) -> Result<Pattern, Error> {
        let p = Pattern::new(pattern).map_err(|source| Error::InvalidRegex {
            kind,
            pattern: pattern.to_string(),
            source,
        })?;
        Self::validate_no_empty_match(&p, kind);
        Ok(p)
    }

    fn validate_no_empty_match(pattern: &Pattern, kind: &str) {
        assert!(
            pattern.search("").is_none(),
            "pattern '{}' for {} matches empty string, which would cause infinite loop",
            pattern.pattern(),
            kind
        );
    }

    pub fn try_new(ws: &str, cmt: &str, eol: &str) -> Result<Self, Error> {
        Ok(Self {
            wsp: Self::compile("whitespace", ws)?,
            cmt: Self::compile("comment", cmt)?,
            eol: Self::compile("end-of-line", eol)?,
        })
    }
}

impl Default for Patterns {
    fn default() -> Self {
        Self::try_new(Self::DEFAULT_WSP, Self::DEFAULT_CMT, Self::DEFAULT_EOL)
            .expect("default StrCursor regex patterns must be valid")
    }
}

#[derive(Debug, Clone)]
pub struct StrCursor<'a> {
    text: &'a str,
    offset: usize,
    patterns: Rc<Patterns>,
}

impl<'a> From<&'a str> for StrCursor<'a> {
    #[inline]
    fn from(text: &'a str) -> Self {
        Self::new(text)
    }
}

impl<'a> StrCursor<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            offset: 0,
            patterns: Rc::new(Patterns::default()),
        }
    }

    pub fn with_patterns(text: &'a str, ws: &str, cmt: &str, eol: &str) -> Result<Self, Error> {
        Ok(Self {
            text,
            offset: 0,
            patterns: Rc::new(Patterns::try_new(ws, cmt, eol)?),
        })
    }

    #[inline]
    fn eat_pattern(&mut self, pat: &Pattern) -> bool {
        let text = &self.text[self.offset..];
        if let Some(mat) = pat.match_(text) {
            self.offset += mat.end(Option::<usize>::None) as usize;
            return true;
        }
        false
    }
}

impl<'a> Cursor for StrCursor<'a> {
    fn mark(&self) -> usize {
        self.offset
    }

    fn reset(&mut self, mark: usize) {
        self.offset = mark;
    }

    fn textstr(&self) -> &str {
        self.text
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

        self.offset += m.end(Option::<usize>::None) as usize;
        m.group(1).or(m.group(0)).map(|s| s.to_string())
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "matches empty string")]
    fn whitespace_pattern_cannot_match_empty() {
        let _ = Patterns::try_new("", "/* */", "//.*$");
    }

    #[test]
    #[should_panic(expected = "matches empty string")]
    fn comment_pattern_cannot_match_empty() {
        let _ = Patterns::try_new(r"\s+", "", "//.*$");
    }

    #[test]
    #[should_panic(expected = "matches empty string")]
    fn eol_pattern_cannot_match_empty() {
        let _ = Patterns::try_new(r"\s+", "/* */", "");
    }

    #[test]
    fn default_patterns_are_valid() {
        let patterns = Patterns::default();
        assert!(patterns.wsp.search("").is_none());
        assert!(patterns.cmt.search("").is_none());
        assert!(patterns.eol.search("").is_none());
    }
}
