// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::state::ParseStateStack;
use super::parsed::Parsed;

pub type ParseResult = Result<Parsed, String>;

pub struct RuleInfo {}

pub struct Ctx<'c> {
    states: ParseStateStack<'c>,
}

impl<'c> Ctx<'c> {
    // State management
    pub fn state_push(&mut self) {}
    pub fn state_merge(&mut self) {}
    pub fn state_undo(&mut self) {}
    pub fn state_pop(&mut self) {}
// 
//     // Parsing primitives
//     fn dot(&mut self) -> ParseResult;
//     fn token(&mut self, token: &str) -> Result<String, String>;
//     fn pattern(&mut self, pattern: &str) -> ParseResult;
//     fn constant(&mut self, literal: &str) -> ParseResult;
// 
//     // Rule and Dispatch
//     fn call(&mut self, ri: &RuleInfo) -> ParseResult;
//     fn find_rule(&self, name: &str) -> Box<dyn Fn(&mut dyn Ctx) -> ParseResult>;
// 
//     // High-level combinators (taking closures to match Python's Func)
//     fn closure(&mut self, exp: &dyn Fn(&mut dyn Ctx) -> ParseResult) -> ParseResult;
//     fn positive_closure(&mut self, exp: &dyn Fn(&mut dyn Ctx) -> ParseResult) -> ParseResult;
//     fn choice(&mut self, options: Vec<Box<dyn Fn(&mut dyn Ctx) -> ParseResult>>) -> ParseResult;
// 
//     // AST / Result management
//     fn define(&mut self, keys: Vec<String>, add_keys: Option<Vec<String>>);
//     fn last_node(&self) -> ParseResult;
// 
//     // Errors and Guards
//     fn eof_check(&mut self) -> Result<(), String>;
//     fn fail(&mut self) -> ParseResult;
//     fn cut(&mut self);
}