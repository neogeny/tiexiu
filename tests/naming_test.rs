//! Named Rules and Overrides Tests

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn named_capture() -> Result<()> {
    let grammar = r#"
        start: name='hello'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_json(), json!({ "name": "hello"}));
    Ok(())
}

#[test]
fn override_singleton() -> Result<()> {
    let grammar = r#"
        start: ='hello'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_json(), json!("hello"));
    Ok(())
}
