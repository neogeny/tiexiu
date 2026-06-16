// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::{DisasterReport, ParseFailure};
pub use super::pretty::*;
use super::rule::{Rule, RuleMap, RuleRef};
use crate::api::error::ParseResult;
use crate::cfg::*;
use crate::context::CtxSem;
use crate::peg::ParseFailure::RuleNotFound;
use crate::rule::RuleName;
use crate::types::{Ref, Str};
use crate::{StrCursor, Tree, new_ctx};
use std::sync::Arc;

/// A reference to a grammar keyword string.
pub type KeywordRef = Str;
/// A heap-allocated slice of keyword references.
pub type GrammarKeywords = Ref<[KeywordRef]>;
/// Grammar-level directives configuration.
pub type GrammarDirectives = Cfg;

/// A parsed grammar containing rules, keywords, and directives.
#[derive(Debug, Clone)]
pub struct Grammar {
    /// The grammar name.
    pub name: Str,
    /// Whether the grammar has been analyzed (linked, left-recursion marked).
    pub analyzed: bool,
    /// Grammar directives.
    pub directives: GrammarDirectives,
    /// Sorted, deduplicated keywords for keyword matching.
    pub keywords: GrammarKeywords,
    /// The map of rule names to rules.
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
    C: CtxSem,
{
    fn parse_at(&self, ctx: &mut C) -> ParseResult {
        Grammar::parse_at(self, ctx)
    }
}

impl Grammar {
    /// Creates a new Grammar with the given name and rules.
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

    pub(crate) fn initialize(&mut self) -> Result<(), ParseFailure> {
        self.mark_left_recursion();
        self.link()?;
        self.analyzed = true;
        Ok(())
    }

    /// Returns a reference to the grammar directives.
    pub fn get_directives(&self) -> &GrammarDirectives {
        &self.directives
    }

    pub(crate) fn set_directives(&mut self, directives: GrammarDirectives) {
        self.directives = directives;
        if let Some(CfgKey::Grammar(name)) = self
            .directives
            .iter()
            .find(|k| matches!(k, CfgKey::Grammar(_)))
        {
            self.name = name.clone();
        }
    }

    pub(crate) fn set_keywords(&mut self, keywords: &[KeywordRef]) {
        let mut vec: Vec<KeywordRef> = keywords.to_vec();

        vec.sort();
        vec.dedup();

        self.keywords = vec.into();
    }

    /// Returns true if the given name is a reserved keyword.
    pub fn is_keyword(&self, name: &str) -> bool {
        self.keywords
            .binary_search_by(|k| k.as_str().cmp(name))
            .is_ok()
    }

    /// Returns the name of the start rule ("start" or the first rule).
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

    /// Parses at the current context position using the start rule.
    pub fn parse_at<C: CtxSem>(&self, ctx: &mut C) -> ParseResult {
        match self.start_rule() {
            Ok(start) => self.parse_from(ctx, start.as_ref()),
            Err(e) => Err(ctx.failure(ctx.mark(), e)),
        }
    }

    /// Parses input and returns the resulting Tree on success.
    pub fn parse_tree<C: CtxSem>(&self, ctx: &mut C) -> crate::error::Result<Tree> {
        match self.start_rule() {
            Ok(start) => self.parse_tree_from(ctx, start.as_ref()),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) fn parse_tree_from<C: CtxSem>(
        &self,
        ctx: &mut C,
        start: &str,
    ) -> crate::error::Result<Tree> {
        let start_mark = ctx.mark();
        match self.parse_from(ctx, start) {
            Ok(tree) => Ok(Arc::unwrap_or_clone(tree)),
            Err(_) => Err(ctx
                .furthest_failure()
                .unwrap_or(DisasterReport::new(
                    start_mark,
                    false,
                    ctx,
                    &ParseFailure::Fail,
                ))
                .into()),
        }
    }

    pub(crate) fn parse_from<C: CtxSem>(&self, ctx: &mut C, start: &str) -> ParseResult {
        let start_mark = ctx.mark();
        ctx.configure(&self.directives);
        ctx.set_keywords(&self.keywords);
        if self.directives.contains(&CfgKey::NoLeftRecursion)
            && self.rules().any(|r| r.is_left_recursive())
        {
            return Err(ctx.failure(start_mark, ParseFailure::LeftRecursionDisabled));
        }

        match self.get_rule(start) {
            Ok(rule) => rule.parse_at(ctx),
            Err(err) => Err(ctx.failure(start_mark, err)),
        }
    }

    /// Parses a string input using the given config and returns the resulting Tree.
    pub fn parse_input(&self, text: &str, cfga: &CfgA) -> crate::error::Result<Tree> {
        let cursor = StrCursor::new(text);
        let mut ctx = new_ctx(cursor, cfga);
        if let Some(start) = config(cfga).start() {
            match self.parse_tree_from(&mut ctx, start) {
                Ok(tree) => Ok(tree),
                Err(failure) => Err(failure),
            }
        } else {
            match self.parse_tree(&mut ctx) {
                Ok(tree) => Ok(tree),
                Err(failure) => Err(failure),
            }
        }
    }

    /// Parses a string input from a specific start rule.
    #[allow(dead_code)]
    pub fn parse_input_from(
        &self,
        text: &str,
        start: &str,
        cfga: &CfgA,
    ) -> crate::error::Result<Tree> {
        let cursor = StrCursor::new(text);
        let mut ctx = new_ctx(cursor, cfga);
        match self.parse_tree_from(&mut ctx, start) {
            Ok(tree) => Ok(tree),
            Err(failure) => Err(failure),
        }
    }

    /// Returns a reference to the rule with the given name.
    pub fn get_rule(&self, name: &str) -> Result<&Rule, ParseFailure> {
        self.rules
            .get(name)
            .map(|r| r.as_ref())
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    /// Returns an Arc reference to the rule with the given name.
    pub fn get_rule_ref(&self, name: &str) -> Result<RuleRef, ParseFailure> {
        self.rules
            .get(name)
            .cloned()
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    /// Returns a reference to the rule at the given index position.
    pub fn get_rule_at(&self, id: usize) -> Option<&Rule> {
        self.rules.get_index(id).map(|(_, r)| r.as_ref())
    }

    /// Returns a reference to the rule by its numeric id.
    pub fn get_rule_by_id(&self, id: usize) -> Option<&Rule> {
        self.get_rule_at(id)
    }

    /// Returns the index of the rule with the given name.
    pub fn get_rule_id(&self, name: &str) -> Result<usize, ParseFailure> {
        self.rules
            .get_index_of(name)
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    /// Returns a mutable reference to the rule with the given name.
    #[allow(dead_code)]
    pub fn get_rule_mut(&mut self, name: &str) -> Result<&mut Rule, ParseFailure> {
        self.rules
            .get_mut(name)
            .map(Arc::make_mut)
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    /// Returns an iterator over all rules in the grammar.
    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.values().map(|r| r.as_ref())
    }

    pub(crate) fn rules_mut(&mut self) -> impl Iterator<Item = &mut Rule> {
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
        assert_eq!(grammar.name, "Test");
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
        let rule = Rule::new("start", &[], exp);
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
