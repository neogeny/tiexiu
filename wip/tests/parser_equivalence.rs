// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's parser_equivalence_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;

// ============================================================================
// Parser Equivalence Tests
// ============================================================================

const INPUT: &str = "1d3";
const OUTPUT: &[(&str, &str)] = &[("number_of_dice", "1"), ("sides", "3")];

const GRAMMAR: &str = r#"
    start = expression $;

    int = /-?\d+/ ;

    dice = number_of_dice:factor /d|D/ sides:factor;

    expression = addition ;

    addition
        =
        | left:dice_expr op:('+' | '-') ~ right:addition
        | dice_expr
        ;

    dice_expr
        =
        | dice
        | factor
        ;

    factor
        =
        | '(' ~ @:expression ')'
        | int
        ;
"#;

#[test]
fn test_model_parse() {
    let grammar = r#"
        @@grammar :: Dice
        start = {die}+ $ ;
        die = number_of_dice:'[0-9]+' 'd' sides:'[0-9]+' ;
    "#;

    let input = "2d6";
    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse(input).expect("Failed to parse");
    let _ = result;
}

#[test]
fn test_error_messages() {
    let grammar = r#"
        @@grammar :: ORDER
        alphabet = a b others $ ;

        a = 'a' ;
        b = 'b' ;
        others = 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o';
    "#;

    let input = "a b";

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse(input);
    assert!(result.is_err());
}

#[test]
fn test_name_checked() {
    let grammar = r#"
        @@grammar :: Test
        @@ignorecase :: True
        @@keyword :: if

        start = rule ;
        rule = @:word if_exp $ ;
        if_exp = 'if' digit ;
        @name word = /\w+/ ;
        digit = /\d/ ;
    "#;

    let model = compile(grammar, "Test").expect("Failed to compile");
    model.parse("nonIF if 1").expect("Failed to parse");

    let result = model.parse("if if 1");
    assert!(result.is_err());
}

#[test]
fn test_first_rule() {
    let grammar = r#"
        @@grammar :: Test

        true = 'test' @: `True` $ ;
        start = 'test' @: `False` $ ;
    "#;

    let _model = compile(grammar, "Test").expect("Failed to compile");
}

#[test]
fn test_dynamic_compiled_ast() {
    let grammar = r#"
        test::Test = 'TEST' ['A' a:number] ['B' b:number] ;
        number::int = /\d+/ ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_none_whitespace() {
    let grammar = r#"
        @@whitespace:: None

        start = "This is a" test;
        test = " test";
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_sep_join() {
    let grammar = r#"
        @@grammar::numbers

        start
            = expression $
            ;

        expression
            = ~ ( "," )%{ digit }+
            ;

        digit = /\d+/ ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}
