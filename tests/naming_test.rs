//! Named Rules and Overrides Tests

use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn named_capture() -> Result<()> {
    let grammar = r#"
        start: name='hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    let json = tree.to_model_json_string()?;
    assert!(json.contains("name"));
    Ok(())
}

#[test]
fn named_list() -> Result<()> {
    let grammar = r#"
        start: names+:'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "aaa", &[])?;
    let json = tree.to_model_json_string()?;
    assert!(json.contains("names"));
    Ok(())
}

#[test]
fn override_singleton() -> Result<()> {
    let grammar = r#"
        start: ='hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn override_list() -> Result<()> {
    let grammar = r#"
        start: @+:'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "aaa", &[])?;
    let json = tree.to_model_json_string()?;
    assert!(json.contains("@+") || json.contains("OverrideList"));
    Ok(())
}

#[test]
fn rule_include() -> Result<()> {
    let grammar = r#"
        start: >base
        base: 'hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}
