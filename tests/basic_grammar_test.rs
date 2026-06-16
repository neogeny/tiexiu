//! Basic Grammar Tests

use serde_json::json;
use tiexiu::*;

#[test]
fn simple_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: 'hello'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_json(), json!("hello"));
    Ok(())
}

#[test]
fn multiple_rules() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: choice
        
        choice:
            | 'a'
            | 'b'
            | 'c'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_json(), json!("a"));
    Ok(())
}

#[test]
fn rule_references() -> Result<()> {
    // Test rule reference - using single rule for now
    let grammar = r#"
        @@grammar :: Test
        start: 'hello' 'world'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "helloworld", &[])?;
    assert_eq!(tree.to_json(), json!(["hello", "world"]));
    Ok(())
}

#[test]
fn empty_input() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: 'test'?
    "#;
    let grammar = compile(grammar, &[])?;
    // Test with optional matching empty
    let tree = parse_input(&grammar, "", &[])?;
    // Optional with no input should return Nil/None
    assert!(tree.to_json().is_null() || matches!(tree, crate::trees::Tree::Nil));
    Ok(())
}
