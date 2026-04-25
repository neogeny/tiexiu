//! Bootstrap tests for TieXiu
//!
//! Tests for parsing grammars in TatSu EBNF format.
//! Run with: cargo test --features bootstrap
//!

// =============================================================================
// Boot Grammar Parsing Tests - Grammar Syntax
// =============================================================================

mod parse_grammar {
    use tiexiu::Result;
    use tiexiu::api::parse_grammar;

    #[test]
    fn simple_grammar() -> Result<()> {
        // TODO: cause of failure - verify bootstrap grammar parsing
        let grammar = r#"
            @@grammar :: Simple
            start: 'hello'
        "#;
        let tree = parse_grammar(grammar, &[])?;
        eprintln!("TREE:\n{:#?}", tree);
        eprintln!("TREE String:\n{}", tree);
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

        assert!(tree.to_model_json_string()?.contains("Simple"));

        eprintln!("GRAMMAR:\n{}", tiexiu::pretty(grammar, &[])?);
        let parser = tiexiu::compile(grammar, &[])?;
        let tree = tiexiu::parse_input(&parser, "hello", &[])?;
        eprintln!("PARSED:\n{}", tree);
        assert_eq!(tree.to_string(), "hello");

        let res = tiexiu::parse_input(&parser, "world", &[]);
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn multiple_rules() -> Result<()> {
        // TODO: cause of failure - verify bootstrap grammar parsing
        let grammar = r#"
            @@grammar :: Multi
            start: a | b | c
            a: 'a'
            b: 'b'
            c: 'c'
        "#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("rules"));
        Ok(())
    }

    #[test]
    fn directive() -> Result<()> {
        // TODO: cause of failure - verify directive parsing
        let grammar = r#"
            @@grammar :: Directives
            @@whitespace :: /\s+/
            start: 'hello'
        "#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("directive"));
        Ok(())
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Expression Types
// =============================================================================

mod parse_expressions {
    use tiexiu::Result;
    use tiexiu::api::*;

    #[test]
    fn token() -> Result<()> {
        // TODO: cause of failure - verify expression parsing
        let grammar = r#"@@grammar :: T start: 'foo' 'bar'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Token"));
        Ok(())
    }

    #[test]
    fn pattern() -> Result<()> {
        // TODO: cause of failure - verify expression parsing
        let grammar = r#"@@grammar :: P start: /\d+/"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Pattern"));
        Ok(())
    }

    #[test]
    fn sequence() -> Result<()> {
        // TODO: cause of failure - verify expression parsing
        let grammar = r#"@@grammar :: S start: 'a' 'b' 'c'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Sequence"));
        Ok(())
    }

    #[test]
    fn choice() -> Result<()> {
        // TODO: cause of failure - verify expression parsing
        let grammar = r#"@@grammar :: C start: 'a' | 'b' | 'c'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Choice"));
        Ok(())
    }

    #[test]
    fn optional() -> Result<()> {
        // TODO: cause of failure - verify expression parsing
        let grammar = r#"@@grammar :: O start: 'a' 'b'? 'c'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Optional"));
        Ok(())
    }

    #[test]
    fn closure() -> Result<()> {
        // TODO: cause of failure - verify expression parsing
        let grammar = r#"@@grammar :: Cl start: 'a'*"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Closure"));
        Ok(())
    }

    #[test]
    fn positive_closure() -> Result<()> {
        // TODO: cause of failure - verify expression parsing
        let grammar = r#"@@grammar :: PC start: 'a'+"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("PositiveClosure"));
        Ok(())
    }

    #[test]
    fn group() -> Result<()> {
        // TODO: cause of failure - verify expression parsing
        let grammar = r#"@@grammar :: G start: ('a' 'b')*"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Group"));
        Ok(())
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Lookahead and Constraints
// =============================================================================

mod parse_constraints {
    use tiexiu::Result;
    use tiexiu::api::parse_grammar;

    #[test]
    fn lookahead() -> Result<()> {
        // TODO: cause of failure - verify constraint parsing
        let grammar = r#"@@grammar :: L start: &'a' 'a'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Lookahead"));
        Ok(())
    }

    #[test]
    fn negative_lookahead() -> Result<()> {
        // TODO: cause of failure - verify constraint parsing
        let grammar = r#"@@grammar :: NL start: !'b' 'a'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("NegativeLookahead"));
        Ok(())
    }

    #[test]
    fn cut() -> Result<()> {
        // TODO: cause of failure - verify cut parsing
        let grammar = r#"@@grammar :: Cu start: 'a' ~ 'b'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Cut"));
        Ok(())
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Naming and References
// =============================================================================

mod parse_naming {
    use tiexiu::*;

    #[test]
    fn named() -> Result<()> {
        // TODO: cause of failure - verify naming parsing
        let grammar = r#"@@grammar :: N start: name='a'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Named"));
        assert!(json.contains("name"));
        Ok(())
    }

    #[test]
    fn rule_call() -> Result<()> {
        // TODO: cause of failure - verify rule call parsing
        let grammar = r#"
            @@grammar :: RC
            start: foo
            foo: 'x'
        "#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Call"));
        Ok(())
    }

    #[test]
    fn rule_include() -> Result<()> {
        // TODO: cause of failure - verify rule include parsing
        let grammar = r#"
            @@grammar :: RI
            start: >base
            base: 'x'
        "#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("RuleInclude"));
        Ok(())
    }

    #[test]
    fn rule_with_params() -> Result<()> {
        // TODO: cause of failure - verify param parsing
        let grammar = r#"
            @@grammar :: RWP

            start: foo ;

            foo[Foo]: 'x' param ;
        "#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Foo"));
        Ok(())
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Special Forms
// =============================================================================

mod parse_special {
    use tiexiu::*;

    #[test]
    fn void() -> Result<()> {
        // TODO: cause of failure - verify special form parsing
        let grammar = r#"@@grammar :: V start: 'a' () 'b'"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Void"));
        Ok(())
    }

    #[test]
    fn eof() -> Result<()> {
        // TODO: cause of failure - verify special form parsing
        let grammar = r#"@@grammar :: E start: 'a' $"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("EOF"));
        Ok(())
    }

    #[test]
    fn dot() -> Result<()> {
        // TODO: cause of failure - verify special form parsing
        let grammar = r#"@@grammar :: D start: /./"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        eprintln!("{:#}", json);
        assert!(json.contains("Dot"));
        Ok(())
    }

    #[test]
    fn constant() -> Result<()> {
        // TODO: cause of failure - verify special form parsing
        let grammar = r#"@@grammar :: Cst start: `constant`"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Constant"));
        Ok(())
    }

    #[test]
    fn join() -> Result<()> {
        // TODO: cause of failure - verify join parsing
        let grammar = r#"@@grammar :: J start: ','%{'a'}*"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Join"));
        Ok(())
    }

    #[test]
    fn gather() -> Result<()> {
        // TODO: cause of failure - verify gather parsing
        let grammar = r#"@@grammar :: Gt start: ','.{'a'}*"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Gather"));
        Ok(())
    }

    #[test]
    fn skip_group() -> Result<()> {
        // TODO: cause of failure - verify special form parsing
        let grammar = r#"@@grammar :: Sk start: (?: 'a' 'b')*"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("SkipGroup"));
        Ok(())
    }

    #[test]
    fn alert() -> Result<()> {
        // TODO: cause of failure - verify special form parsing
        let grammar = r#"@@grammar :: Al start: ^^`danger`"#;
        let tree = parse_grammar(grammar, &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("Alert"));
        Ok(())
    }
}

// =============================================================================
// Integration Tests
// =============================================================================

mod integration {
    use tiexiu::Result;
    use tiexiu::api::parse_grammar;
    use tiexiu::cfg::constants::PATH_TATSU_GRAMMAR_EBNF;

    #[test]
    fn complex_grammar() -> Result<()> {
        // TODO: cause of failure - verify complex bootstrap parsing
        let grammar = r#"
            @@grammar :: Complex
            @@whitespace :: /\s+/

            start: expression

            expression:
                | term ('+' term)*
                | term ('-' term)*

            term:
                | factor ('*' factor)*
                | factor ('/' factor)*

            factor:
                | NUMBER
                | '(' expression ')'

            NUMBER: /\d+/
        "#;
        let tree = parse_grammar(grammar, &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    #[ignore = "TatSu grammar parsing is still work-in-progress"]
    fn tatsu_own_grammar() -> Result<()> {
        let tatsu_grammar = std::fs::read_to_string(PATH_TATSU_GRAMMAR_EBNF)?;

        let tree = parse_grammar(&tatsu_grammar, &[])?;
        let json = tree.to_model_json_string()?;

        assert!(json.contains("Grammar"));
        assert!(json.contains("rules"));
        Ok(())
    }
}

// =============================================================================
// Compilation Tests
// =============================================================================

mod compilation {
    use tiexiu::Result;
    use tiexiu::api::parse_input;

    #[test]
    #[ignore = "Compiled grammar parsing is still work-in-progress"]
    fn compiled_grammar_parses_input() -> Result<()> {
        let grammar = tiexiu::api::compile(
            r#"
            @@grammar :: Test
            start: 'hello' 'world'
        "#,
            &[],
        )?;

        let tree = parse_input(&grammar, "hello world", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }
}

// =============================================================================
// Round-Trip Tests
// =============================================================================

mod round_trips {
    use tiexiu::api::*;
    use tiexiu::peg::grammar::PrettyPrint;

    #[test]
    fn pretty_print_roundtrip() -> Result<()> {
        // TODO: cause of failure - verify round-trip stability
        let grammar_text = r#"
            @@grammar :: Pretty
            start: 'a' | 'b'
        "#;
        let tree = parse_grammar(grammar_text, &[])?;
        eprintln!("TREE\n{:#?}", tree);

        let grammar = compile(grammar_text, &[])?;
        eprintln!("COMPILED\n{}", grammar);

        let pretty = grammar.pretty_print();
        eprintln!("COMPILED->PRETTY\n{:#}", pretty);

        let grammar2 = compile(&pretty, &[])?;
        let pretty2 = grammar2.to_string();

        assert_eq!(
            pretty.trim(),
            pretty2.trim(),
            "Pretty-print round-trip failed:\nOriginal:\n{}\nPretty:\n{}",
            pretty,
            pretty2
        );
        Ok(())
    }
}
