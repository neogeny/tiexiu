// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::peg::{Exp, ExpKind, Grammar, Rule};

pub trait Linked {
    fn linked(&self) -> Self;
}

pub trait LinkedWith {
    fn linked_with(&self, g: &Grammar) -> Self;
}

impl Linked for Grammar {
    fn linked(&self) -> Grammar {
        let mut rules: Vec<_> = vec![];
        for rule in self.rules() {
            rules.push(rule.linked_with(self))
        }
        Grammar::new(self.name.as_str(), rules.as_slice())
    }
}

impl LinkedWith for Rule {
    fn linked_with(&self, g: &Grammar) -> Rule {
        Rule {
            info: self.info.clone(),
            exp: self.exp.linked_with(g),
        }
    }
}

impl LinkedWith for Exp {
    fn linked_with(&self, g: &Grammar) -> Exp {
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
            | ExpKind::Alert(_, _) => self.clone(),

            ExpKind::Call(name, exp) => {
                let mut exp = exp.linked_with(g);
                if matches!(exp.kind, ExpKind::Nil)
                    && let Some(rule) = g.rulemap.get(name)
                {
                    exp = rule.exp.linked_with(g);
                }
                Exp::call(name, exp)
            }
            ExpKind::RuleInclude { name, exp } => {
                let mut exp = exp.linked_with(g);
                if matches!(exp.kind, ExpKind::Nil)
                    && let Some(rule) = g.rulemap.get(name)
                {
                    exp = rule.exp.linked_with(g);
                }
                Exp::rule_include(name, exp)
            }

            ExpKind::Named(name, exp) => Exp::named(name, exp.linked_with(g)),
            ExpKind::NamedList(name, exp) => Exp::named_list(name, exp.linked_with(g)),

            ExpKind::Override(exp) => Exp::override_node(exp.linked_with(g)),
            ExpKind::OverrideList(exp) => Exp::override_list(exp.linked_with(g)),
            ExpKind::Group(exp) => Exp::group(exp.linked_with(g)),
            ExpKind::SkipGroup(exp) => Exp::skip_group(exp.linked_with(g)),
            ExpKind::Lookahead(exp) => Exp::lookahead(exp.linked_with(g)),
            ExpKind::NegativeLookahead(exp) => Exp::negative_lookahead(exp.linked_with(g)),
            ExpKind::SkipTo(exp) => Exp::skip_to(exp.linked_with(g)),
            ExpKind::Alt(exp) => Exp::alt(exp.linked_with(g)),
            ExpKind::Optional(exp) => Exp::optional(exp.linked_with(g)),
            ExpKind::Closure(exp) => Exp::closure(exp.linked_with(g)),
            ExpKind::PositiveClosure(exp) => Exp::positive_closure(exp.linked_with(g)),

            ExpKind::Sequence(elements) => {
                Exp::sequence(elements.iter().map(|e| e.linked_with(g)).collect())
            }
            ExpKind::Choice(options) => {
                Exp::choice(options.iter().map(|e| e.linked_with(g)).collect())
            }

            ExpKind::Join { exp: e, sep: s } => Exp::join(e.linked_with(g), s.linked_with(g)),
            ExpKind::PositiveJoin { exp: e, sep: s } => {
                Exp::positive_join(e.linked_with(g), s.linked_with(g))
            }
            ExpKind::Gather { exp: e, sep: s } => Exp::gather(e.linked_with(g), s.linked_with(g)),
            ExpKind::PositiveGather { exp: e, sep: s } => {
                Exp::positive_gather(e.linked_with(g), s.linked_with(g))
            }
        }
    }
}
