//! Token and Sequence Tests

use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn token_sequence() -> Result<()> {
    let grammar = r#"
        start: 'hello' 'world'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello world", &[])?;
    let json = tree.to_model_json_string()?;
    assert!(json.contains("hello") && json.contains("world"));
    Ok(())
}

#[test]
fn optional_token() -> Result<()> {
    let grammar = r#"
        start: 'a' 'b'?
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "a b", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

    let tree = parse_input(&grammar, "a", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn closure_tokens() -> Result<()> {
    let grammar = r#"
        start: 'a'*
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "aaa", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn positive_closure() -> Result<()> {
    let grammar = r#"
        start: 'a'+
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "aaa", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn choice_alternatives() -> Result<()> {
    let grammar = r#"
        start: 'a' | 'b' | 'c'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    assert!(matches!(
        parse_input(&grammar, "a", &[])?,
        tiexiu::trees::Tree::Node { .. }
    ));
    assert!(matches!(
        parse_input(&grammar, "b", &[])?,
        tiexiu::trees::Tree::Node { .. }
    ));
    assert!(matches!(
        parse_input(&grammar, "c", &[])?,
        tiexiu::trees::Tree::Node { .. }
    ));
    Ok(())
}
