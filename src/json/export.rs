// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::contexts::ast::Ast;
use crate::contexts::Cst;
use std::collections::HashMap;
use std::ops::Deref;

#[cfg(feature = "serde_json")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde_json")]
use serde_json::Value;


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde_json", serde(untagged))]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

pub trait ToJson {
    fn to_json(&self) -> Json;
}

impl ToJson for Ast {
    fn to_json(&self) -> Json {
        let mut map = HashMap::new();
        for (name, cst) in &self.fields {
            map.insert(name.clone(), cst.to_json());
        }
        Json::Object(map)
    }
}

impl ToJson for Cst {
    fn to_json(&self) -> Json {
        match self {
            Cst::Nil | Cst::Bottom | Cst::Void => Json::Null,
            Cst::Token(s) | Cst::Literal(s) => Json::String(s.deref().to_string()),
            Cst::Number(n) => Json::Number(*n),
            Cst::List(v) | Cst::Closure(v) => {
                Json::Array(v.iter().map(|c| c.to_json()).collect())
            }
            Cst::Named(keyval) | Cst::NamedList(keyval) => {
                let (name, cst) = keyval.deref();
                let mut map = HashMap::new();
                map.insert(name.to_string(), cst.to_json());
                Json::Object(map)
            }
            Cst::OverrideValue(cst) | Cst::OverrideList(cst) => cst.to_json(),
            Cst::Ast(ast) => ast.to_json(),
        }
    }
}

impl Json {
    pub fn to_serde(&self) -> Option<Value> {
        #[cfg(not(feature = "serde_json"))]
        {
            None
        }

        #[cfg(feature = "serde_json")]
        {
            serde_json::to_value(self).ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contexts::cst::Cst;

    #[test]
    fn test_cst_to_json_export() {
        // Create a simple Cst structure
        let token = Cst::Token(Box::from("hello"));
        let list = Cst::List(Box::new([token]));

        // 1. Test Internal Json Conversion
        let json_node = list.to_json();
        if let Json::Array(items) = &json_node {
            assert_eq!(items.len(), 1);
            if let Json::String(s) = &items[0] {
                assert_eq!(s, "hello");
            } else {
                panic!("Expected Json::String");
            }
        } else {
            panic!("Expected Json::Array");
        }

        // 2. Test Serde Integration (if feature is on)
        #[cfg(all(feature = "serde", feature = "serde_json"))]
        {
            let serde_val = json_node.to_serde().expect("Conversion failed");
            assert!(serde_val.is_array());
            assert_eq!(serde_val[0], "hello");
        }
    }
}