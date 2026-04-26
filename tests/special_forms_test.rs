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
#[test]
fn void() -> Result<()> {
    let grammar = r#"
        start: 'a' () 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a b", &[])?;
    assert_eq!(tree.to_json(), json!(["a", "b"]));
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
    assert_eq!(tree.to_json(), json!("a"));
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
    assert_eq!(tree.to_json(), json!(["a", "b"]));
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
