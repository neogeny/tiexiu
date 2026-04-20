// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Grammar;
use super::exp::{Exp, ExpKind};
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
                if rule.is_none()
                    && let Ok(r) = grammar.get_rule_ref(name)
                {
                    *rule = Some(r);
                }
            }

            ExpKind::RuleInclude { name, exp } => {
                if exp.is_none()
                    && let Ok(rule) = grammar.get_rule(name)
                {
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

            ExpKind::Join { exp, sep }
            | ExpKind::PositiveJoin { exp, sep }
            | ExpKind::Gather { exp, sep }
            | ExpKind::PositiveGather { exp, sep } => {
                Self::link_exp(exp, grammar);
                Self::link_exp(sep, grammar);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::constants::PATH_TATSU_GRAMMAR_EBNF;
    use crate::input::StrCursor;
    use crate::state::StrCtx;

    #[test]
    #[ignore]
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

        println!("\n=== Trying to parse EBNF ===");
        let ebnf_text =
            std::fs::read_to_string(PATH_TATSU_GRAMMAR_EBNF).expect("Failed to read grammar");
        let cursor = StrCursor::new(&ebnf_text);
        let ctx = StrCtx::new(cursor, &[]);
        match boot.parse(ctx) {
            Ok(_) => {
                println!("SUCCESS!");
                Ok(())
            }
            Err(failure) => {
                println!("FAILED: {:?}", failure);
                println!("Error at mark: {}", failure.mark);
                println!("Error message: {:?}", failure.source);
                Err(failure.into())
            }
        }
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
            ExpKind::Sequence(items) => {
                for (i, item) in items.iter().enumerate() {
                    check_exp_for_unlinked(item, &format!("{}[{}]", path, i), grammar);
                }
            }
            ExpKind::Choice(options) => {
                for (i, opt) in options.iter().enumerate() {
                    check_exp_for_unlinked(opt, &format!("{}[{}]", path, i), grammar);
                }
            }
            ExpKind::Alt(inner) => {
                check_exp_for_unlinked(inner, &format!("{}:alt", path), grammar);
            }
            ExpKind::Named(_, inner) | ExpKind::NamedList(_, inner) => {
                check_exp_for_unlinked(inner, path, grammar);
            }
            ExpKind::Optional(inner)
            | ExpKind::Group(inner)
            | ExpKind::Lookahead(inner)
            | ExpKind::NegativeLookahead(inner)
            | ExpKind::SkipTo(inner)
            | ExpKind::Closure(inner)
            | ExpKind::PositiveClosure(inner)
            | ExpKind::SkipGroup(inner)
            | ExpKind::Override(inner)
            | ExpKind::OverrideList(inner) => {
                check_exp_for_unlinked(inner, path, grammar);
            }
            ExpKind::Join { exp: e1, sep }
            | ExpKind::PositiveJoin { exp: e1, sep }
            | ExpKind::Gather { exp: e1, sep }
            | ExpKind::PositiveGather { exp: e1, sep } => {
                check_exp_for_unlinked(e1, &format!("{}.exp", path), grammar);
                check_exp_for_unlinked(sep, &format!("{}.sep", path), grammar);
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
            _ => {}
        }
    }
}
