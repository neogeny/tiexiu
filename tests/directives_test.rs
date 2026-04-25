//! Directive Tests

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
    let json = tree.to_model_json_string()?;
    assert!(json.contains("a") && json.contains("b"));
    Ok(())
}

#[test]
fn whitespace_none_directive() -> Result<()> {
    let grammar = r#"
        @@whitespace :: None
        start: 'a' 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "ab", &[])?;
    let json = tree.to_model_json_string()?;
    assert!(json.contains("a") && json.contains("b"));
    Ok(())
}

#[test]
fn default_whitespace() -> Result<()> {
    let grammar = r#"
        start: 'a' 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a b", &[])?;
    let json = tree.to_model_json_string()?;
    assert!(json.contains("a") && json.contains("b"));
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
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
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
    let json = tree.to_model_json_string()?;
    assert!(json.contains("parseinfo") || json.contains("pos"));
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
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
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
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}
