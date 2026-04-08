// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::fold::Linker;
use super::{Exp, ExpKind, Grammar};

impl Linker for Grammar {
    fn link_call(&mut self, exp: &mut Exp) {
        if let ExpKind::Call { name, rule } = &mut exp.kind
            && rule.is_none()
        {
            *rule = self.get_rule_ref(name).ok();
        }
    }

    fn link_rule_include(&mut self, exp: &mut Exp) {
        if let ExpKind::RuleInclude { name, exp } = &mut exp.kind
            && exp.is_none()
            && let Ok(rule) = self.get_rule(name)
        {
            *exp = Some(rule.exp.clone().into())
        }
    }
}

impl Grammar {
    pub(super) fn link(&mut self) {
        let old_rules = self.rules.clone();
        let mut new_rules: Vec<_> = Vec::with_capacity(old_rules.len());

        for old_rule in &old_rules {
            let mut rule = (**old_rule).clone();
            <Self as Linker>::link(self, &mut rule.exp);
            new_rules.push(rule.into());
        }

        self.rules = new_rules.into();
    }
}
