// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ExpKind};
use crate::cfg::types::DefineSet;

impl Exp {
    pub(super) fn cache_defines(&mut self) {
        let mut names: DefineSet = DefineSet::new();
        self._defines(&mut names);
        self.df = Some(names.into_iter().collect::<Vec<_>>().into());
    }

    fn _defines(&self, names: &mut DefineSet) {
        match &self.kind {
            ExpKind::Named(name, exp) => {
                names.insert((name.clone(), false));
                exp._defines(names);
            }
            ExpKind::NamedList(name, exp) => {
                names.insert((name.clone(), true));
                exp._defines(names);
            }
            ExpKind::Override(exp) | ExpKind::OverrideList(exp) => {
                exp._defines(names);
            }
            ExpKind::Sequence(arr) => {
                for exp in &**arr {
                    exp._defines(names);
                }
            }
            ExpKind::Choice(arr) => {
                for exp in &**arr {
                    exp._defines(names);
                }
            }
            ExpKind::Alt(exp) => {
                exp._defines(names);
            }
            ExpKind::Optional(exp)
            | ExpKind::Closure(exp)
            | ExpKind::PositiveClosure(exp)
            | ExpKind::SkipGroup(exp)
            | ExpKind::SkipTo(exp)
            | ExpKind::Lookahead(exp)
            | ExpKind::NegativeLookahead(exp) => {
                exp._defines(names);
            }
            ExpKind::Join { exp, .. } => {
                exp._defines(names);
            }
            ExpKind::PositiveJoin { exp, .. } => {
                exp._defines(names);
            }
            ExpKind::Gather { exp, .. } => {
                exp._defines(names);
            }
            ExpKind::PositiveGather { exp, .. } => {
                exp._defines(names);
            }
            ExpKind::RuleInclude { exp: Some(exp), .. } => {
                exp._defines(names);
            }
            ExpKind::Group(exp) => {
                exp._defines(names);
            }
            _ => {}
        }
    }
}
