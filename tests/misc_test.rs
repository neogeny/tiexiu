// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Misc tests - translated from TatSu's asjson_test.py test_mapping_key_conversion

use tiexiu::Result;
use tiexiu::api::parse;

#[test]
fn test_mapping() -> Result<()> {
    // Python test: asjson({123: "integer_key", (1,2): "tuple_key"})
    // Verifies JSON keys are converted to strings
    // In Rust/TieXiu, named elements in the AST produce string keys in JSON
    let grammar = r#"
        @@whitespace :: /\s+/
        start = key:key value:value ;
        key = /\w+/ ;
        value = /\w+/ ;
    "#;
    let ast = parse(grammar, "foo bar", &[])?;
    let json = ast.to_json();
    // Verify JSON has string keys and correct values
    assert!(json.is_object(), "Expected JSON object, got: {:?}", json);
    assert_eq!(json["key"], "foo");
    assert_eq!(json["value"], "bar");
    Ok(())
}
