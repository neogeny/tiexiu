use tiexiu::compile;
use tiexiu::util::indent::dedent_all;

#[test]
// #[ignore]
fn test_ebnf_parsing() -> tiexiu::Result<()> {
    let grammar = dedent_all(
        r#"
        /*
            Example of a grammar that mixes TatSu and EBNF
        */
        @@grammar :: EBNF  // this is TatSu wiht an EBNF comment
        
        start := expression $
        
        expression := expression '+' term | expression '-' term | term
        
        term := term '*' factor | term '/' factor | factor
        
        factor := '(' expression ')' | number
        
        number := /\d+/
    "#,
    );

    let g = compile(grammar.as_ref(), &[("trace", "1")])?;

    assert_eq!(g.name.to_string(), "EBNF");

    Ok(())
}
