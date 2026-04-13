//! Test fixtures for bootstrap tests

use tiexiu::input::StrCursor;
use tiexiu::peg::{Exp, ExpKind, Grammar};
use tiexiu::state::corectx::CoreCtx;

pub fn boot_grammar() -> Grammar {
    tiexiu::json::boot::boot_grammar().expect("Failed to load boot grammar")
}

pub fn parse_ebnf(grammar: &Grammar, text: &str) -> tiexiu::trees::Tree {
    let cursor = StrCursor::new(text);
    let ctx = CoreCtx::new(cursor);
    match grammar.parse(ctx) {
        Ok(s) => s.1,
        Err(f) => panic!("Failed to parse EBNF at mark {}: {:?}", f.mark, f.source),
    }
}

#[allow(dead_code)]
pub fn parse_with_boot(_grammar: &Grammar, text: &str) -> tiexiu::trees::Tree {
    let boot = boot_grammar();
    parse_ebnf(&boot, text)
}

pub fn find_unlinked_calls(exp: &Exp, path: &str) -> Vec<String> {
    let mut result = Vec::new();
    match &exp.kind {
        ExpKind::Call { name, rule } => {
            if rule.is_none() {
                result.push(format!("{}: Call to '{}' is NOT linked", path, name));
            }
        }
        ExpKind::RuleInclude { name, exp: inner } => {
            if inner.is_none() {
                result.push(format!("{}: RuleInclude '{}' is NOT resolved", path, name));
            }
        }
        ExpKind::Sequence(items) => {
            for (i, item) in items.iter().enumerate() {
                result.extend(find_unlinked_calls(item, &format!("{}[{}]", path, i)));
            }
        }
        ExpKind::Choice(options) => {
            for (i, opt) in options.iter().enumerate() {
                result.extend(find_unlinked_calls(opt, &format!("{}[{}]", path, i)));
            }
        }
        ExpKind::Alt(inner) => {
            result.extend(find_unlinked_calls(inner, &format!("{}:alt", path)));
        }
        ExpKind::Named(_, inner)
        | ExpKind::NamedList(_, inner)
        | ExpKind::Optional(inner)
        | ExpKind::Group(inner)
        | ExpKind::Lookahead(inner)
        | ExpKind::NegativeLookahead(inner)
        | ExpKind::SkipTo(inner)
        | ExpKind::Closure(inner)
        | ExpKind::PositiveClosure(inner)
        | ExpKind::SkipGroup(inner)
        | ExpKind::Override(inner)
        | ExpKind::OverrideList(inner) => {
            result.extend(find_unlinked_calls(inner, path));
        }
        ExpKind::Join { exp: e1, sep }
        | ExpKind::PositiveJoin { exp: e1, sep }
        | ExpKind::Gather { exp: e1, sep }
        | ExpKind::PositiveGather { exp: e1, sep } => {
            result.extend(find_unlinked_calls(e1, &format!("{}.exp", path)));
            result.extend(find_unlinked_calls(sep, &format!("{}.sep", path)));
        }
        _ => {}
    }
    result
}

pub fn all_rules_linked(grammar: &Grammar) -> Vec<String> {
    let mut issues = Vec::new();
    for rule in grammar.rules() {
        issues.extend(find_unlinked_calls(
            &rule.exp,
            &format!("rule '{}'", rule.meta.name),
        ));
    }
    issues
}
