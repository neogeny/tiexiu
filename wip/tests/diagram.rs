// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's diagram_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;
use crate::diagram;

#[test]
fn test_dot() {
    let grammar = r#"
        start = "foo\nbar" $;
    "#;
    let model = compile(grammar, "Diagram").expect("Failed to compile");
    if diagram::available() {
        diagram::draw("tmp/diagram.svg", &model);
    }
}
