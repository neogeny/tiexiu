// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for pattern - uses compile() which has BUG

#[test]
#[ignore = "TODO: BUG - compile returns bootstrap instead of user grammar"]
fn test_patterns_with_newlines() {
    let grammar = r#"
        @@whitespace :: /[ \t]/
        start = /\w+/ $ ;
    "#;
    let _result = tiexiu::api::compile(grammar, &[]);
}
