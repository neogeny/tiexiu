// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's parsing_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse_input};
use crate::trees::Tree;

#[test]
fn test_include() {
    let grammar = r#"
        @@include :: "included.ebnf"
        start = item $ ;
        item = /\w+/ ;
    "#;

    let model = compile(grammar);
    assert!(model.is_err());
}

#[test]
fn test_multiple_include() {
    let grammar = r#"
        @@include :: "a.ebnf"
        @@include :: "b.ebnf"
        start = item $ ;
        item = /\w+/ ;
    "#;

    let model = compile(grammar);
    assert!(model.is_err());
}

#[test]
fn test_escape_sequences() {
    let grammar = r#"
        start = 'hello\nworld' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("hello\nworld").expect("Failed to parse");
    assert_eq!(ast.to_string(), "hello\\nworld");
}

// ============================================================================
// Start Rule Tests
// ============================================================================

#[test]
fn test_start() {
    let grammar = r#"
        @@grammar :: Test

        true = 'test' @:`True` $;
        false = 'test' @:`False` $;
    "#;

    let model = compile(grammar).expect("Failed to compile grammar");
    assert_eq!(model.name, "Test");

    let _ast = parse_input(&model, "test").expect("Failed to parse");
}

#[test]
fn test_rule_capitalization() {
    let grammar = r#"
        start = ['test' {rulename}] ;
        {rulename} = /[a-zA-Z0-9]+/ ;
    "#;

    let test_string = "test 12";
    let lowercase_rule_names = vec!["nocaps", "camelCase", "tEST"];
    let uppercase_rule_names = vec!["Capitalized", "CamelCase", "TEST"];

    let ref_lowercase_grammar = grammar.replace("rulename", "reflowercase");
    let ref_lowercase = compile(&ref_lowercase_grammar)
        .expect("Failed to compile")
        .parse(test_string)
        .expect("Failed to parse");

    let ref_uppercase_grammar = grammar.replace("rulename", "Refuppercase");
    let ref_uppercase = compile(&ref_uppercase_grammar)
        .expect("Failed to compile")
        .parse(test_string)
        .expect("Failed to parse");

    for rulename in lowercase_rule_names {
        let g = grammar.replace("rulename", rulename);
        let result = compile(&g)
            .expect("Failed to compile")
            .parse(test_string)
            .expect("Failed to parse");
        assert_eq!(result, ref_lowercase);
    }

    for rulename in uppercase_rule_names {
        let g = grammar.replace("rulename", rulename);
        let result = compile(&g)
            .expect("Failed to compile")
            .parse(test_string)
            .expect("Failed to parse");
        assert_eq!(result, ref_uppercase);
    }
}

// ============================================================================
// Start Rule Issue Tests
// ============================================================================

#[test]
fn test_startrule_issue62() {
    let grammar = r#"
        @@grammar::TEST

        file_input = expr $ ;
        expr = number '+' number ;
        number = /[0-9]/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    parse_input(&model, "4 + 5").expect("Failed to parse");
}

// ============================================================================
// Whitespace Tests
// ============================================================================

#[test]
fn test_skip_whitespace() {
    let grammar = r#"
        statement = 'FOO' subject $ ;
        subject = name:id ;
        id = /[a-z]+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = parse_input(&model, "FOO   bar").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_cut_scope() {
    let grammar = r#"
        start =
            | one
            | two
            ;

        one =
            | ~ !()
            | 'abc';

        two = `something` ;
    "#;

    let ast = compile(grammar)
        .and_then(|g| parse_input(&g, "abc"))
        .expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_node_parseinfo() {
    let grammar = r#"
        @@grammar::Test
        start = 'test' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("test").expect("Failed to parse");
    assert!(!ast.to_string().is_empty());
}

#[test]
fn test_parseinfo_directive() {
    let grammar = r#"
        @@parseinfo :: True
        start = 'test' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("test").expect("Failed to parse");
    assert!(!ast.to_string().is_empty());
}

#[test]
fn test_parseinfo_false_directive() {
    let grammar = r#"
        @@parseinfo :: False
        start = 'test' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("test").expect("Failed to parse");
    let _ = ast;
}
