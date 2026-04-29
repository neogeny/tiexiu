// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's parsing_test.py

use serde_json::json;
use tiexiu::api::compile;
use tiexiu::{Result, parse};

#[test]
#[ignore = "@@include will not be implemented"]
fn test_include() -> Result<()> {
    // WARNING
    //  Textual includes are a nightmare for bookkeeping and semantics.
    //  The only reasonable approach would be a use/import feature, but
    //  the use cases for that are currenly lacking.
    let grammar = r#"
        @@include :: "included.ebnf"
        start = item $ ;
        item = /\w+/ ;
    "#;

    let result = compile(grammar, &[]);
    assert!(
        result.is_err(),
        "Expected error for @@include, got success: {:?}",
        result
    );
    Ok(())
}

#[test]
#[ignore = "@@include will not be implemented"]
fn test_multiple_include() -> Result<()> {
    let grammar = r#"
        @@include :: "a.ebnf"
        @@include :: "b.ebnf"
        start = item $ ;
        item = /\w+/ ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
fn test_escape_sequences() -> Result<()> {
    let grammar = r#"
        start = 'hello\nworld' $ ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

// ============================================================================
// Start Rule Tests
// ============================================================================

#[test]
fn test_start() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test

        true = 'test' @:`True` $;
        false = 'test' @:`False` $;
    "#;

    let tree = parse(grammar, "test", &[])?;
    eprintln!("{:#?}", tree);
    assert_eq!(tree.to_json(), json!("True"));
    Ok(())
}

// ============================================================================
// Whitespace Tests
// ============================================================================

#[test]
fn test_skip_whitespace() -> Result<()> {
    let grammar = r#"
        statement = 'FOO' subject $ ;
        subject = name:id ;
        id = /[a-z]+/ ;
    "#;

    parse(grammar, "FOO something", &[])?;
    assert!(parse(grammar, "somethiing", &[]).is_err());
    assert!(parse(grammar, "FOO", &[]).is_err());
    Ok(())
}

#[test]
fn test_node_parseinfo() -> Result<()> {
    let grammar = r#"
        @@grammar::Test
        start = 'test' $ ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
#[ignore = "this test does nothing"]
fn test_parseinfo_directive() -> Result<()> {
    let grammar = r#"
        @@parseinfo :: True
        start = 'test' $ ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
#[ignore = "this test does nothing"]
fn test_parseinfo_false_directive() -> Result<()> {
    let grammar = r#"
        @@parseinfo :: False
        start = 'test' $ ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
#[ignore = "this test does nothing"]
fn test_cut_scope() -> Result<()> {
    let grammar = r#"
        start =
            | one
            | two
            ;

        one =
            | ~ !()
            | 'abc';

        two = `something` ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}
