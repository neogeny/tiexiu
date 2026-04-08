// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use super::exp::UNRESOLVED_CALL_ID;
use super::fold::Linker;
use super::{Exp, ExpKind, Grammar};

impl Linker for Grammar {
    fn link_call(&mut self, exp: &mut Exp) {
        if let ExpKind::Call { name, id } = &mut exp.kind
            && *id == UNRESOLVED_CALL_ID
        {
            *id = self.get_rule_id(name).unwrap_or(UNRESOLVED_CALL_ID);
        }
    }

    fn link_rule_include(&mut self, exp: &mut Exp) {
        if let ExpKind::RuleInclude { name, exp: inner } = &mut exp.kind {
            self.walk(inner);
            if matches!(inner.kind, ExpKind::Nil)
                && let Ok(rule) = self.get_rule(name)
            {
                *inner = rule.exp.clone().into();
            }
        }
    }
}

impl Grammar {
    pub(super) fn link(&mut self) {
        for i in 0..self.rules().count() {
            let mut exp = std::mem::replace(&mut self.rules_mut().nth(i).unwrap().exp, Exp::nil());
            <Self as Linker>::link(self, &mut exp);
            self.rules_mut().nth(i).unwrap().exp = exp;
        }
    }

    pub fn linked(mut self) -> Result<Self, ParseError> {
        self.link();
        Ok(self)
    }
}
