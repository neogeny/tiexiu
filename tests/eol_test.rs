// Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: BSD-4-Clause

//! End-of-line tests for TieXiu
//!
//! Tests translated from TatSu's test_semantics.py

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn test_basic_eol() -> Result<()> {
    let grammar = r#"
        start = 'hello' $-> 'world'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let ast = parse_input(&model, "hello\nworld", &[])?;
    assert_eq!(ast.to_value(), json!(["hello", "world"]));

    let ast = parse_input(&model, "hello  \nworld", &[])?;
    assert_eq!(ast.to_value(), json!(["hello", "world"]));

    let res = parse_input(&model, "hello world", &[]);
    assert!(res.is_err(), "Should fail: no line break");

    let res = parse_input(&model, "helloX\nworld", &[]);
    assert!(res.is_err(), "Should fail: non-whitespace before EOL");

    Ok(())
}

#[test]
fn test_eol_at_end_of_text() -> Result<()> {
    let grammar = r#"
        start = 'hello' $-> $
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let ast = parse_input(&model, "hello\n", &[])?;
    assert_eq!(ast.to_value(), json!("hello"));

    let ast = parse_input(&model, "hello  \n", &[])?;
    assert_eq!(ast.to_value(), json!("hello"));

    let res = parse_input(&model, "hello world", &[]);
    assert!(res.is_err(), "Should fail: no line break at end");

    Ok(())
}

#[test]
fn test_multiple_eols() -> Result<()> {
    let grammar = r#"
        start = 'line1' $-> 'line2' $-> 'line3'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let ast = parse_input(&model, "line1\nline2\nline3", &[])?;
    assert_eq!(ast.to_value(), json!(["line1", "line2", "line3"]));

    let ast = parse_input(&model, "line1  \nline2\n  line3", &[])?;
    assert_eq!(ast.to_value(), json!(["line1", "line2", "line3"]));

    Ok(())
}

#[test]
fn test_eol_with_indentation() -> Result<()> {
    let grammar = r#"
        start = 'start' $-> 'indented' $-> 'end'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let ast = parse_input(&model, "start\n  indented\nend", &[])?;
    assert_eq!(ast.to_value(), json!(["start", "indented", "end"]));

    let ast = parse_input(&model, "start\nindented\nend", &[])?;
    assert_eq!(ast.to_value(), json!(["start", "indented", "end"]));

    Ok(())
}

#[test]
fn test_eol_in_closure() -> Result<()> {
    let grammar = r#"
        start = ('item' $->)* 'end'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let ast = parse_input(&model, "item\nitem\nend", &[])?;
    assert_eq!(ast.to_value(), json!([["item", "item"], "end"]));

    let ast = parse_input(&model, "item  \nitem\nend", &[])?;
    assert_eq!(ast.to_value(), json!([["item", "item"], "end"]));

    let ast = parse_input(&model, "end", &[])?;
    assert_eq!(ast.to_value(), json!([[], "end"]));

    Ok(())
}

#[test]
fn test_eol_with_comments() -> Result<()> {
    let grammar = r#"
        start = 'hello' $-> 'world'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    // Without comment support, comments are treated as content
    // This test verifies basic EOL behavior
    let ast = parse_input(&model, "hello\nworld", &[])?;
    assert_eq!(ast.to_value(), json!(["hello", "world"]));

    Ok(())
}

#[test]
fn test_eol_with_mixed_whitespace() -> Result<()> {
    let grammar = r#"
        start = 'start' $-> 'next'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let ast = parse_input(&model, "start \t \nnext", &[])?;
    assert_eq!(ast.to_value(), json!(["start", "next"]));

    let ast = parse_input(&model, "start   \nnext", &[])?;
    assert_eq!(ast.to_value(), json!(["start", "next"]));

    let ast = parse_input(&model, "start\t\nnext", &[])?;
    assert_eq!(ast.to_value(), json!(["start", "next"]));

    Ok(())
}

#[test]
fn test_eol_no_whitespace_before_linebreak() -> Result<()> {
    let grammar = r#"
        start = 'start' $-> 'next'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let ast = parse_input(&model, "start\nnext", &[])?;
    assert_eq!(ast.to_value(), json!(["start", "next"]));

    Ok(())
}

#[test]
fn test_eol_followed_by_non_whitespace() -> Result<()> {
    let grammar = r#"
        start = 'start' $-> 'next'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let res = parse_input(&model, "startX\nnext", &[]);
    assert!(res.is_err(), "Should fail: non-whitespace before EOL");

    Ok(())
}

#[test]
fn test_eol_followed_by_non_linebreak() -> Result<()> {
    let grammar = r#"
        start = 'start' $-> 'next'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    let res = parse_input(&model, "start next", &[]);
    assert!(res.is_err(), "Should fail: no linebreak");

    Ok(())
}

#[test]
fn test_eol_in_tatsu_ebnf_endrule() -> Result<()> {
    // Simpler test: EOL in choice with semicolon
    let grammar = r#"
        start = 'a' (';' | $->) 'b'
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    // Test with EOL
    let ast = parse_input(&model, "a\nb", &[])?;
    assert_eq!(ast.to_value(), json!(["a", "b"]));

    // Test with semicolon
    let ast = parse_input(&model, "a;b", &[])?;
    assert_eq!(ast.to_value(), json!(["a", ";", "b"]));

    Ok(())
}
