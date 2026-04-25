//! Special Forms Tests (Group, Void, EOF, Dot, Constant)

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn group() -> Result<()> {
    let grammar = r#"
        start: ('a' 'b')*
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let _tree = parse_input(&grammar, "abab", &[])?;
    // Group creates a list
    Ok(())
}

#[test]
fn skip_group() -> Result<()> {
    let grammar = r#"
        start: (?: 'a' 'b')*
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let _tree = parse_input(&grammar, "abab", &[])?;
    // Skip group doesn't capture
    Ok(())
}

// Void expression that produces nothing.
// ISSUE: The grammar `'a' () 'b'` with input "ab" fails to parse.
// The void expression `()` should consume no input and produce nothing,
// allowing 'a' to match 'a' and 'b' to match 'b'.
// Expected: "ab" -> ["a", "b"]
// Actual: Parse fails with ExpectedToken("a") error at position 0.
// This suggests the void expression `()` may not be implemented correctly,
// or it's consuming input that it shouldn't.
#[test]
fn void() -> Result<()> {
    let grammar = r#"
        start: 'a' () 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "ab", &[])?;
    assert_eq!(tree.to_value(), json!(["a", "b"]));
    Ok(())
}

#[test]
fn eof() -> Result<()> {
    let grammar = r#"
        start: 'a' $
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    // EOF anchors to end
    assert_eq!(tree.to_value(), json!("a"));
    Ok(())
}

#[test]
fn dot() -> Result<()> {
    let grammar = r#"
        start: /./ 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "ab", &[])?;
    // Dot matches any character
    assert_eq!(tree.to_value(), json!(["a", "b"]));
    Ok(())
}

#[test]
fn constant() -> Result<()> {
    let grammar = r#"
        start: `constant`
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let _tree = parse_input(&grammar, "", &[])?;
    // Constant is a special case
    Ok(())
}