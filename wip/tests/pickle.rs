// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's pickle_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

// ============================================================================
// Pickle Tests
// ============================================================================

#[test]
fn test_synth_model() {
    let grammar = r#"
        start::ASeq
            =
            aseq
            $
            ;

        aseq
            =
            {'a'}+
            ;
    "#;

    let m = compile(grammar, "ASeq");
    let model = m.parse("a a a");
    assert!(matches!(model, Tree::Node(..)));
    let type_name = format!("{:?}", model);
    assert!(type_name.contains("ASeq"));
}

#[test]
fn test_nested_class_synth_model() {
    let grammar = r#"
        start::ASeq
            =
            seqs:aseq
            $
            ;

        aseq::Seq
            =
            values:{'a'}+
            ;
    "#;

    let m = compile(grammar, "ASeq");
    let model = m.parse("a a a");
    assert!(matches!(model, Tree::Node(..)));
}
