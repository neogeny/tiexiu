// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Parser;
use super::exp::Exp;
use crate::context::CtxSem;
use crate::peg::error::ParseResult;
use crate::trees::Tree;
use crate::types::Str;
use indexmap::IndexMap;
use std::sync::Arc;

/// Bitmask constants for rule flags.
pub(crate) const FLAG_IS_NAME: u8 = 0b0000_0001;
pub(crate) const FLAG_IS_TOKN: u8 = 0b0000_0010;
pub(crate) const FLAG_IS_MEMO: u8 = 0b0000_0100;
pub(crate) const FLAG_IS_LREC: u8 = 0b0000_1000;
pub(crate) const FLAG_NO_MEMO: u8 = 0b0001_0000;
pub(crate) const FLAG_NO_STAK: u8 = 0b0010_0000;

/// A rule name string.
pub type RuleName = Str;
/// An atomically reference-counted rule.
pub type RuleRef = Arc<Rule>;
/// A map from rule name to its index position.
pub type RuleIndex = IndexMap<Str, usize>;
/// A boxed slice of rules.
pub type Rules = Box<[Rule]>;
/// A map from rule names to rule references.
pub type RuleMap = IndexMap<RuleName, RuleRef>;

/// A PEG parsing rule with name, parameters, flags, and expression.
#[derive(Debug, Clone)]
pub struct Rule {
    /// The rule name.
    pub name: RuleName,
    /// The rule parameters (if any, first is the typename).
    pub params: Box<[Str]>,
    /// Decorator keywords from the EBNF grammar (@name, @nomemo, etc.).
    pub decorators: Box<[Str]>,
    /// Rule flags (is_token, is_memo, is_lrec, etc.).
    pub flags: u8,
    /// The parsing expression for this rule.
    pub exp: Exp,
}

impl<C> Parser<C> for Rule
where
    C: CtxSem,
{
    fn parse_at(&self, ctx: &mut C) -> ParseResult {
        Rule::parse_at(self, ctx)
    }
}

impl Rule {
    fn make_flags(
        is_name: bool,
        is_tokn: bool,
        no_memo: bool,
        no_stak: bool,
        is_memo: bool,
        is_lrec: bool,
    ) -> u8 {
        let mut flags = 0u8;
        if is_name {
            flags |= FLAG_IS_NAME;
        }
        if is_tokn {
            flags |= FLAG_IS_TOKN;
        }
        if is_memo && !no_memo {
            flags |= FLAG_IS_MEMO;
        }
        if is_lrec {
            flags |= FLAG_IS_LREC;
        }
        if no_memo {
            flags |= FLAG_NO_MEMO;
        }
        if no_stak {
            flags |= FLAG_NO_STAK;
        }
        flags
    }

    fn flag(&self, key: u8) -> bool {
        self.flags & key != 0
    }

    fn set_flag(&mut self, key: u8, value: bool) {
        if value {
            self.flags |= key;
        } else {
            self.flags &= !key;
        }
    }

    /// Creates a new Rule with the given name, params, and expression.
    pub fn new(name: &str, params: &[Str], mut exp: Exp) -> Self {
        exp.initialize_caches();
        Self {
            name: name.into(),
            params: params.into(),
            decorators: [].into(),
            flags: Self::make_flags(false, false, false, false, true, false),
            exp,
        }
    }

    /// Creates a Rule from individual parts with flag control.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        name: String,
        params: Vec<String>,
        decorators: Vec<String>,
        mut exp: Exp,
        is_name: bool,
        is_tokn: bool,
        is_memo: bool,
        is_lrec: bool,
        no_memo: bool,
        no_stak: bool,
    ) -> Self {
        exp.cache_lookahead();
        Self {
            name: name.into(),
            params: params.into_iter().map(|p| p.into()).collect(),
            decorators: decorators.into_iter().map(|d| d.into()).collect(),
            flags: Self::make_flags(is_name, is_tokn, no_memo, no_stak, is_memo, is_lrec),
            exp,
        }
    }

    /// Parses at the current context position using this rule's expression.
    pub fn parse_at<C: CtxSem>(&self, ctx: &mut C) -> ParseResult {
        match self.exp.parse_at(ctx) {
            Err(nope) => Err(nope),
            Ok(tree) => {
                let folded = Tree::fold(Arc::unwrap_or_clone(tree).into());
                match ctx.apply_semantics(folded.clone(), self.name.as_ref(), &self.params) {
                    Ok(tree) if *tree != Tree::Bottom => {
                        return Ok(tree);
                    }
                    Ok(_) => {}
                    Err(nope) => {
                        return Err(nope);
                    }
                }
                Ok(if self.params.is_empty() {
                    folded
                } else {
                    let typename = self.params[0].clone();
                    Tree::Node {
                        typename,
                        tree: folded,
                    }
                    .into()
                })
            }
        }
    }

    /// Returns true if this rule is marked as left-recursive.
    pub fn is_left_recursive(&self) -> bool {
        self.flag(FLAG_IS_LREC)
    }

    /// Returns true if this rule should be memoized.
    pub fn is_memoizable(&self) -> bool {
        self.is_left_recursive() || self.flag(FLAG_IS_MEMO) && !self.flag(FLAG_NO_MEMO)
    }

    /// Returns true if this rule has the is_name flag.
    pub fn is_name(&self) -> bool {
        self.has_is_name_flag()
    }

    /// Returns true if this rule is a token rule.
    pub fn is_token(&self) -> bool {
        self.has_is_tokn_flag()
            || self
                .name
                .chars()
                .find(|&c| c != '_')
                .is_some_and(|c| c.is_uppercase())
    }

    /// Returns true if this rule should be traced.
    pub fn should_trace(&self) -> bool {
        !self.has_no_stak_flag() && !self.is_token()
    }

    /// Returns true if the is_name flag is set.
    pub fn has_is_name_flag(&self) -> bool {
        self.flag(FLAG_IS_NAME)
    }

    /// Returns true if the is_tokn flag is set.
    pub fn has_is_tokn_flag(&self) -> bool {
        self.flag(FLAG_IS_TOKN)
    }

    /// Returns true if the no_memo flag is set.
    pub fn has_no_memo_flag(&self) -> bool {
        self.flag(FLAG_NO_MEMO)
    }

    /// Returns true if the is_memo flag is set.
    pub fn has_is_memo_flag(&self) -> bool {
        self.flag(FLAG_IS_MEMO)
    }

    /// Returns true if the is_lrec flag is set.
    pub fn has_is_lrec_flag(&self) -> bool {
        self.flag(FLAG_IS_LREC)
    }

    /// Returns true if the no_stak flag is set.
    pub fn has_no_stak_flag(&self) -> bool {
        self.flag(FLAG_NO_STAK)
    }

    pub(crate) fn reset_left_recursion(&mut self) {
        self.set_flag(FLAG_IS_MEMO, !self.has_no_memo_flag());
        self.set_flag(FLAG_IS_LREC, false);
    }

    pub(crate) fn set_left_recursive(&mut self) {
        self.set_flag(FLAG_IS_LREC, true);
        self.set_flag(FLAG_IS_MEMO, false);
    }

    pub(crate) fn set_no_memo(&mut self) {
        self.set_flag(FLAG_IS_MEMO, false);
    }
}
