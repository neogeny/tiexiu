// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::{KeyValue, Tree, TreeMap, TreeRef};
use serde_json::{Map, Value};

use crate::json::error::JsonError;

impl Tree {
    /// Parses a JSON string into a `Tree`.
    pub fn from_json_str(json: &str) -> Result<Self, JsonError> {
        let value: Value = serde_json::from_str(json)?;
        let tree = Self::from_json(&value);
        Ok(tree)
    }

    /// Serializes the tree to a compact JSON string.
    pub fn to_json_str(&self) -> Box<str> {
        self.to_json_string_pretty().into()
    }

    /// Serializes the tree to a compact JSON string.
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self.to_json()).unwrap()
    }

    /// Serializes the tree to a pretty-printed JSON string.
    pub fn to_json_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.to_json()).unwrap()
    }

    /// Returns the tree as a `Value`.
    pub fn to_value(&self) -> Value {
        self.to_json()
    }

    /// Converts this tree into a `Value`.
    pub fn to_json(&self) -> Value {
        match self {
            Tree::Bottom | Tree::Nil => Value::Null,
            Tree::Text(t) => Value::String(t.to_string()),
            Tree::Seq(items) | Tree::Array(items) => {
                Value::Array(items.iter().map(|t| t.to_json()).collect())
            }
            Tree::Object(m) => {
                let mut obj = Map::new();
                for (k, v) in m.iter() {
                    obj.insert(k.clone(), v.to_json());
                }
                Value::Object(obj)
            }
            Tree::Node { typename, tree } => {
                let json_tree = tree.to_json();
                if let Value::Object(child_map) = json_tree {
                    let has_class = child_map.contains_key("__class__");
                    if !has_class {
                        let mut new_map = Map::new();
                        new_map.insert("__class__".into(), Value::String(typename.to_string()));
                        for (k, v) in child_map.iter() {
                            new_map.insert(k.clone(), v.clone());
                        }
                        return Value::Object(new_map);
                    }
                }
                let mut obj = Map::new();
                obj.insert("__class__".into(), Value::String(typename.to_string()));
                obj.insert("ast".into(), tree.to_json());
                Value::Object(obj)
            }

            Tree::Named(KeyValue(name, tree)) => {
                let mut obj = Map::new();
                obj.insert(name.to_string(), tree.to_json());
                Value::Object(obj)
            }
            Tree::NamedAsList(KeyValue(name, tree)) => {
                let mut obj = Map::new();
                obj.insert(name.to_string(), tree.to_json());
                Value::Object(obj)
            }
            Tree::Bool(b) => Value::Bool(*b),
            Tree::Number(n) => Value::Number(
                serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0)),
            ),
            Tree::Override(tree) | Tree::OverrideAsList(tree) => tree.to_json(),
        }
    }

    /// Converts a `Value` back into a `Tree`.
    pub fn from_json(value: &Value) -> Self {
        match value {
            Value::Null => Tree::Nil,
            Value::String(s) => Tree::Text(s.clone()),
            Value::Array(arr) => {
                let items: Vec<TreeRef> = arr.iter().map(|v| Tree::from_json(v).into()).collect();
                Tree::Seq(items)
            }
            Value::Object(obj) => {
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
                let mut m = TreeMap::default();
                for (key, value) in obj.iter() {
                    let tree = Tree::from_json(value);
                    m.insert(key.clone(), tree.into());
                }
                Tree::Object(m)
            }
            Value::Bool(b) => Tree::Bool(*b),
            Value::Number(n) => Tree::Number(n.as_f64().unwrap_or(0.0)),
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
            Tree::Seq(vec![
                Tree::Text("a".into()).into(),
                Tree::Text("b".into()).into(),
            ]),
        ];

        for tree in cases {
            let json = tree.to_json();
            let round_tripped = Tree::from_json(&json);
            assert_eq!(round_tripped, tree.clone());
        }
    }
}
