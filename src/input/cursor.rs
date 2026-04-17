// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::Configurable;
use crate::input::tokenizing::TokenizingPatterns;
use crate::util::Cfg;
use crate::util::pyre::Pattern;
use std::fmt::Debug;

pub trait Cursor: Debug + Configurable {
    fn mark(&self) -> usize;
    fn reset(&mut self, mark: usize);
    fn textstr(&self) -> &str;

    fn lookahead(&self, start: usize) -> &str {
        self.textstr()[start..].lines().next().unwrap_or("")
    }

    fn at_end(&self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn match_token(&mut self, token: &str) -> bool;
    fn match_pattern(&mut self, pattern: &Pattern) -> Option<String>;
    fn next_token(&mut self);

    fn pos(&self) -> (usize, usize) {
        self.pos_at(self.mark())
    }

    fn pos_at(&self, mut mark: usize) -> (usize, usize) {
        mark = mark.min(self.textstr().len());
        let text = self.textstr();
        let head = &text[0..mark];
        let line = head.lines().count();
        let col = head.lines().last().map_or(0, |l| l.chars().count());
        (line, col)
    }

    fn set_tokenizing(&mut self, patterns: &TokenizingPatterns);

    fn tokenizing_from_cfg(&self, cfg: &Cfg) -> TokenizingPatterns {
        let wsp: &str = cfg.get("whitespace").map_or("", |s| s);
        let cmt = cfg.get("comments").map_or("", |s| s);
        let eol = cfg.get("eol_comments").map_or("", |s| s);
        TokenizingPatterns::try_new(wsp, cmt, eol).unwrap()
    }
    // // Character classification
    // fn is_name(&self, s: &str) -> bool;
    // fn is_name_char(&self, c: Option<&str>) -> bool;
    //
    // // Navigation
    // fn goto(&mut self, pos: usize);
    // fn move_by(&mut self, n: i64);
    // fn at_end(&self) -> bool;
    // fn at_eol(&self) -> bool;
    //
    // // Movement and Peeking
    // fn next(&mut self) -> Option<&str>;
}
