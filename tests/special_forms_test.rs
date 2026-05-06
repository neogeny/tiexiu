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
    let tree = parse_input(&grammar, "abab", &[])?;
    // Group creates a list of the inner expressions
    assert_eq!(tree.to_json(), array![array!["a", "b"], array!["a", "b"]]);
    Ok(())
}

#[test]
fn test_skip_group() -> Result<()> {
    // NOTE: This is a weird grammar!
    let grammar = r#"
        start: (?: 'a' 'b')*
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "abab", &[])?;
    // Skip group doesn't capture the inner expressions
    assert_eq!(tree.to_json(), value!([null, null]));
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
    // Dot matches any character, but doesn't return it
    assert_eq!(tree.to_json(), array!["a", "b"]);
    Ok(())
}

#[test]
#[ignore = "constant evaluation not implemented"]
fn test_constant() -> Result<()> {
    let grammar = r#"
        start: `constant`
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = tiexiu::parse_input(&grammar, "", &[])?;
    // Constant should inject the constant value
    assert_eq!(tree.to_json(), value!("constant"));
    Ok(())
}
