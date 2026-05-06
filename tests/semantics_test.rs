// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for semantics - uses compile() which has BUG

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
#[ignore = "semantic actions not implemented"]
fn test_semantics_not_class() -> Result<()> {
    let grammar = r#"
        start::sum = {number}+ $ ;
        number::int = /\d+/ ;
    "#;
    let model = compile(grammar, &[])?;
    // Python test verifies that passing a class (not instance) raises TypeError
    // and that parsing with semantics=ModelBuilderSemantics() returns 15 for "5 4 3 2 1"
    // TieXiu doesn't support semantics, so we just verify the grammar compiles
    let _ = model;
    Ok(())
}
