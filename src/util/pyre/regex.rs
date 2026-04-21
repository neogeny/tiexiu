// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module provides a regex-crate-backed implementation of the pyre traits.

use crate::util::pyre::error::{Error, Result};
use crate::util::pyre::traits;
use regex::{Captures, Regex, RegexBuilder};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Pattern {
    regex: Regex,
    pattern: String,
}

#[derive(Debug)]
pub struct Match<'a> {
    pub haystack: &'a str,
    captures: Captures<'a>,
}

pub fn escape(pattern: &str) -> Box<str> {
    regex::escape(pattern).into()
}

pub fn compile(pattern: &str) -> Result<Pattern> {
    Pattern::new(pattern)
}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Self> {
        Ok(Self {
            regex: RegexBuilder::new(pattern).unicode(true).build()?,
            pattern: pattern.to_string(),
        })
    }

    pub fn search<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        self.regex.captures(text).map(|captures| Match {
            haystack: text,
            captures,
        })
    }

    pub fn match_<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        self.regex.captures(text).and_then(|captures| {
            if captures.get(0).is_some_and(|m| m.start() == 0) {
                Some(Match {
                    haystack: text,
                    captures,
                })
            } else {
                None
            }
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

        for (splits_done, caps) in self.regex.captures_iter(text).enumerate() {
            if maxsplit > 0 && splits_done >= maxsplit {
                break;
            }
            let m = caps.get(0).unwrap(); // get(0) is always the whole match
            result.push(text[last_end..m.start()].to_string());

            // Add capturing groups to result
            for i in 1..caps.len() {
                if let Some(cap) = caps.get(i) {
                    result.push(text[cap.start()..cap.end()].to_string());
                } else {
                    result.push(String::new());
                }
            }
            last_end = m.end();
        }
        result.push(text[last_end..].to_string());
        result
    }

    pub fn findall(&self, text: &str) -> Vec<Vec<String>> {
        self.regex
            .captures_iter(text)
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
            .captures_iter(text)
            .map(|captures| Match {
                haystack: text,
                captures,
            })
            .collect()
    }

    pub fn sub(&self, repl: &str, text: &str, count: Option<usize>) -> String {
        if let Some(c) = count {
            self.regex.replacen(text, c, repl).to_string()
        } else {
            self.regex.replace_all(text, repl).to_string()
        }
    }

    pub fn subn(&self, repl: &str, text: &str, count: Option<usize>) -> (String, usize) {
        let mut replacements = 0;
        let replaced = if let Some(c) = count {
            self.regex
                .replacen(text, c, |caps: &Captures| {
                    replacements += 1;
                    let mut res = String::new();
                    caps.expand(repl, &mut res);
                    res
                })
                .to_string()
        } else {
            self.regex
                .replace_all(text, |caps: &regex::Captures| {
                    replacements += 1;
                    let mut res = String::new();
                    caps.expand(repl, &mut res);
                    res
                })
                .to_string()
        };
        (replaced, replacements)
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
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

    fn group_name(&self, name: &str) -> Option<&'a str> {
        self.captures.name(name).map(|m| m.as_str())
    }

    fn groupdict(&self) -> HashMap<Box<str>, Option<&'a str>> {
        // Implementation for regex crate Match:
        // We need to iterate over all possible group names.
        // Captures doesn't directly expose named captures list,
        // but we can't implement it efficiently without the Regex object.
        HashMap::new()
    }

    fn expand(&self, template: &str) -> String {
        let mut result = String::new();
        self.captures.expand(template, &mut result);
        result
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

    fn groupindex(&self) -> HashMap<Box<str>, usize> {
        self.regex
            .capture_names()
            .enumerate()
            .filter_map(|(idx, name)| name.map(|n| (n.into(), idx)))
            .collect()
    }

    fn groups_count(&self) -> usize {
        self.regex.captures_len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::pyre::traits::Match as TMatch;
    use crate::util::pyre::traits::Pattern as TPattern;

    #[test]
    fn test_regex_basic() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.search("abc123def").unwrap();
        assert_eq!(TMatch::group(&m, 0), Some("123"));
        assert_eq!(TMatch::start(&m, None), 3);
        assert_eq!(TMatch::end(&m, None), 6);
    }

    #[test]
    fn test_regex_group_name() {
        let p = Pattern::new(r"(?P<digit>\d+)").unwrap();
        let m = p.search("abc123def").unwrap();
        assert_eq!(TMatch::group_name(&m, "digit"), Some("123"));
    }

    #[test]
    fn test_regex_expand() {
        let p = Pattern::new(r"(\d+)").unwrap();
        let m = p.search("123").unwrap();
        assert_eq!(TMatch::expand(&m, r"val: $1"), "val: 123");
    }

    #[test]
    fn test_regex_groupindex() {
        let p = Pattern::new(r"(?P<digit>\d+)").unwrap();
        let index = TPattern::groupindex(&p);
        assert_eq!(index.get("digit").unwrap(), &1);
    }
}
