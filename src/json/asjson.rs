// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::{KeyValue, Tree, TreeMap};
use json::JsonValue;
use std::rc::Rc;

use crate::json::error::JsonError;

#[cfg(feature = "serde_json")]
use serde::Serialize;

#[cfg(feature = "serde_json")]
impl Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json_value = self.to_json();
        let json_str = json_value.dump();
        let serde_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        serde_value.serialize(serializer)
    }
}

impl Tree {
    pub fn from_json_str(json: &str) -> Result<Self, JsonError> {
        let value = json::parse(json)?;
        let tree = Self::from_json(&value);
        Ok(tree)
    }

    pub fn to_json_str(&self) -> Box<str> {
        self.to_json_string_pretty().into()
    }

    pub fn to_json_string(&self) -> String {
        self.to_json().dump()
    }

    pub fn to_json_string_pretty(&self) -> String {
        self.to_json().pretty(2)
    }

    pub fn to_value(&self) -> JsonValue {
        self.to_json()
    }

    pub fn to_json(&self) -> JsonValue {
        match self {
            Tree::Bottom | Tree::Nil => JsonValue::Null,
            Tree::Text(t) => JsonValue::String(t.to_string()),
            Tree::Seq(items) | Tree::List(items) => {
                JsonValue::Array(items.iter().map(|t| t.to_json()).collect())
            }
            Tree::Map(m) => {
                let mut obj = JsonValue::new_object();
                for (k, v) in m.iter() {
                    obj[&**k] = v.to_json();
                }
                obj
            }
            Tree::Node { typename, tree } => {
                let json_tree = tree.to_json();
                if let JsonValue::Object(child_map) = json_tree {
                    let has_class = child_map.iter().any(|(k, _)| k == "__class__");
                    if !has_class {
                        let mut new_map = JsonValue::new_object();
                        new_map["__class__"] = JsonValue::String(typename.to_string());
                        for (k, v) in child_map.iter() {
                            new_map[k] = v.clone();
                        }
                        new_map["__class__"] = JsonValue::String(typename.to_string());
                        return new_map;
                    }
                }
                let mut obj = JsonValue::new_object();
                obj["__class__"] = JsonValue::String(typename.to_string());
                obj["ast"] = tree.to_json();
                obj
            }

            Tree::Named(KeyValue(name, tree)) => {
                let mut obj = JsonValue::new_object();
                obj[name.to_string()] = tree.to_json();
                obj
            }
            Tree::NamedAsList(KeyValue(name, tree)) => {
                let mut obj = JsonValue::new_object();
                obj[name.to_string()] = tree.to_json();
                obj
            }
            Tree::Override(tree) | Tree::OverrideAsList(tree) => tree.to_json(),
        }
    }

    pub fn from_json(value: &JsonValue) -> Self {
        match value {
            JsonValue::Null => Tree::Nil,
            JsonValue::String(s) => Tree::Text(s.clone().into()),
            JsonValue::Short(s) => Tree::Text(s.to_string().into()),
            JsonValue::Array(arr) => {
                let items: Vec<Rc<Tree>> = arr.iter().map(|v| Tree::from_json(v).into()).collect();
                Tree::Seq(items.into())
            }
            JsonValue::Object(obj) => {
                if obj.len() == 1
                    && let Some((key, value)) = obj.iter().next()
                    && key == "typename"
                {
                    let tree = Tree::from_json(value);
                    return Tree::Node {
                        typename: key.into(),
                        tree: tree.into(),
                    };
                }
                let mut m = TreeMap::new();
                for (key, value) in obj.iter() {
                    let tree = Tree::from_json(value);
                    m.insert(key, tree);
                }
                Tree::Map(m.into())
            }
            JsonValue::Boolean(yesno) => Tree::text(yesno.to_string().as_str().into()).clone(),
            JsonValue::Number(n) => Tree::text(n.to_string().as_str().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_json_roundtrip() {
        let cases: Vec<Tree> = vec![
            Tree::Nil,
            Tree::Text("hello".into()),
            Tree::Seq(vec![Tree::Text("a".into()).into(), Tree::Text("b".into()).into()].into()),
        ];

        for tree in cases {
            let json = tree.to_json();
            let round_tripped = Tree::from_json(&json);
            assert_eq!(round_tripped, tree.clone());
        }
    }
}
