//! Basic Grammar Tests

use tiexiu::*;

#[test]
fn simple_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: 'hello'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert!(matches!(tree, Tree::Node { .. }));
    Ok(())
}

#[test]
fn multiple_rules() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: a | b | c
        a: 'a'
        b: 'b'
        c: 'c'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert!(matches!(tree, Tree::Node { .. }));
    Ok(())
}

#[test]
fn rule_references() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: foo bar
        foo: 'hello'
        bar: 'world'
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello world", &[])?;
    let json = tree.to_model_json_string()?;
    assert!(json.contains("hello") && json.contains("world"));
    Ok(())
}

#[test]
fn empty_input() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: ''
    "#;
    let grammar = compile(grammar, &[])?;
    let tree = parse_input(&grammar, "", &[])?;
    assert!(matches!(tree, Tree::Node { .. }));
    Ok(())
}
