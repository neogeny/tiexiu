// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Cursor;
use super::error::Error;
use crate::util::pyre::Pattern as Regex;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Patterns {
    pub wsp: Regex,
    pub cmt: Regex,
    pub eol: Regex,
}

impl Patterns {
    const DEFAULT_WSP: &'static str = r"\s+";
    const DEFAULT_EOL: &'static str = r"//.*$";
    const DEFAULT_CMT: &'static str = r"";

    fn compile(kind: &'static str, pattern: &str) -> Result<Regex, Error> {
        Regex::new(pattern).map_err(|source| Error::InvalidRegex {
            kind,
            pattern: pattern.to_string(),
            source,
        })
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
    fn eat_regex(&mut self, re: &Regex) -> bool {
        let text = &self.text[self.offset..];
        if let Some(mat) = re.search(text) {
            if mat.start(Option::<usize>::None) == 0 {
                self.offset += mat.end(Option::<usize>::None) as usize;
                return true;
            }
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

    fn token(&mut self, token: &str) -> bool {
        if self.text[self.offset..].starts_with(token) {
            self.offset += token.len();
            true
        } else {
            false
        }
    }

    fn pattern_re(&mut self, re: &Regex) -> Option<String> {
        let text = &self.text[self.offset..];
        let matches = re.finditer(text);
        let first = matches.into_iter().next()?;
        if first.start(Option::<usize>::None) != 0 {
            return None;
        }

        self.offset += first.end(Option::<usize>::None) as usize;
        let matched = first
            .group(1)
            .unwrap_or_else(|| first.group(0).unwrap_or(""));
        Some(matched.to_string())
    }

    fn next_token(&mut self) {
        let p = self.patterns.clone();
        let mut last_offset = usize::MAX;

        while self.offset != last_offset {
            last_offset = self.offset;
            self.eat_regex(&p.wsp);
            if self.at_end() {
                break;
            }
            if self.eat_regex(&p.eol) {
                self.eat_regex(&p.wsp);
            }
            self.eat_regex(&p.cmt);
        }
    }
}
