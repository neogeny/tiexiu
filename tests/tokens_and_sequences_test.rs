//! Token and Sequence Tests

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn token_sequence() -> Result<()> {
    let grammar = r#"
        start: 'hello' 'world'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello world", &[])?;
    assert_eq!(tree.to_value(), json!(["hello", "world"]));
    Ok(())
}

// Optional tokens with whitespace handling.
// ISSUE: The grammar `'a' 'b'?` with input "a b" fails because whitespace
// between 'a' and 'b?' is not being consumed properly.
// Expected: "a b" -> ["a", "b"], "a" -> ["a"]
// Actual: Parse fails on "a b" with ExpectedToken("b") error.
// This may be related to how whitespace is handled between tokens.
#[test]
fn optional_token() -> Result<()> {
    let grammar = r#"
        start: 'a' 'b'?
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "a b", &[])?;
    assert_eq!(tree.to_value(), json!(["a", "b"]));

    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_value(), json!(["a"]));
    Ok(())
}

#[test]
fn closure_tokens() -> Result<()> {
    let grammar = r#"
        start: 'a'*
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "aaa", &[])?;
    eprintln!("closure result: {:?}", tree.to_value());
    Ok(())
}

// Positive closure with whitespace handling.
// ISSUE: Similar to optional_token - the grammar `'a'+` with input "aaa"
// may fail because whitespace between 'a' repetitions is not being consumed.
// Expected: "aaa" -> ["a", "a", "a"]
// Actual: Parse may fail or produce incorrect results.
// This is related to the whitespace handling between repeated tokens.
#[test]
fn positive_closure() -> Result<()> {
    let grammar = r#"
        start: 'a'+
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "aaa", &[])?;
    assert_eq!(tree.to_value(), json!(["a", "a", "a"]));
    Ok(())
}

#[test]
fn choice_alternatives() -> Result<()> {
    let grammar = r#"
        start: 'a' | 'b' | 'c'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    assert_eq!(parse_input(&grammar, "a", &[])?.to_value(), json!("a"));
    assert_eq!(parse_input(&grammar, "b", &[])?.to_value(), json!("b"));
    assert_eq!(parse_input(&grammar, "c", &[])?.to_value(), json!("c"));
    Ok(())
}