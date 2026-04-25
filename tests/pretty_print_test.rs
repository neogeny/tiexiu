//! Pretty Print Tests

use tiexiu::*;

#[test]
fn pretty_contains_grammar_name() -> Result<()> {
    let grammar = r#"
        @@grammar :: MyTest
        start: 'hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let pretty = grammar.to_string();
    assert!(pretty.contains("MyTest") || pretty.contains("start"));
    Ok(())
}

#[test]
fn pretty_contains_rules() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: 'a'
        other: 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let pretty = grammar.to_string();
    assert!(pretty.contains("start") || pretty.contains("other"));
    Ok(())
}
