//! Complex Grammar Tests - EXACT translations from Python

use tiexiu::parse_input;
use tiexiu::util::indent::dedent;
use tiexiu::*;

#[test]
fn calculator_grammar() -> Result<()> {
    // Exact grammar from Python bench_parse_test.py CALC_GRAMMAR
    let grammar = r#"
        @@grammar :: CALC

        start: expression $

        expression:
            | expression '+' term
            | expression '-' term
            | term

        term:
            | term '*' factor
            | term '/' factor
            | factor

        factor:
            | NUMBER
            | '(' expression ')'

        NUMBER: /\d+/
    "#;
    let model = tiexiu::compile(grammar, &[])?;

    // Python: parser.parse("3 + 5 * (10 - 20 )")
    let tree = parse_input(&model, "3 + 5 * (10 - 20 )", &[])?;
    let json = tree.to_json();
    // Should produce structured tree: ["3", "+", ["5", "*", ["10", "-", "20"]]
    assert!(!json.is_null(), "Expected result, got null");
    Ok(())
}

#[test]
fn json_like_grammar() -> Result<()> {
    // Exact grammar from Python test_json_like_grammar
    let grammar = r#"
        @@grammar :: MiniJSON
        @@nameguard :: False
        @@whitespace :: /\s+/
        start: value $

        value: object | array | string | number | 'true' | 'false' | 'null'

        object: '{' members? '}'
        array: '[' elements? ']'
        members: pair (',' pair)*
        elements: value (',' value)*
        pair: string ':' value
        string: '"' CONTENT '"'
        CONTENT: /[^"]*/
        number: /-?\d+(\.\d+)?/
    "#;
    eprintln!("GRAMMAR DEDENT\n{}", dedent(grammar));
    eprintln!(
        "GRAMMAR TREE\n{:#?}",
        parse_grammar(dedent(grammar).as_ref(), &[])
    );
    let model = tiexiu::compile(dedent(grammar).as_ref(), &[])?;

    // Python: parser.parse('{"key": "value"}')
    let tree = parse_input(&model, r#"{"key": "value"}"#, &[])?;
    let json = tree.to_json();
    assert!(!json.is_null(), "Expected result, got null");
    Ok(())
}

#[test]
fn lisp_like_grammar() -> Result<()> {
    // Exact grammar from Python test_lisp_like_grammar
    let grammar = r#"
        @@grammar :: Lisp
        @@nameguard :: False
        @@whitespace :: /\s+/
        start: sexp $

        sexp: atom | list

        list: '(' items ')'
        items: sexp*
        atom: WORD
        WORD: /\w+/
    "#;
    let model = tiexiu::compile(dedent(grammar).as_ref(), &[])?;

    // Python: parser.parse("(hello world)")
    let tree = parse_input(&model, "(hello world)", &[])?;
    let json = tree.to_json();
    assert!(!json.is_null(), "Expected result, got null");
    Ok(())
}
