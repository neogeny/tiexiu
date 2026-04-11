// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This API mimics the one offred by the Python `pyre` library module.
//! The definitions in Ruse were taken from the work by the [RustPython][] Team.
//!
//!    [RustPython]: https://github.com/RustPython/RustPython
//!
//! The implementation currently relies on the fancy_regex crate

use fancy_regex;
use fancy_regex::{Captures, Match as FMatch, Regex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid regex pattern: {0}")]
    Regex(#[from] fancy_regex::Error),
}

#[derive(Debug, Clone)]
pub struct Pattern {
    regex: Regex,
    pattern: String,
}

#[derive(Debug, Clone)]
pub struct Match<'a> {
    haystack: &'a str,
    groups: Vec<Option<std::ops::Range<usize>>>,
}

pub fn escape(pattern: &str) -> Box<str> {
    fancy_regex::escape(pattern).into()
}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Self, Error> {
        Ok(Self {
            regex: Regex::new(pattern)?,
            pattern: pattern.to_string(),
        })
    }

    pub fn search<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        match self.regex.find(text) {
            Ok(Some(m)) => Some(create_match(text, &m)),
            _ => None,
        }
    }

    pub fn match_<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        let matches = self.finditer(text);
        let first = matches.into_iter().next()?;
        if first.start(Option::<usize>::None) != 0 {
            return None;
        }
        Some(first)
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
            if let Ok(caps) = caps_result
                && let Some(m) = caps.get(0)
            {
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
        result.push(text[last_end..].to_string());
        result
    }

    pub fn findall(&self, text: &str) -> Vec<Vec<String>> {
        self.regex
            .captures_iter(text)
            .filter_map(|caps_result| caps_result.ok())
            .map(|caps| {
                // caps.len() includes the whole match at index 0.
                if caps.len() == 1 {
                    // No capturing groups: return the whole match as a single-element vec
                    caps.get(0)
                        .map(|m| vec![m.as_str().to_string()])
                        .unwrap_or_default()
                } else if caps.len() == 2 {
                    // Single capturing group: return that group's text (empty string if missing)
                    let g = caps
                        .get(1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                    vec![g]
                } else {
                    // Multiple capturing groups: return each group's text (1..len)
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
            .filter_map(|caps_result| {
                caps_result
                    .ok()
                    .map(|caps| create_match_from_captures(text, &caps))
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
            if let Ok(caps) = caps_result
                && let Some(m) = caps.get(0)
            {
                result.push_str(&text[last_end..m.start()]);
                result.push_str(repl);
                last_end = m.end();
                replacements += 1;
            }
        }
        result.push_str(&text[last_end..]);
        (result, replacements)
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

fn create_match<'a>(haystack: &'a str, m: &FMatch) -> Match<'a> {
    let groups = vec![Some(m.start()..m.end())];
    Match { haystack, groups }
}

fn create_match_from_captures<'a>(haystack: &'a str, caps: &Captures) -> Match<'a> {
    let groups: Vec<Option<std::ops::Range<usize>>> = caps
        .iter()
        .map(|opt| opt.map(|m| m.start()..m.end()))
        .collect();
    Match { haystack, groups }
}

impl<'a> Match<'a> {
    pub fn group(&self, group: usize) -> Option<&'a str> {
        self.groups
            .get(group)?
            .as_ref()
            .map(|r| &self.haystack[r.start..r.end])
    }

    pub fn groups(&self) -> Vec<Option<&'a str>> {
        self.groups
            .iter()
            .map(|g| g.as_ref().map(|r| &self.haystack[r.start..r.end]))
            .collect()
    }

    pub fn start(&self, group: Option<usize>) -> isize {
        let group = group.unwrap_or(0);
        self.groups
            .get(group)
            .and_then(|g| g.as_ref())
            .map(|r| r.start as isize)
            .unwrap_or(-1)
    }

    pub fn end(&self, group: Option<usize>) -> isize {
        let group = group.unwrap_or(0);
        self.groups
            .get(group)
            .and_then(|g| g.as_ref())
            .map(|r| r.end as isize)
            .unwrap_or(-1)
    }

    pub fn span(&self, group: Option<usize>) -> (usize, usize) {
        let group = group.unwrap_or(0);
        self.groups
            .get(group)
            .and_then(|g| g.as_ref())
            .map(|r| (r.start, r.end))
            .unwrap_or((0, 0))
    }
}

use crate::util::pyre::traits as traits_impl;

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
}

impl<'a> traits_impl::Pattern<'a> for Pattern {
    type Match = Match<'a>;
    type Error = Error;

    fn search(&self, text: &'a str) -> Option<Self::Match> {
        self.search(text)
    }

    fn match_(&self, text: &'a str) -> Option<Self::Match> {
        self.match_(text)
    }

    fn fullmatch(&self, text: &'a str) -> Option<Self::Match> {
        self.fullmatch(text)
    }

    fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String> {
        self.split(text, maxsplit)
    }

    fn findall(&self, text: &str) -> Vec<Vec<String>> {
        self.findall(text)
    }

    fn finditer(&self, text: &'a str) -> Vec<Self::Match> {
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

pub fn compile(pattern: &str) -> Result<Pattern, Error> {
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
