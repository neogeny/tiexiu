// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's parsing_test.py

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
#[ignore = "@@include not yet implemented"]
fn test_include() -> Result<()> {
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
#[ignore = "@@include not yet implemented"]
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

    compile(grammar, &[])?;
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

    compile(grammar, &[])?;
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
fn test_parseinfo_directive() -> Result<()> {
    let grammar = r#"
        @@parseinfo :: True
        start = 'test' $ ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
fn test_parseinfo_false_directive() -> Result<()> {
    let grammar = r#"
        @@parseinfo :: False
        start = 'test' $ ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
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
