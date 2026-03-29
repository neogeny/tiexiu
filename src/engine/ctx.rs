// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use crate::input::Cursor;
use crate::model::{ParseResult, Rule};

/// The rule registry. Using Arc allows the Ctx to own it without copying the data.
pub type RuleMap<'g> = HashMap<&'g str, Rule<'g>>;

#[derive(Clone)]
pub struct Ctx<'c> {
    pub cursor: Box<&'c dyn Cursor>,
    pub cut_seen : bool,
    pub error_msg: Option<String>,
    pub rules: RuleMap<'c>,
}

impl<'c> Ctx<'c> {
    pub fn new(cursor: &'c dyn Cursor) -> Self {
        Self {
            cursor: Box::new(cursor),
            cut_seen: false,
            error_msg: None,
            rules: RuleMap::new(),
        }
    }

    pub fn resolve(&self, name: &str) -> &Rule {
        self.rules.get(name).unwrap()
    }

    pub fn mark(&self) -> usize {
        self.cursor.mark()
    }
    
    pub fn cut(&mut self) {
        self.cut_seen = true;
    }
    
    pub fn dot(&mut self) -> ParseResult {
        unimplemented!()
    }
    pub fn eof_check(&mut self) -> ParseResult {
        unimplemented!()
    }

    pub fn next(self) -> Result<Ctx<'c>, Ctx<'c>> {
        // do it with cursor goto(+1)?
        Err(self)
    }

    pub fn token(self, _token: &str) -> ParseResult {
        unimplemented!()
    }
    pub fn pattern(&mut self, _pattern: &str) -> ParseResult {
        unimplemented!()
    }
    pub fn search(&mut self, _pattern: &str) -> ParseResult {
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