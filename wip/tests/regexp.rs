// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/regexp_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

// ============================================================================
// Regexp Utility Tests
// ============================================================================

#[test]
fn test_patterns_quotes() {
    assert_eq!(crate::util::regexpp("'"), r#"r"'"#);
    assert_eq!(crate::util::regexpp('\''), r#"r"'"#);
    assert_eq!(crate::util::regexpp("a'b"), r#"r"a'b""#);
    assert_eq!(crate::util::regexpp('"'), r#"r""""#);
    assert_eq!(crate::util::regexpp('"'), r#"r""""#);
}

#[test]
fn test_backslash_edge_cases() {
    assert_eq!(crate::util::regexpp(r"\'"), r#"r"\\'""#);
    assert_eq!(crate::util::regexpp(r"\\'"), r#"r"\\\\'""#);
}

#[test]
fn test_patterns_newlines() {
    assert_eq!(crate::util::regexpp("\n"), r"r'\n'");
    assert_eq!(crate::util::regexpp(r"\n"), r"r'\n'");
    assert_eq!(crate::util::regexpp(r"\\n"), r"r'\\n'");
}

#[test]
fn test_patterns_expr() {
    assert_eq!(crate::util::regexpp("[abc]"), r"r'[abc]'");
    assert!(crate::util::regexpp("\\").is_err());
}

#[test]
fn test_roundtrip_verification() {
    let result = crate::util::regexpp("it's");
    let evaled: String = result.and_then(|r| Ok(eval(&r))).unwrap();
    assert_eq!(evaled, "it's");
}
