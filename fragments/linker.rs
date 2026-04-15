// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Grammar;
use super::exp::{Exp, ExpKind};
use super::rule::RuleMap;
use std::rc::Rc;

impl Exp {
    fn link(&mut self, rules: &RuleMap) {
        match &mut self.kind {
            ExpKind::RuleInclude { name, exp } => {
                if exp.is_some() {
                    return;
                }
                if let Some(rule) = rules.get(name) {
                    *exp = Some(rule.exp.clone().into())
                }
            }

            ExpKind::Call { name, rule } => {
                if rule.is_some() {
                    return;
                }
                if let Some(r) = rules.get(name) {
                    *rule = Some(r.clone());
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
            | ExpKind::PositiveClosure(exp) => exp.link(rules),

            ExpKind::Sequence(items) | ExpKind::Choice(items) => {
                for item in items.iter_mut() {
                    item.link(rules);
                }
            }

            ExpKind::Join { exp, sep }
            | ExpKind::PositiveJoin { exp, sep }
            | ExpKind::Gather { exp, sep }
            | ExpKind::PositiveGather { exp, sep } => {
                exp.link(rules);
                sep.link(rules);
            }
            _ => {}
        }
    }
}

// impl Linker for Grammar {
//     fn link_call(&mut self, exp: &mut Exp) {
//         if let ExpKind::Call { name, rule } = &mut exp.kind
//             && rule.is_none()
//             && let Ok(r) = self.get_rule_ref(name)
//         {
//             *rule = Some(r);
//         }
//     }
//
//     fn link_rule_include(&mut self, exp: &mut Exp) {
//         if let ExpKind::RuleInclude { name, exp } = &mut exp.kind
//             && exp.is_none()
//             && let Ok(rule) = self.get_rule(name)
//         {
//             *exp = Some(rule.exp.clone().into())
//         }
//     }
// }

impl Grammar {
    pub(super) fn link(&mut self) {
        let map = self.rules.clone();
        for rule in self.rules.values_mut() {
            Rc::make_mut(rule).exp.link(&map);
        }
    }
}
