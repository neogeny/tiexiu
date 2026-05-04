//! Directive Tests

#[macro_use]
extern crate json;
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
    assert_eq!(tree.to_json(), array!["a", "b"]);
    Ok(())
}

// Whitespace None directive - disables automatic whitespace handling.
// NOT IMPLEMENTED: The @@whitespace :: None directive is not yet implemented.
// When implemented, it should disable automatic whitespace between tokens,
// allowing 'a' 'b' to match "ab" without requiring a space.
#[test]
fn whitespace_none_directive() -> Result<()> {
    let grammar = r#"
        @@whitespace :: None
        @@nameguard :: False
        
        start: 'a' 'b'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}

// TieXiu has no default whitespace; this test is for TatSu's default behavior.
// TODO: enable if TieXiu gains default whitespace handling.
#[ignore = "TieXiu has no default whitespace like TatSu"]
#[test]
fn default_whitespace() -> Result<()> {
    let grammar = r#"
        start: 'a' 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a b", &[])?;
    assert_eq!(tree.to_json(), array!["a", "b"]);
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
    assert_eq!(tree.to_json(), value!("test"));
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
    assert_eq!(tree.to_json(), value!("test"));
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
    assert_eq!(tree.to_json(), value!("ab"));
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
    assert_eq!(tree.to_json(), value!("a"));
    Ok(())
}
