// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Traits that define the pyre Pattern and Match interfaces.
//! These traits mirror the Python `re` module API.

pub trait Pattern<'a>: Clone {
    type Match: Match<'a>;
    type Error: std::error::Error;

    fn search(&self, text: &'a str) -> Option<Self::Match>;

    fn match_(&self, text: &'a str) -> Option<Self::Match>;

    fn fullmatch(&self, text: &'a str) -> Option<Self::Match>;

    fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String>;

    /// Returns a vector of matches. Each match is represented as a vector of
    /// strings: if the pattern contains no capturing groups the inner vector
    /// will contain the whole match; if there is one capturing group the
    /// inner vector will contain that group's text; if there are multiple
    /// groups the inner vector contains each group's text (empty string for
    /// non-participating groups), matching Python's `re.findall` semantics.
    fn findall(&self, text: &str) -> Vec<Vec<String>>;

    fn finditer(&self, text: &'a str) -> Vec<Self::Match>;

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
