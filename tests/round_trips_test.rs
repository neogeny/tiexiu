//! Round-trip Tests

use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn grammar_to_json_round_trip() -> Result<()> {
    let grammar_text = r#"
        @@grammar :: Test
        start: foo bar
        foo: 'x'
        bar: 'y'
    "#;

    let grammar = tiexiu::compile(grammar_text, &[])?;

    let tree1 = parse_input(&grammar, "x y", &[])?;

    let j1 = tree1.to_model_json_string()?;
    assert!(j1.contains("x") && j1.contains("y"));
    Ok(())
}

#[test]
fn pretty_print_round_trip() -> Result<()> {
    let grammar_text = r#"
        @@grammar :: Test
        start: 'a' | 'b'
    "#;

    let grammar = tiexiu::compile(grammar_text, &[])?;
    let pretty1 = grammar.to_string();

    let grammar2 = tiexiu::compile(&pretty1, &[])?;
    let pretty2 = grammar2.to_string();

    assert!(!pretty1.is_empty());
    assert!(!pretty2.is_empty());
    Ok(())
}
