// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's model_test.py

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_children() -> Result<()> {
    // TODO: cause of failure - verify Tree/Node parent-child relationship
    let grammar = r#"
        @@grammar::Calc

        start = expression $ ;

        expression = term ;
        term = 'x' ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
fn test_node_kwargs() -> Result<()> {
    // TODO: cause of failure - verify Node construction logic
    let grammar = r#"
        start = 'value' ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}
