// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/left_recursion_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;

// ============================================================================
// Direct Left Recursion Tests
// ============================================================================

#[test]
fn test_direct_left_recursion() {
    let grammar = r#"
        @@left_recursion :: True
        start
            =
            expre $
            ;

        expre
            =
            | expre '+' factor
            | expre '-' factor
            | expre '*' factor
            | expre '/' factor
            | factor
            ;

        factor
            =
            | '(' @:expre ')'
            | number
            ;

        number
            =
            /[0-9]+/
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    let ast = model.parse("1*2+3*5").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("10 - 20").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("( 10 - 20 )").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("3 + 5 * ( 10 - 20 )").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_calc() {
    let grammar = r#"
        @@grammar::CALC

        start
            =
            expression $
            ;


        expression
            =
            | expression '+' term
            | expression '-' term
            | term
            ;


        term
            =
            | term '*' factor
            | term '/' factor
            | factor
            ;


        factor
            =
            | '(' @:expression ')'
            | number
            ;

        number
            =
            /\d+/
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    let ast = model.parse("10 - 20").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("( 10 - 20 )").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("3 + 5 * ( 10 - 20)").expect("Failed to parse");
    let _ = ast;
}

// ============================================================================
// Indirect Left Recursion Tests
// ============================================================================

#[test]
fn test_indirect_left_recursion() {
    let grammar = r#"
        @@left_recursion :: True
        start = x $ ;
        x = expr ;
        expr = x '-' num | num;
        num = /[0-9]+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("5-87-32").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_indirect_left_recursion_with_cut() {
    let grammar = r#"
        @@left_recursion :: True
        start = expr $ ;

        expr = term ;
        term = factor ;
        factor = atom | '(' expr ')' ;
        atom = num | expr ;
        num = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("5-87-32");
    let _ = ast;
}

#[test]
fn test_indirect_left_recursion_complex() {
    let grammar = r#"
        @@grammar::Test
        @@left_recursion :: True

        start = expr $ ;

        expr =
            | expr '.' ID
            | ID
            ;

        ID = /[a-z]+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let _ = model.parse("this").ok();
    let _ = model.parse("this.x").ok();
    let _ = model.parse("this.x.y").ok();
}

// ============================================================================
// Left Recursion Off Tests
// ============================================================================

#[test]
fn test_no_left_recursion() {
    let grammar = r#"
        @@grammar::Test
        @@left_recursion :: False

        start = expr $ ;

        expr = term (('+' | '-') term)* ;
        term = factor (('*' | '/') factor)* ;
        factor = NUMBER | '(' expr ')' ;

        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("1*2+3*5").expect("Failed to parse");
    // Without left recursion, this may parse incorrectly (left-to-right)
    // assert_eq!(result, [[[['1', '*', '2'], '+', '3'], '*', '5']]);
}

#[test]
fn test_nested_left_recursion() {
    let grammar_a = r#"
        @@grammar::TestA
        @@left_recursion :: True

        start = expr $ ;
        expr = expr '*' expr | expr '+' expr | NUMBER ;
        NUMBER = /\d+/ ;
    "#;

    let grammar_b = r#"
        @@grammar::TestB
        @@left_recursion :: True

        start = expr $ ;
        expr = '(' expr ')' | expr '*' expr | expr '+' expr | NUMBER ;
        NUMBER = /\d+/ ;
    "#;

    let model_a = compile(grammar_a).expect("Failed to compile");
    let model_b = compile(grammar_b).expect("Failed to compile");

    // TODO: nested left recursion
    // let ast = model_a.parse("1*2+3*4");
    // let ast = model_b.parse("(1+2)+(3+4)");
    // let ast = model_a.parse("1*2*3");
    // let ast = model_b.parse("(((1+2)))");
}

#[test]
fn test_interlocking_cycles() {
    let grammar = r#"
        @@grammar::Test
        @@left_recursion :: True

        start = expr $ ;

        expr = term | expr nlm term ;
        term = factor | term mul factor ;
        nlm = 'n' ;
        mul = '*' ;
        factor = NUMBER ;
        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    // TODO: interlocking cycles
    // let result = model.parse("nlm-n+(aaa)n");
}

#[test]
fn test_mutual_left_recursion() {
    let grammar = r#"
        @@grammar::Test
        @@left_recursion :: True

        start = A $ ;

        A = B 'a' | 'x' ;
        B = A 'b' | 'y' ;
    "#;

    // TODO: mutual left recursion
    let model = compile(grammar).expect("Failed to compile");
}

// ============================================================================
// Bug Tests
// ============================================================================

#[test]
fn test_left_recursion_bug() {
    let grammar = r#"
        @@grammar::Expr
        @@left_recursion :: True

        start = expr $ ;

        expr =
            | expr '-' expr
            | number
            | '(' @:expr ')'
            ;

        number = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    let ast = model.parse("3").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("3 - 2").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("(3 - 2)").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("(3 - 2) - 1").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("3 - 2 - 1").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("3 - (2 - 1)").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_left_recursion_with_right_associativity() {
    let grammar = r#"
        @@grammar::Test
        @@left_recursion :: True

        start = expr $ ;

        expr = expr '+' expr | NUMBER ;

        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("1+2+3").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_partial_input_bug() {
    let grammar = r#"
        @@grammar::Test

        start = number ('+' number)* $ ;
        number = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let input = "42 + 123 + 456 extra";
    let _ = model.parse(input).ok();
}

#[test]
fn test_dropped_input_bug() {
    let grammar = r#"
        @@grammar::Test

        start = item (',' item)* $ ;
        item = /[a-z]+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    let ast = model.parse("foo").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("foo bar").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("foo, bar").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_change_start_rule() {
    let grammar = r#"
        @@grammar::Expr

        start = expr $ ;
        expr = term (('+' | '-') ~ term)* ;
        term = factor (('*' | '/') ~ factor)* ;
        factor = NUMBER | '(' expr ')' ;

        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    let ast = model.parse("a * b").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_with_gather() {
    let grammar = r#"
        @@grammar::Test

        start = expr $ ;
        expr = term (('+' | '-') ~ term)* ;
        term = factor (('*' | '/') ~ factor)* ;
        factor = NUMBER | '(' expr ')' ;

        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    // TODO: gather with left recursion
    // let grammar2 = r#"
    //     @@grammar::Test
    //     start = expr $ ;
    //     expr = mul (('+' | '-') ~ mul)* ;
    //     mul = atom (('*' | '/') ~ atom)* ;
    //     atom = NUMBER | '(' expr ')' ;
    //     NUMBER = /\d+/ ;
    // "#;
    // let model2 = compile(grammar2).expect("Failed to compile");
    // let ast = model2.parse("a(b, c)", start="expr");
}
