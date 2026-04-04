// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Cursor;
use regex::Regex;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Patterns {
    pub wsp: Regex,
    pub cmt: Regex,
    pub eol: Regex,
}

impl Patterns {
    const DEFAULT_WSP: &'static str = r"^\s+";
    const DEFAULT_EOL: &'static str = r"^//.*$";
    const DEFAULT_CMT: &'static str = r"^$"; // Matches nothing by default

    pub fn new(ws: &str, cmt: &str, eol: &str) -> Self {
        Self {
            wsp: Regex::new(ws).expect("invalid whitespace regex"),
            cmt: Regex::new(cmt).expect("invalid comment regex"),
            eol: Regex::new(eol).expect("invalid EOL regex"),
        }
    }
}

impl Default for Patterns {
    fn default() -> Self {
        Self::new(Self::DEFAULT_WSP, Self::DEFAULT_CMT, Self::DEFAULT_EOL)
    }
}

#[derive(Debug, Clone)]
pub struct StrCursor<'a> {
    text: &'a str,
    offset: usize,
    patterns: Rc<Patterns>,
}

impl<'a> StrCursor<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            offset: 0,
            patterns: Rc::new(Patterns::default()),
        }
    }

    pub fn with_patterns(text: &'a str, ws: &str, cmt: &str, eol: &str) -> Self {
        Self {
            text,
            offset: 0,
            patterns: Rc::new(Patterns::new(ws, cmt, eol)),
        }
    }

    #[inline]
    fn eat_regex(&mut self, re: &Regex) -> bool {
        if let Some(mat) = re.find_at(self.text, self.offset) {
            if mat.start() == self.offset {
                self.offset = mat.end();
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
        let caps = re.captures_at(self.text, self.offset)?;
        let whole = caps.get(0)?;
        if whole.start() != self.offset {
            return None;
        }

        self.offset = whole.end();
        let matched = caps.get(1).or(caps.get(0))?.as_str();
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
