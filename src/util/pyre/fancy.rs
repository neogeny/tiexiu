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

#[derive(Debug, Clone)]
pub struct Pattern {
    regex: Regex,
    anchored: Regex,
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

pub fn searchi<'a>(pattern: &str, text: &'a str) -> Option<Match<'a>> {
    Pattern::new(pattern).ok()?.search(text)
}

pub fn match_<'a>(pattern: &str, text: &'a str) -> Option<Match<'a>> {
    Pattern::new(pattern).ok()?.match_(text)
}

pub fn fullmatch<'a>(pattern: &str, text: &'a str) -> Option<Match<'a>> {
    Pattern::new(pattern).ok()?.fullmatch(text)
}

pub fn split(pattern: &str, text: &str, maxsplit: Option<usize>) -> Vec<String> {
    match Pattern::new(pattern) {
        Ok(p) => p.split(text, maxsplit),
        Err(_) => vec![text.to_string()],
    }
}

pub fn findall(pattern: &str, text: &str) -> Vec<Vec<String>> {
    match Pattern::new(pattern) {
        Ok(p) => p.findall(text),
        Err(_) => vec![],
    }
}

pub fn finditer<'a>(pattern: &str, text: &'a str) -> Vec<Match<'a>> {
    match Pattern::new(pattern) {
        Ok(p) => p.finditer(text),
        Err(_) => vec![],
    }
}

pub fn sub(pattern: &str, repl: &str, text: &str, count: Option<usize>) -> String {
    match Pattern::new(pattern) {
        Ok(p) => p.sub(repl, text, count),
        Err(_) => text.to_string(),
    }
}

pub fn subn(pattern: &str, repl: &str, text: &str, count: Option<usize>) -> (String, usize) {
    match Pattern::new(pattern) {
        Ok(p) => p.subn(repl, text, count),
        Err(_) => (text.to_string(), 0),
    }
}

pub fn purge() {}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Self> {
        let anchored = format!(r"\A(?:{})", pattern);
        Ok(Self {
            regex: Regex::new(pattern)?,
            anchored: Regex::new(&anchored)?,
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
        match self.anchored.captures(text) {
            Ok(Some(captures)) => Some(Match {
                string: text,
                captures,
            }),
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
        assert_eq!(m.unwrap().group(0), Some("123"));
    }

    #[test]
    fn pattern_search_not_found() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.search("abcdef");
        assert!(m.is_none());
    }

    #[test]
    fn pattern_match_at_start() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.match_("123abc");
        assert!(m.is_some());
    }

    #[test]
    fn pattern_match_not_at_start() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.match_("abc123");
        assert!(m.is_none());
    }

    #[test]
    fn pattern_fullmatch() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.fullmatch("123");
        assert!(m.is_some());
    }

    #[test]
    fn pattern_fullmatch_not_full() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.fullmatch("123abc");
        assert!(m.is_none());
    }

    #[test]
    fn pattern_split() {
        let p = Pattern::new(r":").unwrap();
        let result = p.split("a:b:c", None);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn pattern_split_with_maxsplit() {
        let p = Pattern::new(r":").unwrap();
        let result = p.split("a:b:c", Some(1));
        assert_eq!(result, vec!["a", "b:c"]);
    }

    #[test]
    fn pattern_findall() {
        let p = Pattern::new(r"\d").unwrap();
        let result = p.findall("1a2b3");
        assert_eq!(
            result,
            vec![
                vec!["1".to_string()],
                vec!["2".to_string()],
                vec!["3".to_string()]
            ]
        );
    }

    #[test]
    fn pattern_finditer() {
        let p = Pattern::new(r"\d").unwrap();
        let results = p.finditer("1a2b3");
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn pattern_sub() {
        let p = Pattern::new(r"\d").unwrap();
        let result = p.sub("X", "1a2b3", None);
        assert_eq!(result, "XaXbX");
    }

    #[test]
    fn pattern_subn() {
        let p = Pattern::new(r"\d").unwrap();
        let (result, count) = p.subn("X", "1a2b3", None);
        assert_eq!(result, "XaXbX");
        assert_eq!(count, 3);
    }

    #[test]
    fn pattern_sub_with_count() {
        let p = Pattern::new(r"\d").unwrap();
        let result = p.sub("X", "1a2b3", Some(1));
        assert_eq!(result, "Xa2b3");
    }

    #[test]
    fn match_group() {
        let p = Pattern::new(r"(?P<digit>\d)").unwrap();
        let m = p.search("1abc").unwrap();
        assert_eq!(m.group(0), Some("1"));
    }

    #[test]
    fn match_groups() {
        let p = Pattern::new(r"(\d)(\d)").unwrap();
        let m = p.search("12").unwrap();
        let groups = m.groups();
        // groups includes full match at index 0
        assert!(!groups.is_empty());
    }

    #[test]
    fn match_start() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.search("abc123").unwrap();
        assert_eq!(m.start(None), 3);
    }

    #[test]
    fn match_end() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.search("abc123").unwrap();
        assert_eq!(m.end(None), 6);
    }

    #[test]
    fn match_span() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.search("abc123").unwrap();
        assert_eq!(m.span(None), (3, 6));
    }

    #[test]
    fn escape_special_chars() {
        let result = escape(r"[]{}()\");
        assert!(result.contains('['));
        assert!(result.contains('\\'));
    }
}
