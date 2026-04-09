// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This API mimics the one offred by the Python `re` library module.
//! The definitions in Ruse were taken from the work by the [RustPython][] Team.
//!
//!    [RustPython]: https://github.com/RustPython/RustPython
//!
//! The implementation currently relies on the fancy_regex crate

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
pub struct Match {
    haystack: String,
    groups: Vec<Option<std::ops::Range<usize>>>,
}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Self, Error> {
        Ok(Self {
            regex: Regex::new(pattern)?,
            pattern: pattern.to_string(),
        })
    }

    pub fn search(&self, text: &str) -> Option<Match> {
        self.regex
            .find(text)
            .as_ref()
            .ok()
            .and_then(|m| m.as_ref().map(|mm| create_match(text, mm)))
    }

    pub fn match_at_beginning(&self, text: &str) -> Option<Match> {
        let full_pattern = format!("^{}", self.regex.as_str());
        let re = Regex::new(&full_pattern).ok()?;
        re.find(text)
            .as_ref()
            .ok()
            .and_then(|m| m.as_ref().map(|mm| create_match(text, mm)))
    }

    pub fn fullmatch_at_ends(&self, text: &str) -> Option<Match> {
        let full_pattern = format!("^{}$", self.regex.as_str());
        let re = Regex::new(&full_pattern).ok()?;
        re.find(text)
            .as_ref()
            .ok()
            .and_then(|m| m.as_ref().map(|mm| create_match(text, mm)))
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

    pub fn findall(&self, text: &str) -> Vec<String> {
        self.regex
            .captures_iter(text)
            .filter_map(|caps_result| {
                caps_result.ok().and_then(|caps| {
                    if caps.len() == 1 {
                        caps.get(0).map(|m| m.as_str().to_string())
                    } else {
                        caps.get(1).map(|m| m.as_str().to_string())
                    }
                })
            })
            .collect()
    }

    pub fn finditer(&self, text: &str) -> Vec<Match> {
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
        result
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

fn create_match(haystack: &str, m: &FMatch) -> Match {
    let groups = vec![Some(0..m.end())];
    Match {
        haystack: haystack.to_string(),
        groups,
    }
}

fn create_match_from_captures(haystack: &str, caps: &Captures) -> Match {
    let groups: Vec<Option<std::ops::Range<usize>>> = caps
        .iter()
        .map(|opt| opt.map(|m| m.start()..m.end()))
        .collect();
    Match {
        haystack: haystack.to_string(),
        groups,
    }
}

impl Match {
    pub fn group(&self, group: usize) -> Option<&str> {
        self.groups
            .get(group)?
            .as_ref()
            .map(|r| &self.haystack[r.start..r.end])
    }

    pub fn groups(&self) -> Vec<Option<&str>> {
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

pub fn compile(pattern: &str) -> Result<Pattern, Error> {
    Pattern::new(pattern)
}

pub fn search(pattern: &str, text: &str) -> Option<Match> {
    Pattern::new(pattern).ok()?.search(text)
}

pub fn match_(pattern: &str, text: &str) -> Option<Match> {
    Pattern::new(pattern).ok()?.match_at_beginning(text)
}

pub fn fullmatch(pattern: &str, text: &str) -> Option<Match> {
    Pattern::new(pattern).ok()?.fullmatch_at_ends(text)
}

pub fn split(pattern: &str, text: &str, maxsplit: Option<usize>) -> Vec<String> {
    match Pattern::new(pattern) {
        Ok(p) => p.split(text, maxsplit),
        Err(_) => vec![text.to_string()],
    }
}

pub fn findall(pattern: &str, text: &str) -> Vec<String> {
    match Pattern::new(pattern) {
        Ok(p) => p.findall(text),
        Err(_) => vec![],
    }
}

pub fn finditer(pattern: &str, text: &str) -> Vec<Match> {
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

pub fn escape(pattern: &str) -> String {
    let mut result = String::new();
    for c in pattern.chars() {
        match c {
            '\\' | '^' | '$' | '.' | '|' | '?' | '*' | '+' | '(' | ')' | '[' | ']' | '{' | '}'
            | '"' => {
                result.push('\\');
                result.push(c);
            }
            _ => result.push(c),
        }
    }
    result
}

pub fn purge() {}
