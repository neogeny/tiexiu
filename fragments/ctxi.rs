// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait CtxI {
    fn cursor(&self) -> &dyn Cursor;
    fn cursor_mut(&mut self) -> &mut dyn Cursor;
    fn stack(&self) -> TokenList;
    fn enter(&mut self, name: &str);
    fn failure(&self, start: usize, source: ParseError) -> Nope;
    fn eof_check(&mut self) -> bool;
    fn dot(&mut self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn get_pattern(&self, pattern: &str) -> Pattern;
    fn match_token(&mut self, token: &str) -> bool;
    fn match_pattern(&mut self, pattern: &str) -> Option<String>;
    fn next_token(&mut self);
    fn key(&mut self, name: &str) -> Key;
    fn mark(&self) -> usize;
    fn reset(&mut self, mark: usize);
    fn memo(&mut self, key: &Key) -> Option<Memo>;
    fn memoize(&mut self, key: &Key, tree: &Tree);
    fn cut_seen(&self) -> bool;
    fn setcut(&mut self);
    fn uncut(&mut self);
    fn restore_cut(&mut self, was_cut: bool);
    fn prune_cache(&mut self);
}
