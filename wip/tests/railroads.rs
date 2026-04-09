// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's railroads_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;
use crate::diagram::railroads;

#[test]
fn test_railroads() {
    let grammar = std::fs::read_to_string("./grammar/tatsu.tatsu").unwrap_or_default();
    let model = compile(&grammar).expect("Failed to compile");
    railroads::draw(&model);

    let tracks = railroads::tracks(&model);
    let track0 = "start ●─grammar─■";
    assert_eq!(tracks.get(0), Some(&track0.to_string()));
}

#[test]
fn test_per_node() {
    let grammar = r#"
        start: options

        options: number | 'hello'

        number: /\d+/
    "#;
    let model = compile(grammar).expect("Failed to compile");

    let optrule = model.rulemap.get("options").expect("Rule not found");
    let expected = r#"
         options ●───┬─number──┬──■
                     └─'hello'─┘
    "#;
    assert_eq!(optrule.railroads().trim(), expected.trim());
}
