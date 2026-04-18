// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::{KeyValue, Tree, TreeMap};
use serde_json::{Map, Value};

impl Tree {
    pub fn as_json_str(&self) -> Box<str> {
        format!("{:#?}", self.as_json()).into()
    }

    pub fn as_json(&self) -> Value {
        match self {
            Tree::Nil => Value::Null,
            Tree::Bottom => Value::Object({
                let mut m = Map::new();
                m.insert("Bottom".into(), Value::Null);
                m
            }),
            Tree::Text(t) => Value::String(t.to_string()),
            Tree::List(items) | Tree::Closed(items) => {
                Value::Array(items.iter().map(Tree::as_json).collect())
            }
            Tree::Map(m) => {
                let obj = map_to_json_obj(m);
                Value::Object(obj)
            }
            Tree::Node { typename, tree } => {
                let mut obj = Map::new();
                obj.insert(typename.to_string(), tree.as_json());
                Value::Object(obj)
            }
            Tree::Named(KeyValue(name, tree)) => {
                let mut obj = Map::new();
                obj.insert(name.to_string(), tree.as_json());
                Value::Object(obj)
            }
            Tree::NamedAsList(KeyValue(name, tree)) => {
                let mut obj = Map::new();
                obj.insert(name.to_string(), tree.as_json());
                Value::Object(obj)
            }
            Tree::Override(tree) | Tree::OverrideAsList(tree) => tree.as_json(),
        }
    }

    pub fn from_json(value: &Value) -> Option<Self> {
        match value {
            Value::Null => Some(Tree::Nil),
            Value::String(s) => Some(Tree::Text(s.clone().into())),
            Value::Array(arr) => {
                let items: Vec<Tree> = arr.iter().map(Tree::from_json).collect::<Option<_>>()?;
                Some(Tree::List(items.into()))
            }
            Value::Object(obj) => {
                if obj.len() == 1 {
                    let (key, value) = obj.iter().next()?;
                    if key == "Bottom" {
                        return Some(Tree::Bottom);
                    }
                    if key == "Named" || key == "NamedAsList" {
                        let tree = Tree::from_json(value)?;
                        let kv = KeyValue(key.clone().into(), tree.into());
                        return Some(Tree::Named(kv));
                    }
                    if let Some(tree) = Tree::from_json(value) {
                        return Some(Tree::Node {
                            typename: key.clone().into(),
                            tree: tree.into(),
                        });
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

fn map_to_json_obj(m: &TreeMap) -> Map<String, Value> {
    let mut obj = Map::new();
    for (k, v) in m.entries.iter() {
        obj.insert(k.to_string(), v.as_json());
    }
    obj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_json_roundtrip() {
        let cases: Vec<Tree> = vec![
            Tree::Nil,
            Tree::Bottom,
            Tree::Text("hello".into()),
            Tree::List(vec![Tree::Text("a".into()), Tree::Text("b".into())].into_boxed_slice()),
        ];

        for tree in cases {
            let json = tree.as_json();
            let round_tripped = Tree::from_json(&json);
            assert_eq!(round_tripped, Some(tree.clone()));
        }
    }
}
