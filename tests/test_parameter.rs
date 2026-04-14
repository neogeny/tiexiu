// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for parameters - uses compile() which has BUG

#[test]
#[ignore = "TODO: BUG - compile returns bootstrap, parameterized rules not implemented"]
fn test_keyword_params() {
    let grammar = r#"
        start = rule[param] ;
        rule[:param] = 'test' ;
    "#;
    let _result = tiexiu::api::compile(grammar, &[]);
}
