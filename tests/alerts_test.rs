// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/alerts_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use serde_json::json;
use tiexiu::Result;
use tiexiu::api::parse;

#[test]
#[ignore = "TODO: interpolation not yet implemented"]
fn test_alert_interpolation() -> Result<()> {
    let grammar = r#"
        start = a:number b: number i:^`seen: {a}, {b}` $ ;
        number = /\d+/ ;
    "#;

    let ast = parse(grammar, "42 69", &[])?;
    eprintln!("{:#?}", ast);
    assert_eq!(
        ast.to_json(),
        json!(
            {"a":"42", "b":"69", "i": "seen: 42, 69"}
        )
    );
    Ok(())
}
