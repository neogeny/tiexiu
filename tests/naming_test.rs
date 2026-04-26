//! Named Rules and Overrides Tests

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn named_capture() -> Result<()> {
    let grammar = r#"
        start: name='hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_value(), json!({"name": "hello"}));
    Ok(())
}

#[test]
fn override_singleton() -> Result<()> {
    let grammar = r#"
        start: ='hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_value(), json!("hello"));
    Ok(())
}

// Rule include with > operator.
// NOT IMPLEMENTED: The grammar `start: >base` with `base: 'hello'` fails
// with MissingKey error. The RuleInclude expression is not handled properly
// by the linker. When implemented, it should inline the base rule's expression.
#[test]
#[ignore = "RuleInclude not implemented in linker"]
fn rule_include() -> Result<()> {
    let grammar = r#"
        start: >base
        base: 'hello'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}

// Named list with +: modifier.
// The syntax names+:'a' creates a named list accumulator.
// This works with the TatSu grammar import.
#[test]
fn named_list() -> Result<()> {
    let grammar = r#"
        start: names+:'a'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}

// Override list with @+: modifier.
// The syntax @+:'a' creates a list accumulator via override.
// This works with the TatSu grammar import.
#[test]
fn override_list() -> Result<()> {
    let grammar = r#"
        start: @+:'a'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}
