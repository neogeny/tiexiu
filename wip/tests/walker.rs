// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's walker_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::asmodel;
use crate::trees::Tree;
use crate::walker::{BreadthFirstWalker, DepthFirstWalker, NodeWalker};
use std::collections::HashMap;

#[test]
fn test_walk_node_ast() {
    let grammar = r#"
        @@grammar::TLA

        start = expression $;

        expression = temporal_expression | nontemporal_top_expression ;
        nontemporal_top_expression(SampleExpression) =  expr:nontemporal_expression ;
        temporal_expression =    temporal_atom;
        temporal_atom(TemporalSeq) = ['Seq'] '(' @:temporal_arg_list ')';
        temporal_arg_list = "," .{@+:expression}+;
        nontemporal_expression(Number) =  number ;

        number::int = /\d+/;
    "#;

    let parser = asmodel(grammar).expect("Failed to compile");
    let model = parser.parse("Seq(1,1)").expect("Failed to parse");

    struct PW;
    impl DepthFirstWalker for PW {
        fn walk_node(&mut self, node: &Tree) {
            let mut seen = std::cell::RefCell::new(HashMap::new());
            let name = format!("{:?}", node);
            *seen.borrow_mut().entry(name).or_insert(0) += 1;
        }
    }

    let mut walker = DepthFirstWalker::new();
    walker.walk(&model);

    let mut breadth = BreadthFirstWalker::new();
    let result: Vec<String> = breadth
        .walk(&model)
        .iter()
        .map(|n| format!("{:?}", n))
        .collect();
    assert!(result.len() > 0);
}

#[test]
fn test_cache_per_class() {
    let walker_cache = NodeWalker::walker_cache();
    let df_cache = DepthFirstWalker::walker_cache();

    assert!(!walker_cache.is_empty());
    assert!(!df_cache.is_empty());
}
