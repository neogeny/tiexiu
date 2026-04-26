// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::{KeyValue, Tree, TreeMap};
use serde_json::{Map, Value};

use serde::{Serialize, Serializer};

impl Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

impl Tree {
    pub fn to_json_str(&self) -> serde_json::Result<Box<str>> {
        self.to_string_pretty().map(|s| s.into_boxed_str())
    }

    pub fn to_json_string(&self) -> serde_json::Result<String> {
        self.to_string_pretty()
    }

    pub fn to_string_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.to_json())
    }

    pub fn to_json(&self) -> Value {
        self.to_value()
    }

    pub fn to_value(&self) -> Value {
        match self {
            Tree::Bottom | Tree::Nil => Value::Null,
            Tree::Text(t) => Value::String(t.to_string()),
            Tree::Seq(items) | Tree::Closed(items) => {
                Value::Array(items.iter().map(Tree::to_json).collect())
            }
            Tree::Map(m) => {
                let mut obj = Map::new();
                for (k, v) in m.iter() {
                    obj.insert(k.to_string(), v.to_json());
                }
                Value::Object(obj)
            }
            Tree::Node { typename, tree } => {
                let mut obj = Map::new();
                obj.insert("typename".into(), Value::String(typename.to_string()));
                obj.insert(typename.to_string(), tree.to_json());
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
            Tree::Override(tree) | Tree::OverrideAsList(tree) => tree.to_json(),
        }
    }

    pub fn from_json(value: &Value) -> Option<Self> {
        match value {
            Value::Null => Some(Tree::Nil),
            Value::String(s) => Some(Tree::Text(s.clone().into())),
            Value::Array(arr) => {
                let items: Vec<Tree> = arr.iter().map(Tree::from_json).collect::<Option<_>>()?;
                Some(Tree::Seq(items.into()))
            }
            Value::Object(obj) => {
                if obj.len() == 1 {
                    let (key, value) = obj.iter().next()?;
                    if key == "typename" {
                        if let Some(tree) = Tree::from_json(value) {
                            return Some(Tree::Node {
                                typename: key.clone().into(),
                                tree: tree.into(),
                            });
                        }
                    }
                }
                let mut m = TreeMap::new();
                for (key, value) in obj {
                    let tree = Tree::from_json(value)?;
                    m.insert(key, tree);
                }
                Some(Tree::Map(m.into()))
            }
            _ => None,
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
            Tree::Seq(
                vec![Tree::Text("a".into()), Tree::Text("b".into())]
                    .into_boxed_slice()
                    .into(),
            ),
        ];

        for tree in cases {
            let json = tree.to_json();
            let round_tripped = Tree::from_json(&json);
            assert_eq!(round_tripped, Some(tree.clone()));
        }
    }
}
