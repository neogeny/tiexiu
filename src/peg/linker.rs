// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ExpKind};
use super::Grammar;
use std::rc::Rc;

impl Grammar {
    pub(super) fn link(&mut self) {
        let len = self.rules.len();
        let mut all_exps: Vec<*mut Exp> = Vec::with_capacity(len);

        for rule_ref in self.rules.values_mut() {
            let rule = Rc::make_mut(rule_ref);
            all_exps.push(&mut rule.exp as *mut Exp);
        }

        for exp_ptr in all_exps {
            let exp = unsafe { &mut *exp_ptr };
            Self::link_exp(exp, self);
        }
    }

    fn link_exp(exp: &mut Exp, grammar: &Grammar) {
        match &mut exp.kind {
            ExpKind::Call { name, rule } => {
                if rule.is_none() &&
                    let Ok(r) = grammar.get_rule_ref(name) {
                        *rule = Some(r);
                }
            }

            ExpKind::RuleInclude { name, exp } => {
                if exp.is_none()
                    && let Ok(rule) = grammar.get_rule(name) {
                        *exp = Some(rule.exp.clone().into());

                }
            }

            ExpKind::Named(_, exp)
            | ExpKind::NamedList(_, exp)
            | ExpKind::Override(exp)
            | ExpKind::OverrideList(exp)
            | ExpKind::Group(exp)
            | ExpKind::SkipGroup(exp)
            | ExpKind::Lookahead(exp)
            | ExpKind::NegativeLookahead(exp)
            | ExpKind::SkipTo(exp)
            | ExpKind::Alt(exp)
            | ExpKind::Optional(exp)
            | ExpKind::Closure(exp)
            | ExpKind::PositiveClosure(exp) => Self::link_exp(exp, grammar),

            ExpKind::Sequence(items) | ExpKind::Choice(items) => {
                for item in items.iter_mut() {
                    Self::link_exp(item, grammar);
                }
            }

            ExpKind::Join { exp: exp, sep }
            | ExpKind::PositiveJoin { exp: exp, sep }
            | ExpKind::Gather { exp: exp, sep }
            | ExpKind::PositiveGather { exp: exp, sep } => {
                Self::link_exp(exp, grammar);
                Self::link_exp(sep, grammar);
            }
            _ => {}
        }
    }
}
