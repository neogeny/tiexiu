// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This API mimics the one offered by the Python `pyre` library module.
//! The implementation currently relies on the pcre2 crate.

use super::error::{Error, Result};
use super::traits;
use pcre2::bytes::{Captures, Regex, RegexBuilder};

#[derive(Debug, Clone)]
pub struct Pattern {
    regex: Regex,
    anchored: Regex,
    pattern: String,
}

#[derive(Debug)]
pub struct Match<'a> {
    haystack: &'a str,
    captures: Captures<'a>,
}

pub fn escape(pattern: &str) -> Box<str> {
    fancy_regex::escape(pattern).into()
}

pub fn compile(pattern: &str) -> Result<Pattern> {
    Pattern::new(pattern)
}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Self> {
        let regex = RegexBuilder::new().utf(true).jit(true).build(pattern)?;

        let anchored_pattern = format!(r"\A(?:{})", pattern);
        let anchored = RegexBuilder::new()
            .utf(true)
            .jit(true)
            .build(&anchored_pattern)?;

        Ok(Self {
            regex,
            anchored,
            pattern: pattern.to_string(),
        })
    }

    pub fn search<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        self.regex
            .captures(text.as_bytes())
            .ok()
            .flatten()
            .map(|captures| Match {
                haystack: text,
                captures,
            })
    }

    pub fn match_<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        self.anchored
            .captures(text.as_bytes())
            .ok()
            .flatten()
            .map(|captures| Match {
                haystack: text,
                captures,
            })
    }

    pub fn fullmatch<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        let m = self.match_(text)?;
        if m.end(None) == text.len() as isize {
            Some(m)
        } else {
            None
        }
    }

    pub fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String> {
        let maxsplit = maxsplit.unwrap_or(0);
        let mut result = Vec::new();
        let mut last_end = 0;

        for (splits_done, caps_res) in self.regex.captures_iter(text.as_bytes()).enumerate() {
            if maxsplit > 0 && splits_done >= maxsplit {
                break;
            }
            if let Ok(caps) = caps_res {
                if let Some(m) = caps.get(0) {
                    result.push(text[last_end..m.start()].to_string());
                    for i in 1..caps.len() {
                        if let Some(cap) = caps.get(i) {
                            result.push(text[cap.start()..cap.end()].to_string());
                        } else {
                            result.push(String::new());
                        }
                    }
                    last_end = m.end();
                }
            }
        }
        result.push(text[last_end..].to_string());
        result
    }

    pub fn findall(&self, text: &str) -> Vec<Vec<String>> {
        self.regex
            .captures_iter(text.as_bytes())
            .filter_map(|caps_result| caps_result.ok())
            .map(|caps| {
                if caps.len() == 1 {
                    let m = caps.get(0).unwrap();
                    vec![text[m.start()..m.end()].to_string()]
                } else {
                    (1..caps.len())
                        .map(|i| {
                            caps.get(i)
                                .map(|m| text[m.start()..m.end()].to_string())
                                .unwrap_or_default()
                        })
                        .collect()
                }
            })
            .collect()
    }

    pub fn finditer<'a>(&self, text: &'a str) -> Vec<Match<'a>> {
        self.regex
            .captures_iter(text.as_bytes())
            .filter_map(|caps_result| caps_result.ok())
            .map(|captures| Match {
                haystack: text,
                captures,
            })
            .collect()
    }

    pub fn sub(&self, repl: &str, text: &str, count: Option<usize>) -> String {
        self.subn(repl, text, count).0
    }

    pub fn subn(&self, repl: &str, text: &str, count: Option<usize>) -> (String, usize) {
        let count = count.unwrap_or(0);
        let mut result = String::new();
        let mut last_end = 0;
        let mut replacements = 0;

        for caps_result in self.regex.captures_iter(text.as_bytes()) {
            if count > 0 && replacements >= count {
                break;
            }
            if let Ok(caps) = caps_result {
                if let Some(m) = caps.get(0) {
                    result.push_str(&text[last_end..m.start()]);
                    result.push_str(repl);
                    last_end = m.end();
                    replacements += 1;
                }
            }
        }
        result.push_str(&text[last_end..]);
        (result, replacements)
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl<'a> Match<'a> {
    pub fn group(&self, group: usize) -> Option<&'a str> {
        let m = self.captures.get(group)?;
        Some(&self.haystack[m.start()..m.end()])
    }

    pub fn groups(&self) -> Vec<Option<&'a str>> {
        (0..self.captures.len()).map(|i| self.group(i)).collect()
    }

    pub fn start(&self, group: Option<usize>) -> isize {
        let group = group.unwrap_or(0);
        self.captures
            .get(group)
            .map(|m| m.start() as isize)
            .unwrap_or(-1)
    }

    pub fn end(&self, group: Option<usize>) -> isize {
        let group = group.unwrap_or(0);
        self.captures
            .get(group)
            .map(|m| m.end() as isize)
            .unwrap_or(-1)
    }

    pub fn span(&self, group: Option<usize>) -> (usize, usize) {
        let group = group.unwrap_or(0);
        self.captures
            .get(group)
            .map(|m| (m.start(), m.end()))
            .unwrap_or((0, 0))
    }
}

impl<'a> traits::Match<'a> for Match<'a> {
    fn group(&self, group: usize) -> Option<&'a str> {
        self.group(group)
    }

    fn groups(&self) -> Vec<Option<&'a str>> {
        self.groups()
    }

    fn start(&self, group: Option<usize>) -> isize {
        self.start(group)
    }

    fn end(&self, group: Option<usize>) -> isize {
        self.end(group)
    }

    fn span(&self, group: Option<usize>) -> (usize, usize) {
        self.span(group)
    }
}

impl traits::Pattern for Pattern {
    type Match<'a>
        = Match<'a>
    where
        Self: 'a;
    type Error = Error;

    fn search<'a>(&self, text: &'a str) -> Option<Self::Match<'a>> {
        self.search(text)
    }

    fn match_<'a>(&self, text: &'a str) -> Option<Self::Match<'a>> {
        self.match_(text)
    }

    fn fullmatch<'a>(&self, text: &'a str) -> Option<Self::Match<'a>> {
        self.fullmatch(text)
    }

    fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String> {
        self.split(text, maxsplit)
    }

    fn findall(&self, text: &str) -> Vec<Vec<String>> {
        self.findall(text)
    }

    fn finditer<'a>(&self, text: &'a str) -> Vec<Self::Match<'a>> {
        self.finditer(text)
    }

    fn sub(&self, repl: &str, text: &str, count: Option<usize>) -> String {
        self.sub(repl, text, count)
    }

    fn subn(&self, repl: &str, text: &str, count: Option<usize>) -> (String, usize) {
        self.subn(repl, text, count)
    }

    fn pattern(&self) -> &str {
        self.pattern()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcre2_basic() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.search("abc123def").unwrap();
        assert_eq!(traits::Match::group(&m, 0), Some("123"));
        assert_eq!(traits::Match::start(&m, None), 3);
        assert_eq!(traits::Match::end(&m, None), 6);
    }
}
