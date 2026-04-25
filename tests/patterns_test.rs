//! Pattern Tests

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn simple_pattern() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: /\d+/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "123", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn pattern_with_letters() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: /[a-z]+/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_value(), json!("hello"));
    Ok(())
}

#[test]
fn pattern_with_anchors() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: /^start/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "start", &[])?;
    assert_eq!(tree.to_value(), json!("hello"));
    Ok(())
}

#[test]
fn pattern_case_insensitive() -> Result<()> {
    let grammar = r#"
        @@ignorecase :: True
        start: /hello/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "HELLO", &[])?;
    assert_eq!(tree.to_value(), json!("HELLO"));
    Ok(())
}

#[test]
fn regex_character_classes() -> Result<()> {
    let grammar = r#"
        start: /[A-Za-z_]\w*/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello_world", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}
