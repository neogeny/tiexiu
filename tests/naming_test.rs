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
// ISSUE: The grammar `start: >base` with `base: 'hello'` should include
// the base rule's expression inline. Currently fails with "Rule not linked".
// Expected: Grammar compiles successfully.
// Actual: ParseFailure with RuleNotLinked("base") error.
// This suggests the rule include operator > is not implemented in the linker.
#[test]
fn rule_include() -> Result<()> {
    let grammar = r#"
        start: >base
        base: 'hello'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}

// Named list with +: modifier.
// ISSUE: The syntax names+:'a' should create a named list accumulator.
// The +: modifier is part of the override/naming special forms.
// Expected: Grammar compiles successfully.
// Actual: ParseFailure with ExpectedPattern or similar error.
// This suggests the +: modifier for named lists is not implemented.
#[test]
fn named_list() -> Result<()> {
    let grammar = r#"
        start: names+:'a'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}

// Override list with @+: modifier.
// ISSUE: The syntax @+:'a' should create a list accumulator via override.
// The @+: modifier is part of the override/naming special forms.
// Expected: Grammar compiles successfully.
// Actual: ParseFailure with ExpectedPattern or similar error.
// This suggests the @+: modifier for override lists is not implemented.
#[test]
fn override_list() -> Result<()> {
    let grammar = r#"
        start: @+:'a'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}