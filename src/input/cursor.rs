// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::*;
use crate::input::Error;
use crate::input::tokenizing::TokenizingPatterns;
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
    fn match_eol(&mut self) -> bool;
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

    fn tokenizing_from_cfg(&self, cfg: &CfgBox) -> Result<TokenizingPatterns, Error> {
        type P = TokenizingPatterns;
        let mut wsp = P::DEFAULT_WSP;
        let mut cmt = P::DEFAULT_CMT;
        let mut eol = P::DEFAULT_EOL;

        for opt in cfg.iter() {
            match opt {
                Cfg::Wsp(p) => wsp = p.as_str(),
                Cfg::Cmt(p) => cmt = p.as_str(),
                Cfg::Eol(p) => eol = p.as_str(),
                _ => {}
            }
        }

        TokenizingPatterns::try_new(wsp, cmt, eol)
    }
}
