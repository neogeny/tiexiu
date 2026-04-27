// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Parser;
use super::exp::Exp;
use crate::cfg::types::FlagMap;
use crate::engine::Ctx;
use crate::peg::error::{ParseResult, Yeap};
use crate::trees::Tree;
use crate::types::{Ref, Str};
use indexmap::IndexMap;
use std::rc::Rc;

pub const FLAG_IS_NAME: &str = "is_name";
pub const FLAG_IS_TOKN: &str = "is_tokn";
pub const FLAG_NO_MEMO: &str = "no_memo";
pub const FLAG_IS_MEMO: &str = "is_memo";
pub const FLAG_IS_LREC: &str = "is_lrec";

pub type RuleName = Str;
pub type RuleRef = Rc<Rule>;
pub type RuleIndex = IndexMap<Str, usize>;
pub type Rules = Ref<[Rule]>;
pub type RuleMap = IndexMap<RuleName, RuleRef>;

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: RuleName,
    pub params: Ref<[Str]>,
    // kwparams: dict[str, Any] = field(default_factory=dict)
    pub flags: FlagMap,
    pub exp: Exp,
}

impl<C> Parser<C> for Rule
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        Rule::parse(self, ctx)
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
        self.flags.get(key).copied().unwrap_or(false)
    }

    fn set_flag(&mut self, key: &'static str, value: bool) {
        self.flags.insert(key.into(), value);
    }

    pub fn new(name: &str, params: &[Str], mut exp: Exp) -> Self {
        exp.initialize_caches();
        Self {
            name: name.into(),
            params: params.into(),
            flags: Self::make_flags(false, false, false, true, false),
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
        exp.cache_lookahead();
        Self {
            name: name.into(),
            params: params.into_iter().map(|p| p.into()).collect(),
            flags: Self::make_flags(is_name, is_tokn, no_memo, is_memo, is_lrec),
            exp,
        }
    }

    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        let _text = ctx.cursor().textstr();
        match self.exp.parse(ctx) {
            Ok(Yeap(ctx, mut tree)) => {
                tree = tree.node_tree();
                Ok(Yeap(
                    ctx,
                    if self.params.is_empty() {
                        tree
                    } else {
                        Tree::Node {
                            typename: self.params[0].clone(),
                            tree: tree.into(),
                        }
                    },
                ))
            }
            err => err,
        }
    }

    pub fn is_left_recursive(&self) -> bool {
        self.flag(FLAG_IS_LREC)
    }

    pub fn is_memoizable(&self) -> bool {
        self.is_left_recursive() || self.flag(FLAG_IS_MEMO) && !self.flag(FLAG_NO_MEMO)
    }

    pub fn is_name(&self) -> bool {
        self.has_is_name_flag()
    }

    pub fn is_token(&self) -> bool {
        self.has_is_tokn_flag()
            || self
                .name
                .chars()
                .find(|&c| c != '_')
                .is_some_and(|c| c.is_uppercase())
    }

    pub fn has_is_name_flag(&self) -> bool {
        self.flag(FLAG_IS_NAME)
    }

    pub fn has_is_tokn_flag(&self) -> bool {
        self.flag(FLAG_IS_TOKN)
    }

    pub fn has_no_memo_flag(&self) -> bool {
        self.flag(FLAG_NO_MEMO)
    }

    pub fn has_is_memo_flag(&self) -> bool {
        self.flag(FLAG_IS_MEMO)
    }

    pub fn has_is_lrec_flag(&self) -> bool {
        self.flag(FLAG_IS_LREC)
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
