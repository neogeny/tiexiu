// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/safeeval_stress_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::util::{safe_eval, is_eval_safe, make_hashable};

#[test]
fn test_basic_arithmetic() {
    let result = safe_eval("2 + 2 * 10", &std::collections::HashMap::new()).unwrap();
    assert_eq!(result, 22);
}

#[test]
fn test_context_access() {
    let mut context = std::collections::HashMap::new();
    context.insert("a", 5);
    context.insert("b", 10);
    let result = safe_eval("a * b", &context).unwrap();
    assert_eq!(result, 50);
}

#[test]
fn test_is_eval_safe_boolean() {
    assert!(is_eval_safe("1 + 1", &std::collections::HashMap::new()));
    assert!(!is_eval_safe("import os", &std::collections::HashMap::new()));
}

#[test]
fn test_unauthorized_function_call() {
    let result = safe_eval("print('hello')", &std::collections::HashMap::new());
    assert!(result.is_err());
}

#[test]
fn test_dunder_blocking() {
    struct User {
        private_field: String,
    }
    let user = User { private_field: "hidden_value".to_string() };
    let mut context = std::collections::HashMap::new();
    context.insert("u", &user);
    let result = safe_eval("u.__private_field", &context);
    assert!(result.is_err());
}

#[test]
fn test_diamond_reference() {
    let shared_leaf = std::collections::HashMap::from([("key", "value")]);
    let mut branch_a = Vec::new();
    branch_a.push(std::rc::Rc::new(shared_leaf.clone()));
    let mut branch_b = Vec::new();
    branch_b.push(std::rc::Rc::new(shared_leaf));
    let context = std::collections::HashMap::from([
        ("branch_a", branch_a),
        ("branch_b", branch_b),
    ]);
    let result = make_hashable(&context);
    assert!(result.is_ok());
}

#[test]
fn test_recursion_cycle_detection() {
    let circular: std::cell::RefCell<Vec<std::cell::RefCell<Vec>>>> = std::cell::RefCell::new(vec![]);
    let result = make_hashable(&circular);
    assert!(result.is_err());
}

#[test]
fn test_complex_graph_cycle() {
    let mut a = std::collections::HashMap::new();
    let mut b = std::collections::HashMap::new();
    let mut c = std::collections::HashMap::new();
    b.insert("next", std::rc::Rc::new(c));
    a.insert("next", std::rc::Rc::new(b));
    let context = std::collections::HashMap::from([
        ("start", std::rc::Rc::new(a)),
    ]);
    let result = make_hashable(&context);
    assert!(result.is_err());
}