// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use super::{ParseResult, Parser, S};
use crate::state::Ctx;
use crate::trees::Tree;
use crate::trees::tree::{FlagMap, PruneInfo, PruneInfoRef};
use indexmap::IndexMap;
use std::fmt;
use std::rc::Rc;

pub const FLAG_IS_NAME: &str = "is_name";
pub const FLAG_IS_TOKN: &str = "is_tokn";
pub const FLAG_NO_MEMO: &str = "no_memo";
pub const FLAG_IS_MEMO: &str = "is_memo";
pub const FLAG_IS_LREC: &str = "is_lrec";

pub type RuleInfo = PruneInfo;
pub type RuleInfoRef = PruneInfoRef;
pub type RuleMap = IndexMap<Box<str>, Rule>;

#[derive(Debug, Clone)]
pub struct Rule {
    pub info: RuleInfoRef,
    pub exp: Exp,
    // kwparams: dict[str, Any] = field(default_factory=dict)
}

impl<C> Parser<C> for Rule
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        match self.exp.parse(ctx) {
            Ok(S(ctx, tree)) => Ok(S(
                ctx,
                Tree::Pruned(self.info.clone(), tree.trimmed().into()),
            )),
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
    fn make_flags(
        is_name: bool,
        is_tokn: bool,
        no_memo: bool,
        is_memo: bool,
        is_lrec: bool,
    ) -> FlagMap {
        let mut flags = FlagMap::new();
        flags.insert(FLAG_IS_NAME.into(), is_name);
        flags.insert(FLAG_IS_TOKN.into(), is_tokn);
        flags.insert(FLAG_NO_MEMO.into(), no_memo);
        flags.insert(FLAG_IS_MEMO.into(), is_memo && !no_memo);
        flags.insert(FLAG_IS_LREC.into(), is_lrec);
        flags
    }

    fn flag(&self, key: &str) -> bool {
        self.info.flags.get(key).copied().unwrap_or(false)
    }

    fn set_flag(&mut self, key: &'static str, value: bool) {
        Rc::make_mut(&mut self.info).flags.insert(key.into(), value);
    }

    pub fn new(name: &str, params: &[&str], mut exp: Exp) -> Self {
        exp.compute_lookahead();
        Self {
            info: RuleInfo {
                name: name.into(),
                params: params.iter().map(|p| (*p).into()).collect(),
                flags: Self::make_flags(false, false, false, true, false),
            }
            .into(),

            exp,
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
                flags: Self::make_flags(is_name, is_tokn, no_memo, is_memo, is_lrec),
            }
            .into(),
            exp,
        }
    }

    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        (self as &dyn Parser<C>).parse(ctx)
    }

    pub fn is_left_recursive(&self) -> bool {
        self.flag(FLAG_IS_LREC)
    }

    pub fn is_memoizable(&self) -> bool {
        !self.flag(FLAG_NO_MEMO) && self.flag(FLAG_IS_MEMO)
    }

    pub fn is_identifier(&self) -> bool {
        self.flag(FLAG_IS_NAME)
    }

    pub fn has_token_flag(&self) -> bool {
        self.flag(FLAG_IS_TOKN)
    }

    pub fn has_no_memo_flag(&self) -> bool {
        self.flag(FLAG_NO_MEMO)
    }

    pub fn has_memo_flag(&self) -> bool {
        self.flag(FLAG_IS_MEMO)
    }

    pub fn has_left_recursion_flag(&self) -> bool {
        self.flag(FLAG_IS_LREC)
    }

    pub fn is_token(&self) -> bool {
        self.has_token_flag()
            || self
                .info
                .name
                .chars()
                .find(|&c| c != '_')
                .is_some_and(|c| c.is_uppercase())
    }

    pub fn reset_left_recursion(&mut self) {
        self.set_flag(FLAG_IS_MEMO, !self.has_no_memo_flag());
        self.set_flag(FLAG_IS_LREC, false);
    }

    pub fn set_left_recursive(&mut self) {
        self.set_flag(FLAG_IS_LREC, true);
        self.set_flag(FLAG_IS_MEMO, false);
    }

    pub fn set_no_memo(&mut self) {
        self.set_flag(FLAG_IS_MEMO, false);
    }
}
