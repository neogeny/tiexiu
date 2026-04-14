// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/syntax_test.py

use tiexiu::api::compile;
use tiexiu::input::StrCursor;
use tiexiu::peg::Grammar;
use tiexiu::state::corectx::CoreCtx;

fn parse_input(grammar: &Grammar, input: &str) -> tiexiu::trees::Tree {
    let cursor = StrCursor::new(input);
    let ctx = CoreCtx::new(cursor);
    match grammar.parse(ctx) {
        Ok(s) => s.1,
        Err(f) => panic!("Failed to parse at mark {}: {:?}", f.mark, f.source),
    }
}

// ============================================================================
// AST Update Tests
// ============================================================================

#[test]
fn test_update_ast() {
    let grammar = r#"
        start = 'test' $ ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
}

#[test]
#[ignore = "TODO: evaluate - @: and @+: override syntax"]
fn test_ast_assignment() {
    let grammar = r#"
        n  = @: {"a"}* $ ;
        f  = @+: {"a"}* $ ;
        nn = @: {"a"}*  @: {"b"}* $ ;
        nf = @: {"a"}*  @+: {"b"}* $ ;
        fn = @+: {"a"}* @: {"b"}* $ ;
        ff = @+: {"a"}* @+: {"b"}* $ ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
}

#[test]
#[ignore = "TODO: evaluate - closure with override"]
fn test_optional_closure() {
    let grammar = r#"
        start = foo+:"x" foo:{"y"}* {foo:"z"}* ;
    "#;

    let model = compile(grammar, &[]).expect("Failed to compile");
    let _ast = parse_input(&model, "xyyzz");
}

#[test]
#[ignore = "TODO: evaluate - optional sequence"]
fn test_optional_sequence() {
    let grammar = r#"
        start = '1' ['2' '3'] '4' $ ;
    "#;

    let model = compile(grammar, &[]).expect("Failed to compile");
    let _ast = parse_input(&model, "1234");
}

#[test]
#[ignore = "TODO: evaluate - group parsing"]
fn test_group_ast() {
    let grammar = r#"
        start = '1' ('2' '3') '4' $ ;
    "#;

    let model = compile(grammar, &[]).expect("Failed to compile");
    let _ast = parse_input(&model, "1234");
}

#[test]
#[ignore = "TODO: evaluate - partial options"]
fn test_partial_options() {
    let grammar = r#"
        start = [a] ['A' 'A' | 'A' 'B'] $ ;
        a = 'A' !('A'|'B') ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
}
