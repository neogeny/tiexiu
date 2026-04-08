// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use super::{Exp, ExpKind, Grammar, Rule};

impl Grammar {
    pub fn crossrefed(&self) -> Result<Grammar, ParseError> {
        let mut rules: Vec<_> = vec![];
        for rule in self.rules() {
            rules.push(rule.crossrefed(self)?)
        }
        Ok(Grammar::new(self.name.as_str(), rules.as_slice()))
    }
}

impl Rule {
    pub fn crossrefed(&self, g: &Grammar) -> Result<Rule, ParseError> {
        Ok(Rule {
            info: self.info.clone(),
            exp: self.exp.crossrefed(g)?,
        })
    }
}

impl Exp {
    fn xrefd(&self, name: &str, g: &Grammar) -> Result<Exp, ParseError> {
        let mut exp = self.crossrefed(g)?;
        if matches!(exp.kind, ExpKind::Nil) {
            let rule = g.get_rule(name)?;
            exp = rule.exp.crossrefed(g)?;
        }
        Ok(exp)
    }

    pub fn crossrefed(&self, g: &Grammar) -> Result<Exp, ParseError> {
        match &self.kind {
            ExpKind::Nil
            | ExpKind::Cut
            | ExpKind::Void
            | ExpKind::Fail
            | ExpKind::Dot
            | ExpKind::Eof
            | ExpKind::Token(_)
            | ExpKind::Pattern(_)
            | ExpKind::Constant(_)
            | ExpKind::Alert(_, _) => Ok(self.clone()),

            ExpKind::Call(name, exp) => {
                let new = exp.xrefd(name, g)?;
                Ok(Exp::call(name, new))
            }
            ExpKind::RuleInclude { name, exp } => {
                let new = exp.xrefd(name, g)?;
                Ok(Exp::rule_include(name, new))
            }

            ExpKind::Named(name, exp) => Ok(Exp::named(name, exp.crossrefed(g)?)),
            ExpKind::NamedList(name, exp) => Ok(Exp::named_list(name, exp.crossrefed(g)?)),

            ExpKind::Override(exp) => Ok(Exp::override_node(exp.crossrefed(g)?)),
            ExpKind::OverrideList(exp) => Ok(Exp::override_list(exp.crossrefed(g)?)),
            ExpKind::Group(exp) => Ok(Exp::group(exp.crossrefed(g)?)),
            ExpKind::SkipGroup(exp) => Ok(Exp::skip_group(exp.crossrefed(g)?)),
            ExpKind::Lookahead(exp) => Ok(Exp::lookahead(exp.crossrefed(g)?)),
            ExpKind::NegativeLookahead(exp) => Ok(Exp::negative_lookahead(exp.crossrefed(g)?)),
            ExpKind::SkipTo(exp) => Ok(Exp::skip_to(exp.crossrefed(g)?)),
            ExpKind::Alt(exp) => Ok(Exp::alt(exp.crossrefed(g)?)),
            ExpKind::Optional(exp) => Ok(Exp::optional(exp.crossrefed(g)?)),
            ExpKind::Closure(exp) => Ok(Exp::closure(exp.crossrefed(g)?)),
            ExpKind::PositiveClosure(exp) => Ok(Exp::positive_closure(exp.crossrefed(g)?)),

            ExpKind::Sequence(elements) => Ok(Exp::sequence(
                elements
                    .iter()
                    .map(|e| e.crossrefed(g))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            ExpKind::Choice(options) => Ok(Exp::choice(
                options
                    .iter()
                    .map(|e| e.crossrefed(g))
                    .collect::<Result<Vec<_>, _>>()?,
            )),

            ExpKind::Join { exp: e, sep: s } => Ok(Exp::join(e.crossrefed(g)?, s.crossrefed(g)?)),
            ExpKind::PositiveJoin { exp: e, sep: s } => {
                Ok(Exp::positive_join(e.crossrefed(g)?, s.crossrefed(g)?))
            }
            ExpKind::Gather { exp: e, sep: s } => {
                Ok(Exp::gather(e.crossrefed(g)?, s.crossrefed(g)?))
            }
            ExpKind::PositiveGather { exp: e, sep: s } => {
                Ok(Exp::positive_gather(e.crossrefed(g)?, s.crossrefed(g)?))
            }
        }
    }
}
