//! Token and Sequence Tests

#[macro_use]
extern crate json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn token_sequence() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /\s+/
        start: 'hello' 'world'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello world", &[])?;
    assert_eq!(tree.to_json(), array!["hello", "world"]);
    Ok(())
}

#[test]
fn optional_token() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /\s+/
        start: 'a' 'b'?
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "a b", &[])?;
    assert_eq!(tree.to_json(), array!["a", "b"]);

    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_json(), value!("a"));
    Ok(())
}

#[test]
fn closure_tokens() -> Result<()> {
    let grammar = r#"
        start: 'a'*
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "aaa", &[])?;
    eprintln!("closure result: {:?}", tree.to_json());
    Ok(())
}

#[test]
fn positive_closure() -> Result<()> {
    let grammar = r#"
        start: 'a'+
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "aaa", &[])?;
    assert_eq!(tree.to_json(), value!(["a", "a", "a"]));
    Ok(())
}

#[test]
fn choice_alternatives() -> Result<()> {
    let grammar = r#"
        start: 'a' | 'b' | 'c'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    assert_eq!(parse_input(&grammar, "a", &[])?.to_json(), value!("a"));
    assert_eq!(parse_input(&grammar, "b", &[])?.to_json(), value!("b"));
    assert_eq!(parse_input(&grammar, "c", &[])?.to_json(), value!("c"));
    Ok(())
}
