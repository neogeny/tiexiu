// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use super::parser::{ParseResult, Parser};
use super::rule::{Rule, RuleMap};
use crate::state::Ctx;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub analyzed: bool,
    pub directives: HashMap<String, String>,
    pub keywords: HashSet<String>,
    pub rulemap: RuleMap,
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
        let rulemap = Self::new_rulemap(rules);
        let mut grammar = Self {
            name: name.to_string(),
            analyzed: false,
            rulemap,
            directives: HashMap::new(),
            keywords: HashSet::new(),
        };
        grammar.initialize();
        grammar
    }

    pub fn new_rulemap(rules: &[Rule]) -> RuleMap {
        rules
            .iter()
            .cloned()
            .map(|r| (r.info.name.clone(), r))
            .collect()
    }

    pub fn initialize(&mut self) {
        self.mark_left_recursion();
    }

    fn parse_at<C: Ctx>(&self, start: &str, ctx: C) -> ParseResult<C> {
        if let Some(rule) = self.rulemap.get(start) {
            rule.parse(ctx)
        } else {
            Err(ctx.failure(ParseError::RuleNotFound(start.into())))
        }
    }

    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.rulemap.values()
    }

    pub fn rules_mut(&mut self) -> impl Iterator<Item = &mut Rule> {
        self.rulemap.values_mut()
    }
}
