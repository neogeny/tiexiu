// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/error_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;
use crate::error::GrammarError;

#[test]
fn test_missing_rule() {
    let grammar = r#"
        @@grammar::TestGrammar
        block = test ;
    "#;

    let result = compile(grammar);
    match result {
        Err(e) if e.to_string().contains("Unknown rules") => {}
        _ => panic!("Expected GrammarError"),
    }
}

#[test]
fn test_missing_rules() {
    let grammar = r#"
        @@grammar::TestGrammar
        block = test | test2;
    "#;

    let result = compile(grammar);
    match result {
        Err(e) if e.to_string().contains("test") && e.to_string().contains("test2") => {}
        _ => panic!("Expected GrammarError"),
    }
}

#[test]
fn test_missing_rules_at_runtime() {
    let grammar = r#"
        @@grammar::TestGrammar
        start = test | test2 ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("abc");
    assert!(result.is_err());
}
