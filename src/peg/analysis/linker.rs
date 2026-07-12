// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::exp::{Exp, ExpKind};
use crate::grammar::Grammar;
use crate::peg::error::ParseFailure;
use std::sync::Arc;

impl Grammar {
    pub(in crate::peg) fn link(&mut self) -> Result<(), ParseFailure> {
        let len = self.rules.len();
        let mut all_exps: Vec<*mut Exp> = Vec::with_capacity(len);

        for rule_ref in self.rules.values_mut() {
            let rule = Arc::make_mut(rule_ref);
            all_exps.push(&mut rule.exp as *mut Exp);
        }

        for exp_ptr in all_exps {
            let exp = unsafe { &mut *exp_ptr };
            Self::link_exp(exp, self)?;
        }
        Ok(())
    }

    fn link_exp(exp: &mut Exp, grammar: &Grammar) -> Result<(), ParseFailure> {
        match &mut exp.kind {
            ExpKind::Call { name, rule } => {
                let res = grammar.get_rule_ref(name)?;
                *rule = Some(res);
            }
            ExpKind::RuleInclude { name, exp } => {
                if let Ok(rule) = grammar.get_rule(name) {
                    *exp = Some(rule.exp.clone().into());
                }
            }
            _ => {
                if let Some(inner) = exp.kind.single_child_mut() {
                    Self::link_exp(inner, grammar)?;
                } else {
                    for child in exp.kind.children_mut() {
                        Self::link_exp(child, grammar)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_debug() -> Result<(), crate::Error> {
        let boot = crate::api::boot_grammar()?;

        println!("=== Checking boot grammar structure ===\n");

        println!("Rules in boot grammar:");
        for rule in boot.rules() {
            println!("  - {}", rule.name);
        }

        println!("\n=== Checking key rules for linking issues ===\n");

        if let Ok(start_rule) = boot.get_rule("start") {
            println!("Checking 'start' rule:");
            check_exp_for_unlinked(&start_rule.exp, "start", &boot);
        }

        if let Ok(grammar_rule) = boot.get_rule("grammar") {
            println!("\nChecking 'grammar' rule:");
            check_exp_for_unlinked(&grammar_rule.exp, "grammar", &boot);
        }

        if let Ok(rule_rule) = boot.get_rule("rule") {
            println!("\nChecking 'rule' rule:");
            check_exp_for_unlinked(&rule_rule.exp, "rule", &boot);
        }
        Ok(())
    }

    fn check_exp_for_unlinked(exp: &Exp, path: &str, grammar: &Grammar) {
        match &exp.kind {
            ExpKind::Call { name, rule: _ } => {
                println!("  {}: Call to '{}' is NOT linked", path, name);
                match grammar.get_rule(name) {
                    Ok(r) => println!("    BUT '{}' exists in grammar as rule '{}'", name, r.name),
                    Err(_) => println!("    AND '{}' does NOT exist in grammar", name),
                }
            }
            ExpKind::RuleInclude {
                name: ri_name,
                exp: None,
            } => {
                println!("  {}: RuleInclude '{}' is NOT resolved", path, ri_name);
                match grammar.get_rule(ri_name) {
                    Ok(r) => println!("    Rule '{}' exists in grammar", r.name),
                    Err(_) => println!("    AND '{}' does NOT exist in grammar", ri_name),
                }
            }
            _ => {
                for (i, child) in exp.kind.children().into_iter().enumerate() {
                    check_exp_for_unlinked(child, &format!("{}[{}]", path, i), grammar);
                }
            }
        }
    }
}
