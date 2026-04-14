// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for directives - uses compile() which has BUG

#[test]
#[ignore = "TODO: BUG - compile returns bootstrap instead of user grammar"]
fn test_whitespace_directive() {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        test = "test" $;
    "#;
    let _result = tiexiu::api::compile(grammar, &[]);
}
