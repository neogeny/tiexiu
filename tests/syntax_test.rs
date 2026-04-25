// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/syntax_test.py

use serde_json::json;
use tiexiu::api::{compile, parse_grammar};
use tiexiu::engine;
use tiexiu::input::StrCursor;
use tiexiu::peg::{ExpKind, Grammar};
use tiexiu::trees::short::*;
use tiexiu::{Cfg, Result};

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
        @@grammar:: grammar

        start = 'test' $ ;
    "#;

    let tree = parse_grammar(grammar, &[])?;
    eprintln!("{:?}", tree);
    let parser = compile(grammar, &[])?;

    assert_eq!(parser.name.to_string(), "grammar");
    assert!(!parser.analyzed);
    assert_eq!(parser.get_directives().len(), 1);
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
    // @: override operator makes the rule result be the expression result
    // @+: like @: but forces result to be a list
    let grammar = r#"
        n = @: {"a"}* ;
    "#;

    let parser = compile(grammar, &[])?;
    let ast = parse_input(&parser, "a")?;
    // {"a"}* produces ["a"], @: makes rule return it directly
    assert_eq!(ast.to_value(), json!(["a"]));

    // f uses @+:
    let grammar = r#"
        f = @+: {"a"}* ;
    "#;

    let parser = compile(grammar, &[])?;
    let ast = parse_input(&parser, "a")?;
    // {"a"}* produces ["a"], @+ wraps in another list
    assert_eq!(ast.to_value(), json!([["a"]]));

    Ok(())
}

#[test]
fn test_optional_closure() -> Result<()> {
    let grammar = r#"
        start = foo+:"x" foo:{"y"}* {foo:"z"}* ;
    "#;

    let model = compile(grammar, &[])?;
    let ast = parse_input(&model, "x y y z z")?;
    assert_eq!(
        ast,
        m(&[("foo", s(&[t("x"), c(&[t("y"), t("y")]), t("z"), t("z")]))])
    );
    assert_eq!(ast.to_value(), json!({"foo":["x", ["y","y"], "z", "z"]}));
    Ok(())
}

#[test]
fn test_optional_sequence() -> Result<()> {
    let grammar = r#"
        start = '1' ['2' '3'] '4' $ ;
    "#;

    let model = compile(grammar, &[Cfg::Wsp("".to_string())])?;
    let mut ast;

    ast = parse_input(&model, "1 2 3 4")?;
    assert_eq!(ast, s(&[t("1"), s(&[t("2"), t("3")]), t("4")]));

    ast = parse_input(&model, "1     4")?;
    assert_eq!(ast, s(&[t("1"), t("4")]));

    Ok(())
}

#[test]
fn test_group_ast() -> Result<()> {
    let grammar = r#"
        start = '1' ('2' '3') '4' $ ;
    "#;

    let model = compile(grammar, &[])?;
    let ast = parse_input(&model, "1 2 3 4")?;
    assert_eq!(ast, s(&[t("1"), s(&[t("2"), t("3")]), t("4")]));
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
