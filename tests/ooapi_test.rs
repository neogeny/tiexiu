// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! OO API tests for TieXiu

use tiexiu::Result;
use tiexiu::api::ooapi::TieXiu;

#[test]
fn test_default_creates_empty_instance() {
    let tx = TieXiu::default();
    assert!(tx.get("start = 'a'").is_none());
}

#[test]
fn test_get_returns_none_for_uncached() {
    let tx = TieXiu::new(&[]);
    assert!(tx.get("start = 'a'").is_none());
}

#[test]
fn test_compile_caches_and_get_retrieves() -> Result<()> {
    let tx = TieXiu::new(&[]);
    let grammar_text = "start = 'hello'";
    let g1 = tx.compile(grammar_text)?;
    assert_eq!(g1.rules().count(), 1);

    // Should be cached now
    let cached = tx.get(grammar_text);
    assert!(cached.is_some());
    let g2 = cached.unwrap();
    assert_eq!(g2.rules().count(), 1);
    Ok(())
}

#[test]
fn test_get_or_compile_returns_same_grammar() -> Result<()> {
    let tx = TieXiu::new(&[]);
    let grammar_text = "start = 'x' | 'y'";
    let g1 = tx.get_or_compile(grammar_text)?;
    let g2 = tx.get_or_compile(grammar_text)?;
    assert_eq!(g1.rules().count(), g2.rules().count());
    Ok(())
}

#[test]
fn test_get_or_compile_fails_on_empty() {
    let tx = TieXiu::new(&[]);
    assert!(tx.get_or_compile("").is_err());
}

#[test]
fn test_parse_directly() -> Result<()> {
    let tx = TieXiu::new(&[]);
    let tree = tx.parse("start = /a+/", "aaa")?;
    let json = tree.to_json_string();
    assert!(json.contains("a"));
    Ok(())
}

#[test]
fn test_parse_input_with_compiled_grammar() -> Result<()> {
    let tx = TieXiu::new(&[]);
    let grammar = tx.compile("start = 'x' 'y'")?;
    let tree = tx.parse_input(&grammar, "xy")?;
    let json = tree.to_json_string();
    assert!(json.contains("x") && json.contains("y"));
    Ok(())
}

#[test]
fn test_update_cfg() -> Result<()> {
    let mut tx = TieXiu::new(&[]);
    tx.update_cfg(&[]);
    let tree = tx.parse("start = 'a'", "a")?;
    assert_eq!(tree.to_json_string(), "\"a\"");
    Ok(())
}

#[test]
fn test_multiple_grammars_cached_independently() -> Result<()> {
    let tx = TieXiu::new(&[]);
    let g1 = tx.compile("start = 'a'")?;
    let g2 = tx.compile("start = 'b'")?;
    assert_eq!(g1.rules().count(), 1);
    assert_eq!(g2.rules().count(), 1);
    // Both should be independently cached
    assert!(tx.get("start = 'a'").is_some());
    assert!(tx.get("start = 'b'").is_some());
    Ok(())
}

#[test]
fn test_grammar_pretty() -> Result<()> {
    let tx = TieXiu::new(&[]);
    let pretty = tx.grammar_pretty("start = 'a'")?;
    assert!(pretty.contains("start"));
    Ok(())
}

#[test]
fn test_boot_grammar() -> Result<()> {
    let tx = TieXiu::new(&[]);
    let boot = tx.boot_grammar()?;
    assert!(boot.rules().count() > 0);
    Ok(())
}
