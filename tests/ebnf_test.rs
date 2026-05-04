use tiexiu::compile;
use tiexiu::util::indent::dedent_all;

#[test]
fn test_ebnf_parsing() -> tiexiu::Result<()> {
    // Matches TatSu's test_parse_ebnf - verifies EBNF syntax compilation
    let grammar = dedent_all(
        r#"
        /*
            Example of a grammar that mixes TatSu and EBNF
        */
        @@grammar :: EBNF  // this is TatSu with an EBNF comment

        start := expression $

        expression := expression '+' term | expression '-' term | term

        term := term '*' factor | term '/' factor | factor

        factor := '(' expression ')' | number

        number := /\d+/
    "#,
    );

    let g = compile(grammar.as_ref(), &[])?;
    assert_eq!(g.name.to_string(), "EBNF");

    Ok(())
}
