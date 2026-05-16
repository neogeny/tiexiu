// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

//! Traits that define the pyre Pattern and Match interfaces.
//! These traits mirror the Python `re` module API.

use std::collections::HashMap;

/// Trait defining the pyre Pattern interface (mirroring Python's re module).
pub trait Pattern: Clone {
    type Match<'a>: Match<'a>
    where
        Self: 'a;
    type Error: std::error::Error;

    /// Searches for the pattern anywhere in the text.
    fn search<'a>(&self, text: &'a str) -> Option<Self::Match<'a>>;
    /// Matches the pattern at the beginning of text.
    fn match_<'a>(&self, text: &'a str) -> Option<Self::Match<'a>>;
    /// Matches the pattern against the entire text.
    fn fullmatch<'a>(&self, text: &'a str) -> Option<Self::Match<'a>>;
    /// Splits text by pattern matches.
    fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String>;

    /// Returns a vector of matches. Each match is represented as a vector of
    /// strings: if the pattern contains no capturing groups the inner vector
    /// will contain the whole match; if there is one capturing group the
    /// inner vector will contain that group's text; if there are multiple
    /// groups the inner vector contains each group's text (empty string for
    /// non-participating groups), matching Python's `re.findall` semantics.
    fn findall(&self, text: &str) -> Vec<Vec<String>>;

    /// Returns all non-overlapping matches as Match objects.
    fn finditer<'a>(&self, text: &'a str) -> Vec<Self::Match<'a>>;
    /// Replaces pattern matches with a replacement string.
    fn sub(&self, repl: &str, text: &str, count: Option<usize>) -> String;
    /// Replaces pattern matches and returns the count of replacements.
    fn subn(&self, repl: &str, text: &str, count: Option<usize>) -> (String, usize);
    /// Returns the original pattern string.
    fn pattern(&self) -> &str;

    /// Returns true if the pattern matches an empty string.
    fn matches_empty(&self) -> bool {
        self.search("").is_some()
    }

    /// Trims the pattern string.
    fn trim(&self) -> &str {
        self.pattern().trim()
    }

    /// Returns true if the pattern string is empty or contains only whitespace.
    fn is_empty(&self) -> bool {
        self.trim().is_empty()
    }

    /// Returns a mapping of group names to group numbers.
    fn groupindex(&self) -> HashMap<Box<str>, usize>;

    /// Returns the number of capturing groups.
    fn groups_count(&self) -> usize;
}

/// Trait defining the pyre Match interface (mirroring Python's re module).
pub trait Match<'a> {
    /// Returns the captured group by index.
    fn group(&self, group: usize) -> Option<&'a str>;
    /// Returns all captured groups.
    fn groups(&self) -> Vec<Option<&'a str>>;
    /// Returns the start position of a group match.
    fn start(&self, group: Option<usize>) -> isize;
    /// Returns the end position of a group match.
    fn end(&self, group: Option<usize>) -> isize;
    /// Returns the (start, end) span of a group match.
    fn span(&self, group: Option<usize>) -> (usize, usize);

    /// Returns the subgroup named `name`.
    fn group_name(&self, name: &str) -> Option<&'a str>;

    /// Returns a mapping of all named subgroups.
    fn groupdict(&self) -> HashMap<Box<str>, Option<&'a str>>;

    /// Returns the string obtained by doing backreference substitution on the template.
    fn expand(&self, template: &str) -> String;
}
