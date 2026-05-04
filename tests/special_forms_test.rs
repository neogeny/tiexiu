//! Special Forms Tests (Group, Void, EOF, Dot, Constant)

#[macro_use]
extern crate json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn test_group() -> Result<()> {
    let grammar = r#"
        start: ('a' 'b')*
    "#;
    let grammar = compile(grammar, &[])?;
    let _tree = parse_input(&grammar, "abab", &[])?;
    // Group creates a list
    Ok(())
}

#[test]
fn test_skip_group() -> Result<()> {
    let grammar = r#"
        start: (?: 'a' 'b')*
    "#;
    let grammar = compile(grammar, &[])?;
    let _tree = parse_input(&grammar, "abab", &[])?;
    // Skip group doesn't capture
    Ok(())
}

// Void expression that produces nothing.
#[test]
fn test_void() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /\s+/
        start: 'a' () 'b'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a b", &[])?;
    assert_eq!(tree.to_json(), array!["a", "b"]);
    Ok(())
}

#[test]
fn test_eof() -> Result<()> {
    let grammar = r#"
        start: 'a' $
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    // EOF anchors to end
    assert_eq!(tree.to_json(), value!("a"));
    Ok(())
}

#[test]
fn test_dot() -> Result<()> {
    let grammar = r#"
        start: /./ 'b'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "ab", &[])?;
    // Dot matches any character, but does not retur it
    assert_eq!(tree.to_json(), value!(["a", "b"]));
    Ok(())
}

#[test]
fn test_constant() -> Result<()> {
    let grammar = r#"
        start: `constant`
    "#;
    let grammar = compile(grammar, &[])?;
    let _tree = parse_input(&grammar, "", &[])?;
    // Constant is a special case
    Ok(())
}
