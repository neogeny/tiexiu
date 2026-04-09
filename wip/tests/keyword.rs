// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/keyword_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse};
use crate::error::FailedKeyword;

#[test]
fn test_keywords_in_rule_names() {
    let grammar = r#"
        start
            =
            whitespace
            ;

        whitespace
            =
            {'x'}+
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    model.parse("x").expect("Failed to parse");
}

#[test]
fn test_python_keywords_in_rule_names() {
    let grammar = r#"
        not = 'x' ;
    "#;

    let model = compile(grammar, "Keywords");

    struct Semantics {
        called: bool,
    }
    impl crate::semantics::Semantics for Semantics {
        fn not_(&mut self, _ast: &Tree) {
            self.called = true;
        }
    }

    let mut semantics = Semantics { called: false };
    model.parse("x", &mut semantics);
    assert!(semantics.called);
}

#[test]
fn test_define_keywords() {
    let grammar = r#"
        @@keyword :: B C
        @@keyword :: 'A'

        start = ('a' 'b').{'x'}+ ;
    "#;

    let model = compile(grammar, "test");
    let c = crate::codegen::pythongen(&model);
    parse(&c.to_string(), "");
}

#[test]
fn test_check_keywords() {
    let grammar = r#"
        @@keyword :: A

        start = {id}+ $ ;

        @name
        id = /\w+/ ;
    "#;

    let model = compile(grammar, "test");
    let ast = model.parse("hello world");
    assert_eq!(ast, vec!["hello", "world"]);

    let result = model.parse("hello A world");
    match result {
        Err(e) if e.to_string().contains("keyword") => {}
        _ => panic!("Expected FailedKeyword"),
    }
}

#[test]
fn test_check_unicode_name() {
    let grammar = r#"
        @@keyword :: A

        start = {id}+ $ ;

        @name
        id = /\w+/ ;
    "#;

    let model = compile(grammar, "test");
    let ast = model.parse("hello Øresund");
    assert_eq!(ast, vec!["hello", "Øresund"]);
}

#[test]
fn test_sparse_keywords() {
    let grammar = r#"
        @@keyword :: A

        @@ignorecase :: False

        start = {id}+ $ ;

        @@keyword :: B

        @name
        id = /\w+/ ;
    "#;

    let model = compile(grammar, "test");
    let ast = model.parse("hello world");
    assert_eq!(ast, vec!["hello", "world"]);

    for k in &["A", "B"] {
        let result = model.parse(format!("hello {} world", k));
        match result {
            Err(e) if e.to_string().contains("keyword") => {}
            _ => panic!("Expected FailedKeyword for {}", k),
        }
    }
}

#[test]
fn test_ignorecase_keywords() {
    let grammar = r#"
        @@ignorecase :: True
        @@keyword :: if

        start = rule ;

        @name
        rule = @:word if_exp $ ;

        if_exp = 'if' digit ;

        word = /\w+/ ;
        digit = /\d/ ;
    "#;

    let model = compile(grammar, "test");
    model.parse("nonIF if 1");

    let result = model.parse("if if 1");
    match result {
        Err(e) if e.to_string().contains("keyword") => {}
        _ => panic!("Expected FailedKeyword"),
    }
}

#[test]
fn test_keywords_are_str() {
    let grammar = r#"
        @@keyword :: True False

        start = $ ;
    "#;

    let model = compile(grammar, "test");
    let keywords: Vec<String> = model.keywords.iter().map(|k| k.to_string()).collect();
    assert!(keywords.iter().all(|k| k == "True" || k == "False"));
}
