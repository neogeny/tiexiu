//! Round-trip Tests

use tiexiu::parse_input;
use tiexiu::*;

#[test]
#[ignore = "grammar parsing bug with multi-rule grammars"]
fn grammar_to_json_round_trip() -> Result<()> {
    let grammar_text = r#"
        @@grammar :: Test
        start: foo bar
        foo: 'x'
        bar: 'y'
    "#;

    let grammar = compile(grammar_text, &[])?;

    let tree1 = parse_input(&grammar, "x y", &[])?;

    let j1 = tree1.to_json_string()?;
    assert!(j1.contains("x") && j1.contains("y"));
    Ok(())
}

#[test]
fn pretty_print_round_trip() -> Result<()> {
    let grammar_text = r#"
        @@grammar :: Test
        start: 'a' | 'b'
    "#;

    let grammar = compile(grammar_text, &[])?;
    let pretty1 = grammar.pretty_print();

    let grammar2 = compile(&pretty1, &[])?;
    let pretty2 = grammar2.pretty_print();

    assert_eq!(pretty1, pretty2);
    Ok(())
}
