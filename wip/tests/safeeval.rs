// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/safeeval_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

// ============================================================================
// SafeEval Utility Tests
// ============================================================================

use crate::util::safe_eval;

#[test]
fn test_simple_context_eval() {
    let context = std::collections::HashMap::from([("a", 1), ("b", 2)]);
    let expression = "seen: {a} {b}";
    let expected = "seen: 1 2";
    let result = safe_eval(expression, &context);
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_unauthorized_call() {
    let result = safe_eval(
        "open('file.txt')",
        &std::collections::HashMap::from([("x", 1)]),
    );
    assert!(result.is_err());
}

#[test]
fn test_dunder_block() {
    let result = safe_eval("x.__class__", &std::collections::HashMap::from([("x", 1)]));
    assert!(result.is_err());
}

#[test]
fn test_name_mismatch() {
    let mut context = std::collections::HashMap::new();
    context.insert(
        "abs",
        std::sync::Arc::new(|x: i64| x.abs()) as std::sync::Arc<dyn Fn(i64) -> i64>,
    );
    let result = safe_eval("fake_fn()", &context);
    assert!(result.is_err());
}

#[test]
fn test_bad_argcount() {
    let mut context = std::collections::HashMap::new();
    context.insert(
        "object",
        std::sync::Arc::new(()) as std::sync::Arc<dyn std::any::Any>,
    );
    context.insert(
        "type",
        std::sync::Arc::new(|_: &str, _: (), _: ()| ()) as std::sync::Arc<dyn Fn(&str, (), ())>,
    );
    let result = safe_eval("type('Evil', (object,), {})", &context);
    assert!(result.is_err());
}
