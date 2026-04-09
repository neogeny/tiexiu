// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/parameter_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;

// ============================================================================
// Keyword Params Tests
// ============================================================================

#[test]
fn test_keyword_params() {
    let grammar = r#"
        start(k1=1, k2=2)
            =
            {'a'} $
            ;
    "#;

    // TODO: Rule keyword parameters not implemented
    // g = TatSuParserGenerator('Keywords')
    // model = g.parse(grammar)
}

#[test]
fn test_35_only_keyword_params() {
    let grammar = r#"
        rule[kwdA=A, kwdB=B]: 'a'
    "#;

    // TODO: Rule parameters not implemented
    // model = compile(grammar, 'test')
}

#[test]
fn test_36_params_and_keyword_params() {
    let grammar = r#"
        rule[A, kwdB=B]: 'a'
    "#;

    // TODO: Rule parameters not implemented
    // model = compile(grammar, 'test')
}

#[test]
fn test_36_param_combinations() {
    // TODO: Rule parameters with semantics not implemented
    // grammar = """
    //     start = {rule_positional | rule_keywords | rule_all} $ ;
    //     rule_positional('ABC', 123, '=', '+') = 'a' ;
    //     rule_keywords(k1=ABC, k3='=', k4='+', k2=123) = 'b' ;
    //     rule_all('DEF', 456, '=', '+', k1=HIJ, k3='=', k4='+', k2=789) = 'c' ;
    // """
    //
    // model = compile(grammar, 'RuleArguments')
    // ast = model.parse('a b c')
}

#[test]
fn test_36_unichars() {
    // TODO: Unicode in rule parameters not implemented
    // grammar = """
    //     start = { rule_positional | rule_keywords | rule_all }* $ ;
    //     rule_positional("ÄÖÜäöüß") = 'a' ;
    //     rule_keywords(k1='äöüÄÖÜß') = 'b' ;
    //     rule_all('ßÄÖÜäöü', k1="ßäöüÄÖÜ") = 'c' ;
    // """
    //
    // m = compile(grammar, 'UnicodeRuleArguments')
}

#[test]
fn test_numbers_and_unicode() {
    let grammar = r#"
        rúle[1, -23, 4.56, 7.89e-11, Añez]: 'a'

        rúlé[Añez]: 'ñ'
    "#;

    // TODO: Numeric parameters not implemented
    // model = compile(grammar, 'test')
}
