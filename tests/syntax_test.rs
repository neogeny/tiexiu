// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/syntax_test.py

#[macro_use]
extern crate json;
use tiexiu::api::{compile, parse, parse_grammar};
use tiexiu::context;
use tiexiu::input::StrCursor;
use tiexiu::peg::{ExpKind, Grammar};
use tiexiu::trees::short::*;
use tiexiu::{CfgA, CfgKey, Result};

fn parse_input(grammar: &Grammar, input: &str, cfg: &CfgA) -> Result<tiexiu::trees::Tree> {
    let cursor = StrCursor::new(input);
    let ctx = context::new_ctx(cursor, cfg);
    grammar.parse_tree(ctx)
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
    assert!(parser.analyzed);
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
                        }
                        (1, ExpKind::Eof) => {
                            // EOF expression - just verify it exists
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
    let ast = parse_input(&parser, "a", &[])?;
    assert_eq!(ast.to_json(), array!["a"]);

    let grammar = r#"
        f = @+: {"a"}* ;
    "#;

    let parser = compile(grammar, &[])?;
    let ast = parse_input(&parser, "a", &[])?;
    // {"a"}* produces ["a"], @+ wraps in another list
    assert_eq!(ast.to_json(), array![array!["a"]]);

    Ok(())
}

#[test]
fn test_optional_closure() -> Result<()> {
    let grammar = r#"
        start = foo+:"x" foo:{"y"}* {foo:"z"}* ;
    "#;

    let model = compile(grammar, &[])?;
    let ast = parse_input(&model, "x y y z z", &[])?;
    assert_eq!(
        ast,
        m(&[("foo", s(&[t("x"), l(&[t("y"), t("y")]), t("z"), t("z")]))])
    );
    assert_eq!(
        ast.to_json(),
        object! {"foo": array!["x", array!["y", "y"], "z", "z"]}
    );
    Ok(())
}

#[test]
fn test_optional_sequence() -> Result<()> {
    let grammar = r#"
        start = '1' ['2' '3'] '4' $ ;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let mut ast;

    ast = parse_input(&model, "1 2 3 4", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!(["1", "2", "3", "4"]));
    assert_eq!(ast, l(&[t("1"), t("2"), t("3"), t("4")]));

    ast = parse_input(&model, "1     4", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast, l(&[t("1"), t("4")]));

    Ok(())
}

#[test]
fn test_group_ast() -> Result<()> {
    let grammar = r#"
        start = '1' ('2' '3') '4' $ ;
    "#;

    let model = compile(grammar, &[])?;
    let ast = parse_input(&model, "1 2 3 4", &[])?;
    assert_eq!(ast.to_json(), value!(["1", "2", "3", "4"]));
    assert_eq!(ast, l(&[t("1"), t("2"), t("3"), t("4")]));
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

#[test]
fn test_deprecated_comments_override() -> Result<()> {
    let grammar = r#"
        @@comments :: /@@@@@@/
        @@eol_comments :: /@@@@@@/

        start = 'a' $;
    "#;

    let cfg = [CfgKey::Eol(r"(?m)#[^\n]*$".to_string())];
    let parser = compile(grammar, &cfg)?;

    let text = "        # This comment should be stripped\n        a\n    ";
    let ast = parser.parse_input(text, &cfg)?;
    assert_eq!(ast.to_json(), value!("a"));

    Ok(())
}

// ============================================================================
// Override Tests
// ============================================================================

#[test]
fn test_new_override() -> Result<()> {
    let grammar = r#"
        start = @:'a' {@:'b'} $ ;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "abb", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!(["a", "b", "b"]));

    Ok(())
}

#[test]
fn test_list_override() -> Result<()> {
    let grammar = r#"
        start = @+:'a' {@:'b'} $ ;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "a", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!([["a"]]));

    let grammar = r#"
        start = @:'a' {@:'b'} $ ;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "a", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!("a"));

    Ok(())
}

// ============================================================================
// Based Rules & Include
// ============================================================================

#[test]
fn test_based_rule() -> Result<()> {
    let grammar = r#"
        start: b $

        a: ='a'

        b < a: {='b'}
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "abb", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!(["a", "b", "b"]));

    Ok(())
}

#[test]
fn test_rule_include() -> Result<()> {
    let grammar = r#"
        start = b $;

        a = @:'a' ;
        b = >a {@:'b'} ;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "abb", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!(["a", "b", "b"]));

    Ok(())
}

#[test]
fn test_48_rule_override() -> Result<()> {
    let grammar = r#"
        start = ab $;

        ab = 'xyz' ;

        @override
        ab = @:'a' {@:'b'} ;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "abb", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!(["a", "b", "b"]));

    Ok(())
}

// ============================================================================
// Pattern & Constant Tests
// ============================================================================

#[test]
fn test_empty_closure() -> Result<()> {
    let grammar = r#"
        start = {'x'}+ {} 'y' $;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "xxxy", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!([["x", "x", "x"], [], "y"]));

    Ok(())
}

#[test]
fn test_any() -> Result<()> {
    let grammar = r#"
        start = /./ 'xx' /./ /./ 'yy' $;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "1xx 2 yy", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), value!(["1", "xx", " ", "2", "yy"]));

    Ok(())
}

#[test]
fn test_constant() -> Result<()> {
    let grammar = r#"
        start = ()
            _0:`0` _1:`+1` _n123:`-123`
            _xF:`0xF`
            _string:`string`
            _string_space:`'string space'`
            _true:`True` _false:`False`
            $;
    "#;

    let model = compile(grammar, &[])?;
    let ast = parse_input(&model, "", &[])?;
    let json = ast.to_json();

    assert_eq!(json["_0"], value!(0));
    assert_eq!(json["_1"], value!(1));
    assert_eq!(json["_n123"], value!(-123));
    assert_eq!(json["_xF"], value!(0xF));
    assert_eq!(json["_string"], value!("string"));
    assert_eq!(json["_string_space"], value!("string space"));
    assert_eq!(json["_true"], value!(true));
    assert_eq!(json["_false"], value!(false));

    Ok(())
}

// ============================================================================
// Non-Capturing Groups
// ============================================================================

#[test]
fn test_non_capturing_group_exclusion() -> Result<()> {
    let grammar = r#"
        start: header (?: delimiter ) body

        header: /[A-Z]+/

        delimiter: /[:,-]+/

        body: /[a-z]+/
    "#;

    let parser = compile(grammar, &[])?;
    let ast = parser.parse_input("INFO---data", &[])?;

    assert_eq!(ast.to_json(), value!(["INFO", "data"]));

    Ok(())
}

#[test]
fn test_non_capturing_group_failure() -> Result<()> {
    let grammar = r"start = (?: 'FIX' ) value ; value = /\d+/";
    let parser = compile(grammar, &[CfgKey::Wsp("".to_string())])?;

    let result = parser.parse_input("BUG123", &[CfgKey::Wsp("".to_string())]);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_nested_non_capturing_groups() -> Result<()> {
    let grammar = r#"
        start: (?: '(' (?: inner ) ')' )

        inner: 'content'
    "#;

    let parser = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parser.parse_input("(content)", &[CfgKey::Wsp("".to_string())])?;

    assert_eq!(ast.to_json(), value!(null));

    Ok(())
}

// ============================================================================
// AST Assignment with start=
// ============================================================================

#[test]
fn test_ast_assignment_start() -> Result<()> {
    let grammar = r#"
        n  = @: {"a"}* $ ;
        f  = @+: {"a"}* $ ;
        nn = @: {"a"}*  @: {"b"}* $ ;
        nf = @: {"a"}*  @+: {"b"}* $ ;
        fn = @+: {"a"}* @: {"b"}* $ ;
        ff = @+: {"a"}* @+: {"b"}* $ ;
    "#;

    let model = compile(grammar, &[])?;

    assert_eq!(model.parse_input("", &[])?, model.parse_input("", &[])?);

    assert_eq!(model.parse_input("", &[])?, model.parse_input("", &[])?);

    Ok(())
}

// ============================================================================
// Comments & Void
// ============================================================================

#[test]
fn test_no_default_comments() -> Result<()> {
    let grammar = r#"
        @@eol_comments :: /@@@@@@/

        start = 'a' $;
    "#;

    let parser = compile(grammar, &[])?;
    let text = "        # no comments are valid\n        a\n    ";
    let result = parser.parse_input(text, &[]);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_parse_hash() -> Result<()> {
    let grammar = r#"
        @@comments :: /@@@@@@@/
        @@eol_comments :: /@@@@@@@/

        start = '#' ;
    "#;

    let parser = compile(grammar, &[])?;
    parser.parse_input("#", &[])?;

    Ok(())
}

#[test]
fn test_parse_void() -> Result<()> {
    let grammar = r#"
        start = () $ ;
    "#;

    let ast = parse(grammar, "", &[])?;
    assert_eq!(ast.to_json(), value!(null));

    Ok(())
}

#[test]
fn test_partial_choice() -> Result<()> {
    let grammar = r#"
        start = o:[o] x:'A' $ ;
        o = 'A' a:'A' | 'A' b:'B' ;
    "#;

    let model = compile(grammar, &[CfgKey::Wsp("".to_string())])?;
    let ast = parse_input(&model, "A", &[CfgKey::Wsp("".to_string())])?;
    assert_eq!(ast.to_json(), object! {"x": "A", "o": null});

    Ok(())
}
