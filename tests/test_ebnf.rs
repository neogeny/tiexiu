use tiexiu::compile;

#[test]
fn test_ebnf_parsing() -> tiexiu::Result<()> {
    let grammar = r#"
        /*
            Example of a grammar that mixes TatSu and EBNF
        */
        @@grammar :: EBNF  // this is TatSu wiht an EBNF comment

        start := expression $

        expression := expression '+' term | expression '-' term | term

        term := term '*' factor | term '/' factor | factor

        factor := '(' expression ')' | number

        number := /\d+/
    "#;

    let g = compile(grammar, &[])?;

    assert_eq!(g.name, "EBNF");

    Ok(())
}
