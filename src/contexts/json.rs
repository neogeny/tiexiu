// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ast::Ast;
use super::cst::Cst;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl Ast {
    pub fn to_json(&self) -> Json {
        let mut map = std::collections::HashMap::new();
        for (name, cst) in &self.fields {
            map.insert(name.clone(), cst.to_json());
        }
        Json::Object(map)
    }
}

impl Cst {
    pub fn to_json(&self) -> Json {
        match self {
            Cst::Nil | Cst::Bottom | Cst::Void => Json::Null,
            Cst::Token(s) | Cst::Literal(s) => Json::String(s.deref().to_string()),
            Cst::List(v) | Cst::Closure(v) => Json::Array(v.iter().map(|c| c.to_json()).collect()),
            Cst::Named(keyval) | Cst::NamedList(keyval) => {
                let (name, cst) = keyval.deref();
                let mut map: HashMap<String, Json> = std::collections::HashMap::new();
                map.insert(name.to_string(), cst.to_json());
                Json::Object(map)
            }
            Cst::Number(n) => Json::Number(*n),
            Cst::OverrideValue(cst) | Cst::OverrideList(cst) => cst.to_json(),
            Cst::Ast(ast) => ast.to_json(),
        }
    }
}

#[cfg(not(feature = "serde_json"))]
type Value = ();

impl Json {
    pub fn to_serde(&self) -> Option<Value> {
        #[cfg(not(feature = "serde_json"))]
        {
            #[allow(clippy::needless_return)]
            return None;
        }
        #[cfg(feature = "serde_json")]
        {
            use serde_json::{Map, Value}; // Local, gated imports

            fn convert(json: &Json) -> Value {
                match json {
                    Json::Null => Value::Null,
                    Json::Bool(b) => Value::Bool(*b),
                    Json::Number(n) => {
                        // Handle potential non-finite numbers if necessary
                        serde_json::Number::from_f64(*n)
                            .map(Value::Number)
                            .unwrap_or(Value::Null)
                    }
                    Json::String(s) => Value::String(s.clone()),
                    Json::Array(arr) => Value::Array(arr.iter().map(convert).collect()),
                    Json::Object(obj) => {
                        let mut map = Map::new();
                        for (k, v) in obj {
                            map.insert(k.clone(), convert(v));
                        }
                        Value::Object(map)
                    }
                }
            }
            Some(convert(self))
        }
    }
}
