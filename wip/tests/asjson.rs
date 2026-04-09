// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/asjson_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::util::asjson;

#[test]
fn test_primitives() {
    assert_eq!(asjson(&1).unwrap(), serde_json::json!(1));
    assert_eq!(asjson(&"test").unwrap(), serde_json::json!("test"));
    assert_eq!(asjson(&true).unwrap(), serde_json::json!(true));
    assert_eq!(asjson(&()).unwrap(), serde_json::json!(null));
}

#[test]
fn test_containers() {
    use std::collections::{HashMap, HashSet};
    
    let data: HashMap<String, Vec<i32>> = HashMap::from([
        ("a".to_string(), vec![1, 2]),
    ]);
    let result = asjson(&data).unwrap();
    assert_eq!(result["a"], serde_json::json!([1, 2]));
    
    let tuple = (3, 4);
    let result = asjson(&tuple).unwrap();
    assert_eq!(result, serde_json::json!([3, 4]));
    
    let set: HashSet<i32> = HashSet::from([5, 6]);
    let result = asjson(&set).unwrap();
    assert!(result.as_array().unwrap().contains(&serde_json::json!(5)));
    assert!(result.as_array().unwrap().contains(&serde_json::json!(6)));
}

#[test]
fn test_enum_handling() {
    #[derive(Debug)]
    enum Color {
        RED = 1,
        GREEN = 2,
    }
    
    let result = asjson(&Color::RED).unwrap();
    assert_eq!(result, serde_json::json!(1));
}

#[test]
fn test_namedtuple_handling() {
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 10, y: 20 };
    let result = asjson(&p).unwrap();
    assert_eq!(result["x"], serde_json::json!(10));
    assert_eq!(result["y"], serde_json::json!(20));
}

#[test]
fn test_circular_reference() {
    use std::cell::RefCell;
    use std::rc::Rc;
    
    let node: Rc<RefCell<Option<Rc<RefCell<Option<Rc<RefCell<Option>>>>>>>>> = Rc::new(RefCell::new(None));
    let result = asjson(&node);
    assert!(result.is_err() || result.unwrap().is_null());
}

#[test]
fn test_custom_json_protocol_with_cycle() {
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct CustomNode {
        name: String,
        #[serde(skip)]
        child: Option<Rc<CustomNode>>,
    }
    
    let parent = Rc::new(CustomNode { name: "parent".to_string(), child: None });
    let child = Rc::new(CustomNode { name: "child".to_string(), child: Some(Rc::clone(&parent)) });
    Rc::get_mut(&mut *Rc::unwrap_or_clone(parent)).map(|p| p.child = Some(Rc::clone(&child)));
    
    let result = asjson(&*parent).unwrap();
    assert_eq!(result["name"], serde_json::json!("parent"));
}

#[test]
fn test_mapping_key_conversion() {
    use std::collections::HashMap;
    
    let mut data: HashMap<serde_json::Value, String> = HashMap::new();
    data.insert(serde_json::json!(123), "integer_key".to_string());
    data.insert(serde_json::json!([1, 2]), "tuple_key".to_string());
    
    let result = asjson(&data).unwrap();
    assert_eq!(result["123"], serde_json::json!("integer_key"));
    assert_eq!(result["[1, 2]"], serde_json::json!("tuple_key"));
}