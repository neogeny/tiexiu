// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/directive_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse};

// ============================================================================
// Whitespace Directive Tests
// ============================================================================

#[test]
fn test_whitespace_directive() {
    let grammar = r#"
        @@whitespace :: /[\t ]+/

        test = "test" $;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("test").expect("Failed to parse");
}

#[test]
fn test_whitespace_none_directive() {
    let grammars = vec![
        r#"
            @@whitespace :: None
            @@nameguard :: False

            test = {'x'}+ $;
        "#,
        r#"
            @@whitespace :: False
            @@nameguard :: False

            test = {'x'}+ $;
        "#,
    ];

    for grammar in grammars {
        let model = compile(grammar).expect("Failed to compile");
        let result = model.parse("xx").expect("Failed to parse");

        let result = model.parse("x x");
        assert!(result.is_err());
    }
}

#[test]
fn test_whitespace_escaping() {
    let grammar = r#"
        @@grammar::Markdown
        @@whitespace :: /[ ]/
        start = pieces $ ;
        text = text:/[a-z]+/ ;
        pieces = {text}* ;
    "#;

    let result = parse(grammar, "[]");
    assert!(result.is_err());
}

#[test]
fn test_default_whitespace() {
    let grammar = r#"
        start = {'x'}+ $;
    "#;

    // Default whitespace should skip spaces
    let result = parse(grammar, "x x x").expect("Failed to parse");
}

#[test]
fn test_eol_comments_re_directive() {
    let grammar = r#"
        @@eol_comments :: /#.*?$/

        test = "test" $;
    "#;
}

#[test]
fn test_left_recursion_directive() {
    let grammar = r#"
        @@left_recursion :: False

        test = "test" $;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let _ = model;
}

// ============================================================================
// Whitespace With Newlines Tests
// ============================================================================

#[test]
fn test_whitespace_no_newlines() {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        token = /[^ \n]+/;
        token2 = {token}* /\n/;
        document = {@+:token2}* $;
    "#;

    let text = "a b\nc d\ne f";

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse(text).expect("Failed to parse");
    let _ = ast;
}

// ============================================================================
// Grammar Directive Tests
// ============================================================================

#[test]
fn test_grammar_directive() {
    let grammar = r#"
        @@grammar :: Test

        start = test $;
        test = "test";
    "#;

    let model = compile(grammar).expect("Failed to compile");
    assert_eq!(model.name, "Test");
}

// ============================================================================
// Nameguard Directive Tests
// ============================================================================

#[test]
fn test_nameguard_directive() {
    let grammar = r#"
        @@grammar :: test
        @@nameguard :: False
        @@namechars :: ''

        start = sequence $ ;
        sequence = {digit}+ ;
        digit = 'x' | '1' | '2' | '3' | '4' | '5' ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let _ = model.parse("23").ok();
    let _ = model.parse("xx").ok();
}
