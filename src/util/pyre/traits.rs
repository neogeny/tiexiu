// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

//! Traits that define the pyre Pattern and Match interfaces.
//! These traits mirror the Python `re` module API.

pub trait Pattern: Clone {
    type Match<'a>: Match<'a>
    where
        Self: 'a;
    type Error: std::error::Error;

    fn search<'a>(&self, text: &'a str) -> Option<Self::Match<'a>>;

    fn match_<'a>(&self, text: &'a str) -> Option<Self::Match<'a>>;

    fn fullmatch<'a>(&self, text: &'a str) -> Option<Self::Match<'a>>;

    fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String>;

    /// Returns a vector of matches. Each match is represented as a vector of
    /// strings: if the pattern contains no capturing groups the inner vector
    /// will contain the whole match; if there is one capturing group the
    /// inner vector will contain that group's text; if there are multiple
    /// groups the inner vector contains each group's text (empty string for
    /// non-participating groups), matching Python's `re.findall` semantics.
    fn findall(&self, text: &str) -> Vec<Vec<String>>;

    fn finditer<'a>(&self, text: &'a str) -> Vec<Self::Match<'a>>;

    fn sub(&self, repl: &str, text: &str, count: Option<usize>) -> String;

    fn subn(&self, repl: &str, text: &str, count: Option<usize>) -> (String, usize);

    fn pattern(&self) -> &str;
}

pub trait Match<'a> {
    fn group(&self, group: usize) -> Option<&'a str>;

    fn groups(&self) -> Vec<Option<&'a str>>;

    fn start(&self, group: Option<usize>) -> isize;

    fn end(&self, group: Option<usize>) -> isize;

    fn span(&self, group: Option<usize>) -> (usize, usize);
}
