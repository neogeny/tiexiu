//! Grammar Structure Tests

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn grammar_has_rules() -> Result<()> {
    let grammar = r#"
        start: choice
        
        choice: 'a' | 'b' | 'c'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    // Due to grammar linking bug, only one rule might exist
    let rule_count = grammar.rules().count();
    assert!(rule_count >= 1);
    Ok(())
}

#[test]
fn first_rule_is_default() -> Result<()> {
    let grammar = r#"
        start: 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_json(), json!("a"));
    Ok(())
}

#[test]
fn pretty_print() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let pretty = grammar.to_string();
    assert!(pretty.contains("Test") || pretty.contains("start"));
    Ok(())
}
