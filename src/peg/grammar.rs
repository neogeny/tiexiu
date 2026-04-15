// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use super::parser::{ParseResult, Parser};
use super::rule::{Rule, RuleMap, RuleRef};
use crate::peg::ParseError::RuleNotFound;
use crate::state::Ctx;
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub analyzed: bool,
    pub directives: HashMap<String, String>,
    pub keywords: HashSet<String>,
    pub rules: RuleMap,
}

impl<C> Parser<C> for Grammar
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        Grammar::parse(self, ctx)
    }
}

impl Default for Grammar {
    #[inline]
    fn default() -> Self {
        Self::new("Default", &[])
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "@@grammar:: {}", self.name)?;

        for rule in self.rules() {
            writeln!(f, "{}", rule)?;
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl Grammar {
    pub fn new(name: &str, rules: &[Rule]) -> Self {
        let rules: IndexMap<Box<str>, RuleRef> = rules
            .iter()
            .cloned()
            .map(|r| (r.name.clone(), r.into()))
            .collect();
        let mut grammar = Self {
            name: name.to_string(),
            analyzed: false,
            rules,
            directives: HashMap::new(),
            keywords: HashSet::new(),
        };
        grammar.initialize();
        grammar
    }

    pub fn initialize(&mut self) {
        self.mark_left_recursion();
        self.link();
    }

    pub fn start_rule(&self) -> Result<&Rule, ParseError> {
        if self.rules.is_empty() {
            return Err(ParseError::NoRulesInGrammar);
        }
        let start = "start";
        self.rules
            .get(start)
            .map(|r| r.as_ref())
            .or_else(|| self.rules.get_index(0).map(|(_, r)| r.as_ref()))
            .ok_or_else(|| RuleNotFound(start.into()))
    }

    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        let start_mark = ctx.mark();
        match self.start_rule() {
            Ok(rule) => rule.parse(ctx),
            Err(err) => Err(ctx.failure(start_mark, err)),
        }
    }

    pub fn parse_from<C: Ctx>(&self, start: &str, ctx: C) -> ParseResult<C> {
        let start_mark = ctx.mark();
        match self.get_rule(start) {
            Ok(rule) => rule.parse(ctx),
            Err(err) => Err(ctx.failure(start_mark, err)),
        }
    }

    pub fn get_rule(&self, name: &str) -> Result<&Rule, ParseError> {
        self.rules
            .get(name)
            .map(|r| r.as_ref())
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    pub fn get_rule_ref(&self, name: &str) -> Result<RuleRef, ParseError> {
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

    pub fn get_rule_id(&self, name: &str) -> Result<usize, ParseError> {
        self.rules
            .get_index_of(name)
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    pub fn get_rule_mut(&mut self, name: &str) -> Result<&mut Rule, ParseError> {
        self.rules
            .get_mut(name)
            .map(Rc::make_mut)
            .ok_or_else(|| RuleNotFound(name.into()))
    }

    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.values().map(|r| r.as_ref())
    }

    pub fn rules_mut(&mut self) -> impl Iterator<Item = &mut Rule> {
        self.rules.values_mut().map(Rc::make_mut)
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
        let grammar = Grammar::new("Test", &[rule]);
        let count = grammar.rules().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn get_rule() {
        let exp = Exp::nil();
        let rule = Rule::new("start", &[], exp.clone());
        let grammar = Grammar::new("Test", &[rule]);
        assert!(grammar.get_rule("start").is_ok());
    }

    #[test]
    fn get_rule_not_found() {
        let grammar = Grammar::new("Test", &[]);
        assert!(grammar.get_rule("missing").is_err());
    }

    #[test]
    fn grammar_not_analyzed() {
        let grammar = Grammar::new("Test", &[]);
        assert!(!grammar.analyzed);
    }
}
