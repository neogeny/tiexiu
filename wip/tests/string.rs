// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/string_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse};

// ============================================================================
// Multiline String Tests
// ============================================================================

#[test]
fn test_multiline_string() {
    let grammar = r#"
        @@grammar :: Test

        start: longone | shortone $ ;

        shortone: "short"

        longone: """
            this "" \" \"""
            is a long "string"
            """
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}
