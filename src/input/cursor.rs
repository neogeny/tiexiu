// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::util::pyre::Pattern as Regex;
use std::fmt::Debug;

pub trait Cursor: Debug {
    fn mark(&self) -> usize;
    fn reset(&mut self, mark: usize);
    fn textstr(&self) -> &str;
    fn at_end(&self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn token(&mut self, token: &str) -> bool;
    fn pattern_re(&mut self, re: &Regex) -> Option<String>;
    fn next_token(&mut self);

    // // Character classification
    // fn is_name(&self, s: &str) -> bool;
    // fn is_name_char(&self, c: Option<&str>) -> bool;
    //
    // /// The full source text from the provider.
    // fn source(&self) -> &'a str;
    //
    // // Navigation
    // fn goto(&mut self, pos: usize);
    // fn move_by(&mut self, n: i64);
    // fn at_end(&self) -> bool;
    // fn at_eol(&self) -> bool;
    //
    // // Movement and Peeking
    // fn next(&mut self) -> Option<&'a str>;
    // fn lookahead(&self) -> &'a str;
    // fn lookahead_pos(&self) -> &'a str;
    //
    // // Tokenization Logic
    // fn current(&self) -> Option<&'a str>;
    // fn match_str(&mut self, token: &str) -> Option<&'a str>;
    // fn match_re(&mut self, pattern: &str) -> Option<&'a str>;
    //
    // // Simplified coordinates
    // fn line_at(&self, pos: Option<usize>) -> usize;
    //
}
