// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use super::parser::{ParseResult, Parser};
use super::rule::{Rule, RuleIndex, RuleRef};
use crate::state::Ctx;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub analyzed: bool,
    pub directives: HashMap<String, String>,
    pub keywords: HashSet<String>,
    pub(super) rules: Box<[RuleRef]>,
    index: RuleIndex,
}

impl<C> Parser<C> for Grammar
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.parse_at("start", ctx)
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
        writeln!(f, "@@grammar:: {};", self.name)?;

        for rule in self.rules() {
            writeln!(f, "{}", rule)?;
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl Grammar {
    pub fn new(name: &str, rules: &[Rule]) -> Self {
        let rules: Box<[RuleRef]> = rules
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<_>>()
            .into();
        let mut grammar = Self {
            name: name.to_string(),
            analyzed: false,
            index: Self::new_rule_index(&rules),
            rules,
            directives: HashMap::new(),
            keywords: HashSet::new(),
        };
        grammar.initialize();
        grammar
    }

    pub fn new_rule_index(rules: &[RuleRef]) -> RuleIndex {
        rules
            .iter()
            .enumerate()
            .map(|(i, r)| (r.info.name.clone(), i))
            .collect()
    }

    pub fn initialize(&mut self) {
        self.mark_left_recursion();
        self.link();
    }

    fn parse_at<C: Ctx>(&self, start: &str, ctx: C) -> ParseResult<C> {
        match self.get_rule(start) {
            Ok(rule) => rule.parse(ctx),
            Err(err) => Err(ctx.failure(err)),
        }
    }

    pub fn get_rule(&self, name: &str) -> Result<&Rule, ParseError> {
        self.index
            .get(name)
            .map(|i| self.rules[*i].as_ref())
            .ok_or_else(|| ParseError::RuleNotFound(name.into()))
    }

    pub fn get_rule_ref(&self, name: &str) -> Result<RuleRef, ParseError> {
        self.index
            .get(name)
            .map(|i| self.rules[*i].clone())
            .ok_or_else(|| ParseError::RuleNotFound(name.into()))
    }

    pub fn get_rule_at(&self, id: usize) -> Option<&Rule> {
        self.rules.get(id).map(|r| r.as_ref())
    }

    pub fn get_rule_by_id(&self, id: usize) -> Option<&Rule> {
        self.get_rule_at(id)
    }

    pub fn get_rule_id(&self, name: &str) -> Result<usize, ParseError> {
        self.index
            .get(name)
            .copied()
            .ok_or_else(|| ParseError::RuleNotFound(name.into()))
    }

    pub fn get_rule_mut(&mut self, name: &str) -> Result<&mut Rule, ParseError> {
        match self.index.get(name).copied() {
            Some(i) => Ok(Rc::make_mut(&mut self.rules[i])),
            None => Err(ParseError::RuleNotFound(name.into())),
        }
    }

    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter().map(|r| r.as_ref())
    }

    pub fn rules_mut(&mut self) -> impl Iterator<Item = &mut Rule> {
        self.rules.iter_mut().map(Rc::make_mut)
    }
}
