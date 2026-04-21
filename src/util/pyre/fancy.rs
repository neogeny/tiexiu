// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This API mimics the one offred by the Python `pyre` library module.
//! The definitions in Ruse were taken from the work by the [RustPython][] Team.
//!
//!    [RustPython]: https://github.com/RustPython/RustPython
//!
//! The implementation currently relies on the fancy_regex crate

use super::error::{Error, Result};
use super::traits as traits_impl;
use fancy_regex;
use fancy_regex::{Captures, Regex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Pattern {
    regex: Regex,
}

#[derive(Debug)]
pub struct Match<'a> {
    pub string: &'a str,
    captures: Captures<'a>,
}

pub fn escape(pattern: &str) -> Box<str> {
    fancy_regex::escape(pattern).into()
}

pub fn compile(pattern: &str) -> Result<Pattern> {
    Pattern::new(pattern)
}

pub fn purge() {}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Self> {
        Ok(Self {
            regex: Regex::new(pattern)?,
        })
    }

    pub fn search<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        match self.regex.captures(text) {
            Ok(Some(captures)) => Some(Match {
                string: text,
                captures,
            }),
            _ => None,
        }
    }

    pub fn match_<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        match self.regex.captures(text) {
            Ok(Some(captures)) => {
                if captures.get(0).is_some_and(|m| m.start() == 0) {
                    Some(Match {
                        string: text,
                        captures,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn fullmatch<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        let m = self.match_(text)?;
        if m.end(None) != text.len() as isize {
            return None;
        }
        Some(m)
    }

    pub fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String> {
        let maxsplit = maxsplit.unwrap_or(0);
        let mut result = Vec::new();
        let mut last_end = 0;
        let mut splits_done = 0;

        for caps_result in self.regex.captures_iter(text) {
            if maxsplit > 0 && splits_done >= maxsplit {
                break;
            }
            if let Ok(caps) = caps_result {
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
                    splits_done += 1;
                }
            }
        }
        result.push(text[last_end..].to_string());
        result
    }

    pub fn findall(&self, text: &str) -> Vec<Vec<String>> {
        self.regex
            .captures_iter(text)
            .filter_map(|caps_result| caps_result.ok())
            .map(|caps| {
                if caps.len() == 1 {
                    caps.get(0)
                        .map(|m| vec![m.as_str().to_string()])
                        .unwrap_or_default()
                } else if caps.len() == 2 {
                    let g = caps
                        .get(1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                    vec![g]
                } else {
                    (1..caps.len())
                        .map(|i| {
                            caps.get(i)
                                .map(|m| m.as_str().to_string())
                                .unwrap_or_default()
                        })
                        .collect()
                }
            })
            .collect()
    }

    pub fn finditer<'a>(&self, text: &'a str) -> Vec<Match<'a>> {
        self.regex
            .captures_iter(text)
            .filter_map(|caps_result| caps_result.ok())
            .map(|captures| Match {
                string: text,
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

        for caps_result in self.regex.captures_iter(text) {
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
        self.regex.as_str()
    }
}

impl<'a> Match<'a> {
    pub fn group(&self, group: usize) -> Option<&'a str> {
        self.captures.get(group).map(|m| m.as_str())
    }

    pub fn groups(&self) -> Vec<Option<&'a str>> {
        self.captures
            .iter()
            .map(|opt| opt.map(|m| m.as_str()))
            .collect()
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

impl<'a> traits_impl::Match<'a> for Match<'a> {
    fn group(&self, group: usize) -> Option<&'a str> {
        Match::group(self, group)
    }

    fn groups(&self) -> Vec<Option<&'a str>> {
        Match::groups(self)
    }

    fn start(&self, group: Option<usize>) -> isize {
        Match::start(self, group)
    }

    fn end(&self, group: Option<usize>) -> isize {
        Match::end(self, group)
    }

    fn span(&self, group: Option<usize>) -> (usize, usize) {
        Match::span(self, group)
    }

    fn group_name(&self, name: &str) -> Option<&'a str> {
        self.captures.name(name).map(|m| m.as_str())
    }

    fn groupdict(&self) -> HashMap<Box<str>, Option<&'a str>> {
        // fancy_regex::Captures doesn't easily expose an iterator over named groups.
        // We can't implement this without more complex tracking.
        HashMap::new()
    }

    fn expand(&self, template: &str) -> String {
        let mut result = String::new();
        let dollared = template.replace("\\", "$");
        self.captures.expand(&dollared, &mut result);
        result
    }
}

impl traits_impl::Pattern for Pattern {
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

    fn groupindex(&self) -> HashMap<Box<str>, usize> {
        // FIXME: fancy-regex doesn't provide an easy way to iterate named groups.
        HashMap::new()
    }

    fn groups_count(&self) -> usize {
        // Regex doesn't expose number of groups directly.
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_new_valid() {
        let p = Pattern::new(r"\d+");
        assert!(p.is_ok());
    }

    #[test]
    fn pattern_new_invalid() {
        let p = Pattern::new(r"[");
        assert!(p.is_err());
    }

    #[test]
    fn pattern_search_found() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.search("abc123def");
        assert!(m.is_some());
        assert_eq!(traits_impl::Match::group(&m.unwrap(), 0), Some("123"));
    }

    #[test]
    fn match_group_name() {
        let p = Pattern::new(r"(?P<digit>\d+)").unwrap();
        let m = p.search("abc123def").unwrap();
        assert_eq!(traits_impl::Match::group_name(&m, "digit"), Some("123"));
    }

    #[test]
    fn match_expand() {
        let p = Pattern::new(r"(\d+)").unwrap();
        let m = p.search("123").unwrap();
        assert_eq!(traits_impl::Match::expand(&m, r"val: \1"), "val: 123");
    }
}
