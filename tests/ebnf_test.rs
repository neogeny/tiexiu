use tiexiu::util::indent::dedent_all;
use tiexiu::{Cfg, compile};

#[test]
// #[ignore]
fn test_ebnf_parsing() -> tiexiu::Result<()> {
    // TODO: cause of failure - verify full EBNF grammar parsing
    let grammar = dedent_all(
        r#"
        /*
            Example of a grammar that mixes TatSu and EBNF
        */
        @@grammar :: EBNF  // this is TatSu wiht an EBNF comment
        @@keyword :: None At
        @@keyword :: This Time

        start := expression $


        expression := expression '+' term | expression '-' term | term
        
        term := term '*' factor | term '/' factor | factor
        
        factor := '(' expression ')' | number
        
        number := /\d+/
    "#,
    );

    let g = compile(grammar.as_ref(), &[])?;

    assert_eq!(g.name.to_string(), "EBNF");
    assert_eq!(g.rules.len(), 5, "Unexpected number of rules");
    assert_eq!(g.keywords.len(), 4, "Unexpected number of keywords");

    eprintln!("GRAMMAR\n{:#?}", g);

    let mut tree;

    tree = g.parse_input("3", &[Cfg::Trace])?;
    eprintln!("{:#?}", tree);
    eprintln!("{:#?}", tree.to_value());
    eprintln!("{:#}", tree.to_string_pretty()?);

    tree = g.parse_input("3 * (2 + 5)", &[Cfg::Trace])?;
    eprintln!("{:#?}", tree);

    // Err(Error::AndNowAMessageFromYourFriendlyTest(
    //     "aborted".to_string(),
    // ))
    Ok(())
}
