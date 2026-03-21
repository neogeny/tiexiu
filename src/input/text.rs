// Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

/// A type alias for the boxed trait object used in the state stack.
pub type CursorBox<'a> = Box<dyn Cursor<'a> + 'a>;

/// The Text trait (the dump holder) provides the source and generates cursors.
pub trait Text<'a> {
    fn source(&self) -> &'a str;
    fn new_cursor(&self) -> CursorBox<'a>;
}

/// The abstraction of the position in the input.
pub trait Cursor<'a> {
    /// Returns the Text provider (the dump holder) for this cursor.
    fn text(&self) -> &dyn Text<'a>;

    // Basic state accessors
    fn pos(&self) -> usize;
    fn set_pos(&mut self, pos: usize);

    /// The text from current pos to end.
    fn textstr(&self) -> &'a str;

    /// The full source text from the provider.
    fn source(&self) -> &'a str;

    // Navigation
    fn goto(&mut self, pos: usize);
    fn move_by(&mut self, n: i64);
    fn at_end(&self) -> bool;
    fn at_eol(&self) -> bool;

    // Movement and Peeking
    fn next(&mut self) -> Option<&'a str>;
    fn lookahead(&self) -> &'a str;
    fn lookahead_pos(&self) -> &'a str;

    // Tokenization Logic
    fn current(&self) -> Option<&'a str>;
    fn next_token(&mut self);
    fn match_str(&mut self, token: &str) -> Option<&'a str>;
    fn match_re(&mut self, pattern: &str) -> Option<&'a str>;

    // Character classification
    fn is_name(&self, s: &str) -> bool;
    fn is_name_char(&self, c: Option<&str>) -> bool;

    // Simplified coordinates
    fn line_at(&self, pos: Option<usize>) -> usize;

    /// Required for the State Stack to backtrack.
    fn clone_box(&self) -> CursorBox<'a>;
}

impl<'a> Clone for CursorBox<'a> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}