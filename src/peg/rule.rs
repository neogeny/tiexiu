// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use super::{ParseResult, Parser, S};
use crate::state::Ctx;
use crate::trees::Tree;
use crate::trees::tree::{PruneInfo, PruneInfoRef};
use indexmap::IndexMap;
use std::fmt;

pub type RuleInfo = PruneInfo;
pub type RuleInfoRef = PruneInfoRef;
pub type RuleMap = IndexMap<Box<str>, Rule>;

#[derive(Debug, Clone)]
pub struct Rule {
    pub info: RuleInfoRef,
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
        match self.exp.parse(ctx) {
            Ok(S(ctx, tree)) => Ok(S(ctx, Tree::Pruned(self.info(), tree.trimmed().into()))),
            err => err,
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut params_str = String::new();
        if !self.info.params.is_empty() {
            params_str = format!("[{}]", self.info.params.join(", "));
        }
        let rhs_str = self.exp.to_string();
        let start_str = if rhs_str.lines().count() <= 1 {
            " "
        } else {
            ""
        };
        write!(
            f,
            "{}{}:{}{}",
            self.info.name, params_str, start_str, rhs_str
        )
    }
}

impl Rule {
    pub fn new(name: &str, params: &[&str], mut exp: Exp) -> Self {
        exp.compute_lookahead();
        Self {
            info: RuleInfo {
                name: name.into(),
                params: params.iter().map(|p| (*p).into()).collect(),
            }
            .into(),

            exp,

            is_name: false,
            is_tokn: false,
            no_memo: false,
            is_memo: true,
            is_lrec: false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        name: String,
        params: Vec<String>,
        mut exp: Exp,
        is_name: bool,
        is_tokn: bool,
        no_memo: bool,
        is_memo: bool,
        is_lrec: bool,
    ) -> Self {
        exp.compute_lookahead();
        Self {
            info: RuleInfo {
                name: name.into(),
                params: params.into_iter().map(|p| p.into()).collect(),
            }
            .into(),
            exp,
            is_name,
            is_tokn,
            no_memo,
            is_memo: is_memo && !no_memo,
            is_lrec,
        }
    }

    pub fn info(&self) -> RuleInfoRef {
        self.info.clone()
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
                .info
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
