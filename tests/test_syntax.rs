// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/syntax_test.py

use tiexiu::Result;
use tiexiu::api::{compile, parse_grammar};
use tiexiu::engine;
use tiexiu::input::StrCursor;
use tiexiu::peg::{ExpKind, Grammar};

fn parse_input(grammar: &Grammar, input: &str) -> Result<tiexiu::trees::Tree> {
    let cursor = StrCursor::new(input);
    let ctx = engine::new_ctx(cursor, &[]);
    match grammar.parse(ctx) {
        Ok(s) => Ok(s.1),
        Err(f) => Err(f.into()),
    }
}

// ============================================================================
// AST Update Tests
// ============================================================================

#[test]
fn test_update_ast() -> Result<()> {
    let grammar = r#"
        start = 'test' $ ;
    "#;

    let tree = parse_grammar(grammar, &[])?;
    eprintln!("{:?}", tree);
    let parser = compile(grammar, &[])?;

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
    Ok(())
}

#[test]
fn test_ast_assignment() -> Result<()> {
    let grammar = r#"
        n  = @: {"a"}* $ ;
        f  = @+: {"a"}* $ ;
        nn = @: {"a"}*  @: {"b"}* $ ;
        nf = @: {"a"}*  @+: {"b"}* $ ;
        fn = @+: {"a"}* @: {"b"}* $ ;
        ff = @+: {"a"}* @+: {"b"}* $ ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
fn test_optional_closure() -> Result<()> {
    let grammar = r#"
        start = foo+:"x" foo:{"y"}* {foo:"z"}* ;
    "#;

    let model = compile(grammar, &[])?;
    let _ast = parse_input(&model, "xyyzz")?;
    Ok(())
}

#[test]
fn test_optional_sequence() -> Result<()> {
    let grammar = r#"
        start = '1' ['2' '3'] '4' $ ;
    "#;

    let model = compile(grammar, &[])?;
    let _ast = parse_input(&model, "1234")?;
    Ok(())
}

#[test]
fn test_group_ast() -> Result<()> {
    let grammar = r#"
        start = '1' ('2' '3') '4' $ ;
    "#;

    let model = compile(grammar, &[])?;
    let _ast = parse_input(&model, "1234")?;
    Ok(())
}

#[test]
fn test_partial_options() -> Result<()> {
    let grammar = r#"
        start = [a] ['A' 'A' | 'A' 'B'] $ ;
        a = 'A' !('A'|'B') ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}
