// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/syntax_test.py

use tiexiu::api::{compile, parse_grammar};
use tiexiu::input::StrCursor;
use tiexiu::peg::{ExpKind, Grammar};
use tiexiu::state::corectx::CoreCtx;

fn parse_input(grammar: &Grammar, input: &str) -> tiexiu::trees::Tree {
    let cursor = StrCursor::new(input);
    let ctx = CoreCtx::new(cursor, &[]);
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

    let tree = parse_grammar(grammar, &[]);
    eprintln!("{:?}", tree);
    let parser = compile(grammar, &[]).expect("Failed to compile");

    assert_eq!(parser.name.to_string(), "grammar");
    assert!(!parser.analyzed);
    assert!(parser.get_directives().is_empty());
    assert!(parser.keywords.is_empty());

    for rule in parser.rules() {
        assert_eq!(&*rule.name, "start");
        match &rule.exp.kind {
            ExpKind::Sequence(exps) => {
                assert_eq!(exps.len(), 2);
                for (i, e) in exps.iter().enumerate() {
                    match (i, &e.kind) {
                        (0, ExpKind::Token(tok)) => {
                            assert_eq!(tok.as_ref(), "test");
                            let la: Vec<&str> = e.la.iter().map(|s| s.as_ref()).collect();
                            assert_eq!(la.as_slice(), &["test"]);
                        }
                        (1, ExpKind::Eof) => {
                            assert!(e.la.is_empty());
                        }
                        _ => panic!("Unexpected exp at {}: {:?}", i, e.kind),
                    }
                }
            }
            other => panic!("Expected Sequence, got {:?}", other),
        }
    }
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

    let _model = compile(grammar, &[]).expect("Failed to compile");
}

#[test]
fn test_optional_closure() {
    let grammar = r#"
        start = foo+:"x" foo:{"y"}* {foo:"z"}* ;
    "#;

    let model = compile(grammar, &[]).expect("Failed to compile");
    let _ast = parse_input(&model, "xyyzz");
}

#[test]
fn test_optional_sequence() {
    let grammar = r#"
        start = '1' ['2' '3'] '4' $ ;
    "#;

    let model = compile(grammar, &[]).expect("Failed to compile");
    let _ast = parse_input(&model, "1234");
}

#[test]
fn test_group_ast() {
    let grammar = r#"
        start = '1' ('2' '3') '4' $ ;
    "#;

    let model = compile(grammar, &[]).expect("Failed to compile");
    let _ast = parse_input(&model, "1234");
}

#[test]
fn test_partial_options() {
    let grammar = r#"
        start = [a] ['A' 'A' | 'A' 'B'] $ ;
        a = 'A' !('A'|'B') ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
}
