// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's parsing_test.py

use tiexiu::api::compile;
use tiexiu::input::StrCursor;
use tiexiu::peg::Grammar;
use tiexiu::state::corectx::CoreCtx;

fn _parse_input(grammar: &Grammar, input: &str) -> tiexiu::trees::Tree {
    let cursor = StrCursor::new(input);
    let ctx = CoreCtx::new(cursor, &[]);
    match grammar.parse(ctx) {
        Ok(s) => s.1,
        Err(f) => panic!("Failed to parse at mark {}: {:?}", f.mark, f.source),
    }
}

#[test]
fn test_include() {
    let grammar = r#"
        @@include :: "included.ebnf"
        start = item $ ;
        item = /\w+/ ;
    "#;

    let result = compile(grammar, &[]);
    assert!(
        result.is_err(),
        "Expected error for @@include, got success: {:?}",
        result
    );
}

#[test]
fn test_multiple_include() {
    let grammar = r#"
        @@include :: "a.ebnf"
        @@include :: "b.ebnf"
        start = item $ ;
        item = /\w+/ ;
    "#;

    let _result = compile(grammar, &[]);
    // assert!(result.is_err());
}

#[test]
fn test_escape_sequences() {
    let grammar = r#"
        start = 'hello\nworld' $ ;
    "#;

    let _model = compile(grammar, &[])
        .expect("Failed to compile (escape sequences in tokens not supported)");
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

    let _model = compile(grammar, &[]).expect("Failed to compile grammar");
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

    let _model = compile(grammar, &[]).expect("Failed to compile");
}

#[test]
fn test_node_parseinfo() {
    let grammar = r#"
        @@grammar::Test
        start = 'test' $ ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
}

#[test]
fn test_parseinfo_directive() {
    let grammar = r#"
        @@parseinfo :: True
        start = 'test' $ ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
}

#[test]
fn test_parseinfo_false_directive() {
    let grammar = r#"
        @@parseinfo :: False
        start = 'test' $ ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
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

    let _model = compile(grammar, &[]).expect("Failed to compile");
    // let _ast = parse_input(&model, "abc");
}
