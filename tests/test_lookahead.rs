// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for lookahead - uses compile() which has BUG

#[test]
#[ignore = "TODO: BUG - compile returns bootstrap instead of user grammar"]
fn test_skip_to() {
    let grammar = r#"
        start = 'x' ab $ ;
        ab = 'a' 'b' | -> 'b' ;
    "#;
    let _result = tiexiu::api::compile(grammar, &[]);
}
