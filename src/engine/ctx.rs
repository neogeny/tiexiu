// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::state::ParseStateStack;

pub type ParseResult = Result<Cst, String>;

pub struct RuleInfo {}

pub struct Ctx<'c> {
    states: ParseStateStack<'c>,
}

impl<'c> Ctx<'c> {
    pub fn group<F>(&mut self, body: F) -> ParseResult
    where
        F: FnOnce(&mut Self) -> ParseResult
    {
        self.states.push();
        match body(self) {
            Ok(parsed) => {
                self.states.merge();
                Ok(parsed)
            }
            Err(err) => {
                self.states.undo();
                Err(err)
            }
        }
    }
}
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