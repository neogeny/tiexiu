// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::{KeyValue, Tree, TreeMap};
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

impl ToJson for TreeMap {
    fn to_json(&self) -> Json {
        let mut map = HashMap::new();
        for (name, tree) in &self.entries {
            map.insert(name.deref().into(), tree.to_json());
        }
        Json::Object(map)
    }
}

impl ToJson for Tree {
    fn to_json(&self) -> Json {
        match self {
            Tree::Nil | Tree::Bottom | Tree::Void => Json::Null,
            Tree::Text(s) => Json::String(s.deref().to_string()),
            Tree::List(v) => Json::Array(v.iter().map(|c| c.to_json()).collect()),
            Tree::Named(keyval) | Tree::NamedAsList(keyval) => {
                let KeyValue(name, tree) = keyval.deref();
                let mut map = HashMap::new();
                map.insert(name.to_string(), tree.to_json());
                Json::Object(map)
            }
            Tree::Override(tree) | Tree::OverrideAsList(tree) => tree.to_json(),
            Tree::Map(tags) => tags.to_json(),
            Tree::Node { info, tree } => {
                let params = Json::Array(
                    info.params
                        .iter()
                        .map(|c| Json::String(c.deref().into()))
                        .collect(),
                );
                let mut map: HashMap<String, Json> = HashMap::new();
                map.insert("name".into(), Json::String(info.name.to_string()));
                map.insert("params".into(), params);
                map.insert("tree".into(), tree.to_json());
                Json::Object(map)
            }
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

    #[test]
    fn test_cst_to_json_export() {
        // Create a simple Cst structure
        let token = Tree::Text(Box::from("hello"));
        let list = Tree::List(Box::new([token]));

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
