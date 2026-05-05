// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::{ParseFailure, Yeap};
pub use super::pretty::*;
use super::rule::{Rule, RuleMap, RuleRef};
use crate::api::error::{DisasterReport, ParseResult};
use crate::cfg::*;
use crate::context::Ctx;
use crate::peg::ParseFailure::RuleNotFound;
use crate::rule::RuleName;
use crate::types::{Ref, Str};
use crate::{StrCursor, Tree, new_ctx};
use std::rc::Rc;
use std::sync::Arc;

pub type KeywordRef = Str;
pub type GrammarKeywords = Ref<[KeywordRef]>;
pub type GrammarDirectives = Cfg;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: Str,
    pub analyzed: bool,
    pub directives: GrammarDirectives,
    pub keywords: GrammarKeywords,
    pub rules: RuleMap,
}

impl Default for Grammar {
    #[inline]
    fn default() -> Self {
        Self::new("Default", &[])
    }
}

impl<C> crate::peg::Parser<C> for Grammar
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        Grammar::parse(self, ctx)
    }
}

impl Grammar {
    pub fn new(name: &str, rules: &[RuleRef]) -> Self {
        let rules: RuleMap = rules.iter().cloned().map(|r| (r.name.clone(), r)).collect();
        Self {
            name: name.into(),
            analyzed: false,
            rules,
            directives: GrammarDirectives::default(),
            keywords: [].into(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), ParseFailure> {
        self.mark_left_recursion();
        self.link()?;
        self.analyzed = true;
        Ok(())
    }

    pub fn get_directives(&self) -> &GrammarDirectives {
        &self.directives
    }

    pub fn set_directives(&mut self, directives: GrammarDirectives) {
        self.directives = directives;
        if let Some(CfgKey::Grammar(name)) = self
            .directives
            .iter()
            .find(|k| matches!(k, CfgKey::Grammar(_)))
        {
            self.name = name.clone().into();
        }
    }

    pub fn set_keywords(&mut self, keywords: &[KeywordRef]) {
        let mut vec: Vec<KeywordRef> = keywords.to_vec();

        vec.sort();
        vec.dedup();

        self.keywords = vec.into();
    }

    pub fn is_keyword(&self, name: &str) -> bool {
        self.keywords
            .binary_search_by(|k| k.as_ref().cmp(name))
            .is_ok()
    }

    pub fn start_rule(&self) -> Result<RuleName, ParseFailure> {
        if self.rules.is_empty() {
            return Err(ParseFailure::NoRulesInGrammar);
        }
        let start = "start";
        match self.rules.get(start) {
            Some(rule) => Ok(rule.name.clone()),
            None => self
                .rules
                .get_index(0)
                .map_or(Err(RuleNotFound(start.into())), |(_, r)| Ok(r.name.clone())),
        }
    }

    pub fn parse<C: Ctx>(&self, mut ctx: C) -> ParseResult<C> {
        match self.start_rule() {
            Ok(start) => self.parse_from(start.as_ref(), ctx),
            Err(e) => Err(ctx.failure(ctx.mark(), e)),
        }
    }

    pub fn parse_tree<C: Ctx>(&self, ctx: C) -> crate::error::Result<Tree> {
        match self.start_rule() {
            Ok(start) => self.parse_tree_from(start.as_ref(), ctx),
            Err(e) => Err(e.into()),
        }
    }

    pub fn parse_tree_from<C: Ctx>(&self, start: &str, mut ctx: C) -> crate::error::Result<Tree> {
        let start_mark = ctx.mark();
        match self.parse_from(start, ctx.push()) {
            Ok(Yeap(_, tree)) => Ok(Rc::unwrap_or_clone(tree)),
            Err(_) => Err(ctx
                .furthest_failure()
                .unwrap_or(DisasterReport::new(start_mark, &ctx, &ParseFailure::Fail))
                .into()),
        }
    }

    pub fn parse_from<C: Ctx>(&self, start: &str, mut ctx: C) -> ParseResult<C> {
        let start_mark = ctx.mark();
        ctx.configure(&self.directives);
        ctx.set_keywords(&self.keywords);
        match self.get_rule(start) {
            Ok(rule) => rule.parse(ctx.push()),
            Err(err) => Err(ctx.failure(start_mark, err)),
        }
    }

    pub fn parse_input(&self, text: &str, cfg: &CfgA) -> crate::error::Result<Tree> {
        let cursor = StrCursor::new(text);
        let ctx = new_ctx(cursor, cfg);
        match self.parse_tree(ctx) {
            Ok(tree) => Ok(tree),
            Err(failure) => Err(failure),
        }
    }

    pub fn get_rule(&self, name: &str) -> Result<&Rule, ParseFailure> {
        self.rules
            .get(name)
            .map(|r| r.as_ref())
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    pub fn get_rule_ref(&self, name: &str) -> Result<RuleRef, ParseFailure> {
        self.rules
            .get(name)
            .cloned()
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    pub fn get_rule_at(&self, id: usize) -> Option<&Rule> {
        self.rules.get_index(id).map(|(_, r)| r.as_ref())
    }

    pub fn get_rule_by_id(&self, id: usize) -> Option<&Rule> {
        self.get_rule_at(id)
    }

    pub fn get_rule_id(&self, name: &str) -> Result<usize, ParseFailure> {
        self.rules
            .get_index_of(name)
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    pub fn get_rule_mut(&mut self, name: &str) -> Result<&mut Rule, ParseFailure> {
        self.rules
            .get_mut(name)
            .map(Arc::make_mut)
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.values().map(|r| r.as_ref())
    }

    pub fn rules_mut(&mut self) -> impl Iterator<Item = &mut Rule> {
        self.rules.values_mut().map(Arc::make_mut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::peg::Exp;
    use crate::peg::rule::Rule;

    #[test]
    fn new_grammar() {
        let grammar = Grammar::new("Test", &[]);
        assert_eq!(grammar.name, "Test".into());
    }

    #[test]
    fn grammar_with_rules() {
        let exp = Exp::nil();
        let rule = Rule::new("start", &[], exp);
        let grammar = Grammar::new("Test", &[rule.into()]);
        let count = grammar.rules().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn get_rule() {
        let exp = Exp::nil();
        let rule = Rule::new("start", &[], exp.clone());
        let grammar = Grammar::new("Test", &[rule.into()]);
        assert!(grammar.get_rule("start").is_ok());
    }

    #[test]
    fn get_rule_not_found() {
        let grammar = Grammar::new("Test", &[]);
        assert!(grammar.get_rule("missing").is_err());
    }

    #[test]
    fn grammar_analyzed() {
        let mut grammar = Grammar::new("Test", &[]);
        grammar.initialize().unwrap();
        assert!(grammar.analyzed);
    }
}
