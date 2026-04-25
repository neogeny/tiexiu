//! Directive Tests

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn grammar_directive() -> Result<()> {
    let grammar = r#"
        @@grammar :: MyGrammar
        start: 'test'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    assert_eq!(grammar.name.to_string(), "MyGrammar");
    Ok(())
}

#[test]
fn whitespace_directive() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start: 'a' 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a b", &[])?;
    assert_eq!(tree.to_value(), json!(["a", "b"]));
    Ok(())
}

// Whitespace None directive - disables automatic whitespace handling.
// ISSUE: The @@whitespace :: None directive should disable automatic
// whitespace between tokens. With this directive, `'a' 'b'` should match
// "ab" without requiring a space.
// Expected: Grammar with @@whitespace :: None parses "ab" as ["a", "b"]
// Actual: ParseFailure - directive may not be implemented or not working.
// This suggests the @@whitespace :: None handling is not implemented.
#[test]
fn whitespace_none_directive() -> Result<()> {
    let grammar = r#"
        @@whitespace :: None
        start: 'a' 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "ab", &[])?;
    assert_eq!(tree.to_value(), json!(["a", "b"]));
    Ok(())
}

#[test]
fn default_whitespace() -> Result<()> {
    let grammar = r#"
        start: 'a' 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a b", &[])?;
    assert_eq!(tree.to_value(), json!(["a", "b"]));
    Ok(())
}

#[test]
fn left_recursion_directive() -> Result<()> {
    let grammar = r#"
        @@left_recursion :: False
        start: 'test'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "test", &[])?;
    assert_eq!(tree.to_value(), json!("test"));
    Ok(())
}

#[test]
fn parseinfo_directive() -> Result<()> {
    let grammar = r#"
        @@parseinfo :: True
        start: 'test'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "test", &[])?;
    // parseinfo may add metadata, just check we got text
    assert_eq!(tree.to_value(), json!("test"));
    Ok(())
}

#[test]
fn nameguard_directive() -> Result<()> {
    let grammar = r#"
        @@nameguard :: False
        start: 'ab'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "ab", &[])?;
    assert_eq!(tree.to_value(), json!("ab"));
    Ok(())
}

#[test]
fn comments_directive() -> Result<()> {
    let grammar = r#"
        @@comments :: /#[^\n]*/
        start: 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_value(), json!("a"));
    Ok(())
}