// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains logic for generating Rust model code from a PEG grammar.

use crate::cfg::types::Str;
use crate::peg::exp::{Exp, ExpKind, ERef, ERefArr};
use crate::peg::rule::{Rule, RuleMap};
use indexmap::IndexMap;
use std::rc::Rc;

/// Represents a field in a generated Rust model struct.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    /// The original name from the grammar (e.g., 'my_field').
    pub name: Str,
    /// The generated Rust field name (e.g., 'MyField').
    pub rust_name: Str,
    /// The inferred Rust type string (e.g., 'Rc<String>', 'Rc<Expression>', 'Vec<Rc<Statement>>').
    pub rust_type: Str,
    /// True if the field is derived from a NamedList expression.
    pub from_named_list: bool,
}

/// Represents the information needed to generate a Rust model struct for a grammar rule.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelInfo {
    /// The name of the rule in the grammar.
    pub rule_name: Str,
    /// The requested type name for the model (from rule parameters, e.g., 'MyModel').
    pub type_name: Str,
    /// The fields collected for this model struct.
    pub fields: Vec<FieldDef>,
}

/// Capitalizes the first letter of a string, converting it to PascalCase for Rust struct field names.
fn pascal_case(s: &str) -> Str {
    if s.is_empty() {
        return "".into();
    }
    let mut chars = s.chars();
    let first_char = chars.next().unwrap().to_ascii_uppercase();
    format!("{}{}", first_char, chars.as_str()).into()
}

/// Recursively collects named fields from a PEG expression tree.
///
/// This function walks the `Exp` tree to identify `ExpKind::Named` and `ExpKind::NamedList`
/// nodes, which correspond to fields in the generated model structs. It infers the
/// Rust type for each field based on the inner expression.
fn collect_fields(fields: &mut Vec<FieldDef>, exp: &Exp, rules: &RuleMap) {
    match &exp.kind {
        ExpKind::Named(name, inner_exp) => {
            let type_name = resolve_type_name(inner_exp, rules);
            let rust_type: Str = match (type_name, is_list_exp(inner_exp)) {
                (Some(tn), true) => format!("Vec<Rc<{}>>", tn).into(), // Named list of typed ref
                (Some(tn), false) => format!("Rc<{}>", tn).into(),    // Named typed ref
                (None, true) if is_constant_text(inner_exp, rules) => "Vec<Rc<String>>".into(), // Named list of string
                (None, false) if is_constant_text(inner_exp, rules) => "Rc<String>".into(),    // Named string
                _ => "Rc<crate::trees::Tree>".into(), // Default to Rc<Tree> for unknown/untyped
            };
            fields.push(FieldDef {
                name: name.clone(),
                rust_name: pascal_case(name),
                rust_type,
                from_named_list: false,
            });
        }
        ExpKind::NamedList(name, inner_exp) => {
            let type_name = resolve_type_name(inner_exp, rules);
            let rust_type: Str = match type_name {
                Some(tn) => format!("Vec<Rc<{}>>", tn).into(), // NamedList of typed ref
                None if is_constant_text(inner_exp, rules) => "Vec<Rc<String>>".into(), // NamedList of string
                _ => "Vec<Rc<crate::trees::Tree>>".into(), // Default to Vec<Rc<Tree>>
            };
            fields.push(FieldDef {
                name: name.clone(),
                rust_name: pascal_case(name),
                rust_type,
                from_named_list: true,
            });
        }
        ExpKind::Sequence(elements) | ExpKind::Choice(elements) => {
            for element in elements.iter() {
                collect_fields(fields, element, rules);
            }
        }
        ExpKind::Optional(inner_exp)
        | ExpKind::Group(inner_exp)
        | ExpKind::SkipGroup(inner_exp)
        | ExpKind::Closure(inner_exp)
        | ExpKind::PositiveClosure(inner_exp)
        | ExpKind::Lookahead(inner_exp)
        | ExpKind::NegativeLookahead(inner_exp)
        | ExpKind::Override(inner_exp)
        | ExpKind::OverrideList(inner_exp)
        | ExpKind::Alt(inner_exp) => {
            collect_fields(fields, inner_exp, rules);
        }
        ExpKind::Join { exp: inner_exp, sep: _ }
        | ExpKind::PositiveJoin { exp: inner_exp, sep: _ }
        | ExpKind::Gather { exp: inner_exp, sep: _ }
        | ExpKind::PositiveGather { exp: inner_exp, sep: _ } => {
            collect_fields(fields, inner_exp, rules);
        }
        // Base cases or expressions that don't introduce named fields directly
        ExpKind::Call { .. }
        | ExpKind::Token(_)
        | ExpKind::Pattern(_)
        | ExpKind::Constant(_)
        | ExpKind::Cut
        | ExpKind::Void
        | ExpKind::Fail
        | ExpKind::Dot
        | ExpKind::Eof
        | ExpKind::Eol
        | ExpKind::EmptyClosure
        | ExpKind::Alert(_, _)
        | ExpKind::SkipTo(_)
        | ExpKind::Nil
        | ExpKind::RuleInclude { .. } => {
            // No named fields in these directly
        }
    }
}

/// Tries to determine the Rust type name for an expression, typically from a rule call's parameters.
///
/// This is analogous to `ogopego`'s `resolvedTypeName`. It looks for `ExpKind::Call`
/// and retrieves the type annotation from the called rule's parameters.
fn resolve_type_name(exp: &Exp, rules: &RuleMap) -> Option<Str> {
    match &exp.kind {
        ExpKind::Call { name, .. } => {
            if let Some(rule_ref) = rules.get(name) {
                if !rule_ref.params.is_empty() {
                    return Some(rule_ref.params[0].clone());
                }
            }
            None
        }
        ExpKind::Group(inner)
        | ExpKind::Optional(inner)
        | ExpKind::Closure(inner)
        | ExpKind::PositiveClosure(inner)
        | ExpKind::Gather { exp: inner, .. }
        | ExpKind::PositiveGather { exp: inner, .. } => {
            resolve_type_name(inner, rules)
        }
        ExpKind::Sequence(elements) | ExpKind::Choice(elements) => {
            // In a sequence or choice, if any element resolves to a type, return it.
            // This might need refinement based on TatSu's exact type inference rules for these.
            for element in elements.iter() {
                if let Some(tn) = resolve_type_name(element, rules) {
                    return Some(tn);
                }
            }
            None
        }
        _ => None,
    }
}

/// Checks if an expression is guaranteed to produce `Tree::Text` at runtime.
///
/// This is analogous to `ogopego`'s `isConstantText`.
fn is_constant_text(exp: &Exp, rules: &RuleMap) -> bool {
    match &exp.kind {
        ExpKind::Pattern(_) | ExpKind::Token(_) | ExpKind::Constant(_) => true,
        ExpKind::Group(inner) => is_constant_text(inner, rules),
        ExpKind::Call { name, .. } => {
            // If it's a call to a rule that is itself a constant text, then yes.
            // This requires looking up the rule and checking its expression.
            if let Some(rule_ref) = rules.get(name) {
                is_constant_text(&rule_ref.exp, rules)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Checks if an expression implies a list-like output in the folded tree.
///
/// This is analogous to `ogopego`'s `isListExpr`.
fn is_list_exp(exp: &Exp) -> bool {
    match &exp.kind {
        ExpKind::Closure(_)
        | ExpKind::PositiveClosure(_)
        | ExpKind::Gather { .. }
        | ExpKind::PositiveGather { .. } => true,
        ExpKind::Group(inner) => is_list_exp(inner),
        ExpKind::Sequence(elements) => {
            // If any element in a sequence is a list expression, the sequence *might* become a list.
            // This heuristic can be tricky and might need refinement based on actual TatSu behavior.
            elements.iter().any(|item| is_list_exp(item))
        }
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::peg::Rule;
    use crate::peg::grammar::Grammar;
    use crate::peg::exp::ExpKind;

    #[test]
    fn test_pascal_case() {
        assert_eq!(pascal_case("hello_world"), "HelloWorld".into());
        assert_eq!(pascal_case("single"), "Single".into());
        assert_eq!(pascal_case(""), "".into());
    }

    #[test]
    fn test_collect_fields_simple_named() {
        // rule :: MyType: a:'A'
        let mut fields = Vec::new();
        let rules = IndexMap::new();

        let exp = ExpKind::Named(
            "a".into(),
            Box::new(Exp {
                kind: ExpKind::Pattern("A".into()),
                la: None, df: None
            }),
        ).into();

        collect_fields(&mut fields, &exp, &rules);

        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "a");
        assert_eq!(fields[0].rust_name, "A");
        assert_eq!(fields[0].rust_type, "Rc<String>");
        assert_eq!(fields[0].from_named_list, false);
    }

    #[test]
    fn test_collect_fields_named_list_of_text() {
        // rule :: MyType: a+:'A'
        let mut fields = Vec::new();
        let rules = IndexMap::new();

        let exp = ExpKind::NamedList(
            "a".into(),
            Box::new(Exp {
                kind: ExpKind::Pattern("A".into()),
                la: None, df: None
            }),
        ).into();

        collect_fields(&mut fields, &exp, &rules);

        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "a");
        assert_eq!(fields[0].rust_name, "A");
        assert_eq!(fields[0].rust_type, "Vec<Rc<String>>");
        assert_eq!(fields[0].from_named_list, true);
    }

    #[test]
    fn test_collect_fields_named_call_to_typed_rule() {
        // rule_x :: X: 'x'
        // rule_y :: Y: x=rule_x
        let mut rules = IndexMap::new();
        let rule_x_exp = ExpKind::Pattern("x_val".into()).into();
        let rule_x = Rule::new("rule_x", &[ "X".into() ], rule_x_exp);
        rules.insert("rule_x".into(), Rc::new(rule_x));

        let mut fields = Vec::new();
        let exp = ExpKind::Named(
            "x".into(),
            Box::new(Exp {
                kind: ExpKind::Call { name: "rule_x".into(), rule: None },
                la: None, df: None
            }),
        ).into();

        collect_fields(&mut fields, &exp, &rules);

        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "x");
        assert_eq!(fields[0].rust_name, "X");
        assert_eq!(fields[0].rust_type, "Rc<X>");
        assert_eq!(fields[0].from_named_list, false);
    }

    #[test]
    fn test_collect_fields_named_list_call_to_typed_rule() {
        // rule_x :: X: 'x'
        // rule_y :: Y: x+='rule_x'
        let mut rules = IndexMap::new();
        let rule_x_exp = ExpKind::Pattern("x_val".into()).into();
        let rule_x = Rule::new("rule_x", &[ "X".into() ], rule_x_exp);
        rules.insert("rule_x".into(), Rc::new(rule_x));

        let mut fields = Vec::new();
        let exp = ExpKind::NamedList(
            "x".into(),
            Box::new(Exp {
                kind: ExpKind::Call { name: "rule_x".into(), rule: None },
                la: None, df: None
            }),
        ).into();

        collect_fields(&mut fields, &exp, &rules);

        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "x");
        assert_eq!(fields[0].rust_name, "X");
        assert_eq!(fields[0].rust_type, "Vec<Rc<X>>");
        assert_eq!(fields[0].from_named_list, true);
    }

    #[test]
    fn test_resolve_type_name_simple_call() {
        let mut rules = IndexMap::new();
        let rule_a_exp = ExpKind::Pattern("a".into()).into();
        let rule_a = Rule::new("rule_a", &[ "AType".into() ], rule_a_exp);
        rules.insert("rule_a".into(), Rc::new(rule_a));

        let exp = ExpKind::Call { name: "rule_a".into(), rule: None }.into();
        assert_eq!(resolve_type_name(&exp, &rules), Some("AType".into()));
    }

    #[test]
    fn test_resolve_type_name_nested_group() {
        let mut rules = IndexMap::new();
        let rule_a_exp = ExpKind::Pattern("a".into()).into();
        let rule_a = Rule::new("rule_a", &[ "AType".into() ], rule_a_exp);
        rules.insert("rule_a".into(), Rc::new(rule_a));

        let inner_exp = ExpKind::Call { name: "rule_a".into(), rule: None }.into();
        let exp = ExpKind::Group(Box::new(inner_exp)).into();
        assert_eq!(resolve_type_name(&exp, &rules), Some("AType".into()));
    }

    #[test]
    fn test_is_constant_text_pattern() {
        let rules = IndexMap::new();
        let exp = ExpKind::Pattern("abc".into()).into();
        assert!(is_constant_text(&exp, &rules));
    }

    #[test]
    fn test_is_constant_text_call_to_token_rule() {
        let mut rules = IndexMap::new();
        let rule_token_exp = ExpKind::Token("KEY".into()).into();
        let rule_token = Rule::new("KEY_RULE", &[], rule_token_exp);
        rules.insert("KEY_RULE".into(), Rc::new(rule_token));

        let exp = ExpKind::Call { name: "KEY_RULE".into(), rule: None }.into();
        assert!(is_constant_text(&exp, &rules));
    }

    #[test]
    fn test_is_list_exp_closure() {
        let exp = ExpKind::Closure(Box::new(ExpKind::Pattern("x".into()).into())).into();
        assert!(is_list_exp(&exp));
    }

    #[test]
    fn test_is_list_exp_sequence_with_closure() {
        let exp = ExpKind::Sequence(
            vec![
                ExpKind::Pattern("a".into()).into(),
                ExpKind::Closure(Box::new(ExpKind::Pattern("x".into()).into())).into(),
            ]
            .into(),
        ).into();
        assert!(is_list_exp(&exp));
    }
}
