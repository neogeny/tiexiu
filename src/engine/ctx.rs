// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use crate::model::ParseResult;



#[derive(Debug, Clone)]
pub struct Ctx<C: Cursor> {
    pub cursor: C,
    pub cut_seen : bool,
    pub error_msg: Option<String>,
}

impl<C: Cursor> Ctx<C> {
    pub fn new(cursor: C) -> Self {
        Self { cursor, cut_seen: false, error_msg: None }
    }

    pub fn mark(&self) -> usize {
        self.cursor.mark()
    }
    
    pub fn cut(&mut self) {
        self.cut_seen = true;
    }
    
    pub fn dot(&mut self) -> ParseResult<C> {
        unimplemented!()
    }
    pub fn eof_check(&mut self) -> ParseResult<C> {
        unimplemented!()
    }

    pub fn next(&mut self) -> Option<Ctx<C>> {
        // do it with cursor goto(+1)?
        unimplemented!()
    }

    pub fn token(&mut self, _token: &str) -> ParseResult<C> {
        unimplemented!()
    }
    pub fn pattern(&mut self, _pattern: &str) -> ParseResult<C> {
        unimplemented!()
    }
    pub fn search(&mut self, _pattern: &str) -> ParseResult<C> {
        unimplemented!()
    }
}
//     fn token(&mut self, token: &str) -> Result<String, String>;
//     fn pattern(&mut self, pattern: &str) -> ParseResult;
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
//     fn fail(&mut self) -> ParseResult;