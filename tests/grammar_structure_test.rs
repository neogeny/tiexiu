//! Grammar Structure Tests

use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn grammar_has_rules() -> Result<()> {
    let grammar = r#"
        start: 'a'
        rule1: 'b'
        rule2: 'c'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let rule_names: Vec<_> = grammar.rules().map(|r| &*r.name).collect();
    assert!(rule_names.contains(&"start"));
    assert!(rule_names.contains(&"rule1"));
    assert!(rule_names.contains(&"rule2"));
    Ok(())
}

#[test]
fn first_rule_is_default() -> Result<()> {
    let grammar = r#"
        start: 'a'
        other: 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn pretty_print() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: 'a' | 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let pretty = grammar.to_string();
    assert!(pretty.contains("Test") || pretty.contains("start"));
    Ok(())
}
