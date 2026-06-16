// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde_json::json;
use tiexiu::Result;
use tiexiu::parse;

#[test]
fn test_meta_name() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @name $ ;
    "#;
    let tree = parse(grammar, "hello_world", &[])?;
    assert_eq!(tree.to_json(), json!("hello_world"));
    Ok(())
}

#[test]
fn test_meta_name_failure() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @name $ ;
    "#;
    let result = parse(grammar, "123bad", &[]);
    assert!(result.is_err(), "Expected parse error for @name on digits");
    Ok(())
}

#[test]
fn test_meta_named_name() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = name=@name $ ;
    "#;
    let tree = parse(grammar, "hello_world", &[])?;
    assert_eq!(tree.to_json(), json!({"name": "hello_world"}));
    Ok(())
}

#[test]
fn test_meta_int() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @int $ ;
    "#;
    let tree = parse(grammar, "42", &[])?;
    assert_eq!(tree.to_json(), json!(42.0));
    Ok(())
}

#[test]
fn test_meta_int_negative() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @int $ ;
    "#;
    let tree = parse(grammar, "-17", &[])?;
    assert_eq!(tree.to_json(), json!(-17.0));
    Ok(())
}

#[test]
fn test_meta_int_failure() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @int $ ;
    "#;
    let result = parse(grammar, "abc", &[]);
    assert!(result.is_err(), "Expected parse error for @int on alpha");
    Ok(())
}

#[test]
fn test_meta_uint() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @uint $ ;
    "#;
    let tree = parse(grammar, "007", &[])?;
    assert_eq!(tree.to_json(), json!(7.0));
    Ok(())
}

#[test]
fn test_meta_uint_failure() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @uint $ ;
    "#;
    let result = parse(grammar, "-1", &[]);
    assert!(
        result.is_err(),
        "Expected parse error for @uint on negative"
    );
    Ok(())
}

#[test]
fn test_meta_float() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @float $ ;
    "#;
    let tree = parse(grammar, "7.53", &[])?;
    assert_eq!(tree.to_json(), json!(7.53));
    Ok(())
}

#[test]
fn test_meta_float_exponent() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @float $ ;
    "#;
    let tree = parse(grammar, "1.5e-2", &[])?;
    assert_eq!(tree.to_json(), json!(0.015));
    Ok(())
}

#[test]
fn test_meta_float_failure() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @float $ ;
    "#;
    let result = parse(grammar, "abc", &[]);
    assert!(result.is_err(), "Expected parse error for @float on alpha");
    Ok(())
}

#[test]
fn test_meta_bool_true() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @bool $ ;
    "#;
    let tree = parse(grammar, "true", &[])?;
    assert_eq!(tree.to_json(), json!(true));
    Ok(())
}

#[test]
fn test_meta_bool_false() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @bool $ ;
    "#;
    let tree = parse(grammar, "false", &[])?;
    assert_eq!(tree.to_json(), json!(false));
    Ok(())
}

#[test]
fn test_meta_bool_failure() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @bool $ ;
    "#;
    let result = parse(grammar, "TRUE", &[]);
    assert!(
        result.is_err(),
        "Expected parse error for @bool on uppercase"
    );
    Ok(())
}

#[test]
fn test_meta_multiple_in_sequence() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @int @float @name $ ;
    "#;
    let tree = parse(grammar, "42 3.14 hello", &[])?;
    let _json = tree.to_json();
    Ok(())
}
