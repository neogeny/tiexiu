//! Basic Grammar Tests

#[macro_use]
extern crate json;
use tiexiu::*;

#[test]
fn simple_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: 'hello'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_json(), value!("hello"));
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
    assert_eq!(tree.to_json(), value!("a"));
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
    assert_eq!(tree.to_json(), array!["hello", "world"]);
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
    let _tree = parse_input(&grammar, "", &[])?;
    // Optional without input
    Ok(())
}
