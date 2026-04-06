// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Exp, ParseResult, Parser};
use crate::state::Ctx;
use std::collections::HashMap;
use std::fmt;

pub type RuleMap = HashMap<String, Rule>;

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub params: Vec<String>,
    // decorators: HashMap<String, String>;

    // NOTE: these come from the grammar definition
    pub is_name: bool,
    pub is_tokn: bool,
    pub no_memo: bool,

    // NOTE: these belong to the left-recursion analyzer
    pub is_memo: bool,
    pub is_lrec: bool,

    pub exp: Exp,
    // kwparams: dict[str, Any] = field(default_factory=dict)
}

impl<C> Parser<C> for Rule
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.exp.parse(ctx)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut params_str = String::new();
        if !self.params.is_empty() {
            params_str = format!("[{}]", self.params.join(", "));
        }
        let rhs_str = self.exp.to_string();
        let start_str = if rhs_str.lines().count() <= 1 {
            " "
        } else {
            ""
        };
        write!(f, "{}{}:{}{}", self.name, params_str, start_str, rhs_str)
    }
}

impl Rule {
    pub fn new(name: &str, _params: Vec<String>, rhs: Exp) -> Self {
        Self {
            name: name.to_string(),
            params: vec![],
            exp: rhs,

            is_name: false,
            is_tokn: false,
            no_memo: false,
            is_memo: true,
            is_lrec: false,
        }
    }

    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        (self as &dyn Parser<C>).parse(ctx)
    }

    pub fn is_left_recursive(&self) -> bool {
        self.is_lrec
    }

    pub fn is_memoizable(&self) -> bool {
        self.is_memo
    }

    pub fn is_identifier(&self) -> bool {
        self.is_name
    }

    pub fn is_token(&self) -> bool {
        self.is_tokn
            || self
                .name
                .chars()
                .find(|&c| c != '_')
                .is_some_and(|c| c.is_uppercase())
    }

    pub fn reset_left_recursion(&mut self) {
        self.is_memo = !self.no_memo;
        self.is_lrec = false;
    }

    pub fn set_left_recursive(&mut self) {
        self.is_lrec = true;
        self.is_memo = false;
    }

    pub fn set_no_memo(&mut self) {
        self.is_memo = false;
    }
}
