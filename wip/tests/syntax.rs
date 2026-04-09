// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/syntax_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse};

// ============================================================================
// AST Update Tests
// ============================================================================

#[test]
fn test_update_ast() {
    let grammar = r#"
        foo = name:"1" [ name: bar ] ;
        bar = { "2" }* ;
    "#;

    let model = compile(grammar, "Keywords").expect("Failed to compile");
    let ast = model.parse("1 2").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_ast_assignment() {
    let grammar = r#"
        n  = @: {"a"}* $ ;
        f  = @+: {"a"}* $ ;
        nn = @: {"a"}*  @: {"b"}* $ ;
        nf = @: {"a"}*  @+: {"b"}* $ ;
        fn = @+: {"a"}* @: {"b"}* $ ;
        ff = @+: {"a"}* @+: {"b"}* $ ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let _ = model.parse("").ok();
}

#[test]
fn test_optional_closure() {
    let grammar = r#"
        start = foo+:"x" foo:{"y"}* {foo:"z"}* ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("xyyzz").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_optional_sequence() {
    let grammar = r#"
        start = '1' ['2' '3'] '4' $ ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("1234").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_group_ast() {
    let grammar = r#"
        start = '1' ('2' '3') '4' $ ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("1234").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_partial_options() {
    let grammar = r#"
        start = [a] ['A' 'A' | 'A' 'B'] $ ;
        a = 'A' !('A'|'B') ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("AB").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_include_and_override() {
    let grammar = r#"
        @@include :: "included.ebnf"
        @@override
        start = 'test' $ ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let _ = model;
}

// ============================================================================
// Token and Sequence Tests
// ============================================================================

#[test]
fn test_token_sequence() {
    let grammar = r#"
        start = 'a' 'b' 'c' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("abc").expect("Failed to parse");
}

#[test]
fn test_closure_tokens() {
    let grammar = r#"
        start = {'a'}+ $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("aaa").expect("Failed to parse");
}

#[test]
fn test_optional_token() {
    let grammar = r#"
        start = 'a' ['b'] $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result1 = model.parse("ab").expect("Failed to parse");
    let result2 = model.parse("a").expect("Failed to parse");
}

#[test]
fn test_positive_closure_tokens() {
    let grammar = r#"
        start = {'a'}+ $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("aaa").expect("Failed to parse");
}

#[test]
fn test_choice_alternatives() {
    let grammar = r#"
        start = 'a' | 'b' | 'c' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result1 = model.parse("a").expect("Failed to parse");
    let result2 = model.parse("b").expect("Failed to parse");
    let result3 = model.parse("c").expect("Failed to parse");
}
