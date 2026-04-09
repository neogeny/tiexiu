// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/typetools_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::util::cast;

#[test]
fn test_cast_success() {
    assert_eq!(cast::<i32>(42), Ok(42));
    assert_eq!(cast::<String>("hello".to_string()), Ok("hello".to_string()));
    assert_eq!(cast::<Vec<i32>>(vec![1, 2, 3]), Ok(vec![1, 2, 3]));
    assert_eq!(
        cast::<std::collections::HashMap<String, i32>>(std::collections::HashMap::from([(
            "a".to_string(),
            1
        )])),
        Ok(std::collections::HashMap::from([("a".to_string(), 1)]))
    );
}

#[test]
fn test_cast_failure() {
    assert!(cast::<i32>("not an int".to_string()).is_err());
    assert!(cast::<Vec<i32>>(std::collections::HashMap::new()).is_err());
}

#[test]
fn test_cast_union_success() {
    let result: Result<i32, _> = cast(42);
    assert!(result.is_ok());
    let result: Result<String, _> = cast("hello".to_string());
    assert!(result.is_ok());
    let result: Result<Option<f64>, _> = cast(None::<Option<f64>>);
    assert!(result.is_ok());
}

#[test]
fn test_cast_union_failure() {
    let result: Result<i32, _> = cast(1.5_f64);
    assert!(result.is_err());
}
