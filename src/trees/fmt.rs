// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::cst::TreeMap;
use crate::trees::{KeyValue, Tree, TreeRef};
use std::fmt;

pub fn fmt_treemap(map: &TreeMap, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "m(&[")?;
    for (i, key) in map.keys().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "({:?}, {})", key, map.get(key).unwrap())?;
    }
    write!(f, "])")
}

impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {}", self.0, self.1)
    }
}

fn fmt_items(items: &[TreeRef]) -> String {
    items
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(s) => write!(f, "t({:?})", s),
            Self::Named(kv) => write!(f, "k({})", kv),
            Self::NamedAsList(kv) => write!(f, "kl({})", kv),
            Self::Override(v) => write!(f, "o({})", v),
            Self::OverrideAsList(v) => write!(f, "ol({})", v),
            Self::Map(map) => fmt_treemap(map, f),
            Self::Null => write!(f, "NIL"),
            Self::Bottom => write!(f, "BOTTOM"),
            Self::Seq(items) => write!(f, "s(&[{}])", fmt_items(items)),
            Self::List(items) => write!(f, "c(&[{}])", fmt_items(items)),
            Self::Node { typename, tree } => {
                write!(f, "n({}, {})", typename, tree)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::{KeyValue, Tree, TreeMap};

    #[test]
    fn test_treemap_display() {
        let mut map = TreeMap::default();
        map.insert("key1".into(), Tree::Text("value1".into()));
        map.insert("key2".into(), Tree::Text("value2".into()));
        assert_eq!(
            Tree::Map(map).to_string(),
            "m(&[(\"key1\", t(\"value1\")), (\"key2\", t(\"value2\"))])"
        );

        let empty_map = TreeMap::default();
        assert_eq!(Tree::Map(empty_map).to_string(), "m(&[])");
    }

    #[test]
    fn test_key_value_display() {
        let kv = KeyValue("name".into(), Tree::Text("value".into()).into());
        assert_eq!(kv.to_string(), "\"name\", t(\"value\")");
    }

    #[test]
    fn test_tree_display() {
        assert_eq!(Tree::Text("hello".into()).to_string(), "t(\"hello\")");

        let kv_leaf = KeyValue("tag".into(), Tree::Text("leaf_val".into()).into());
        assert_eq!(
            Tree::Named(kv_leaf).to_string(),
            "k(\"tag\", t(\"leaf_val\"))"
        );

        let kv_node = KeyValue("node".into(), Tree::Text("node_val".into()).into());
        assert_eq!(
            Tree::NamedAsList(kv_node).to_string(),
            "kl(\"node\", t(\"node_val\"))"
        );

        assert_eq!(
            Tree::Override(Tree::Text("root".into()).into()).to_string(),
            "o(t(\"root\"))"
        );

        assert_eq!(
            Tree::OverrideAsList(Tree::Seq(vec![Tree::Text("item".into()).into()]).into())
                .to_string(),
            "ol(s(&[t(\"item\")]))"
        );

        let mut map = TreeMap::default();
        map.insert("a".into(), Tree::Text("1".into()));
        assert_eq!(Tree::Map(map).to_string(), "m(&[(\"a\", t(\"1\"))])");

        assert_eq!(Tree::Null.to_string(), "NIL");

        assert_eq!(Tree::Bottom.to_string(), "BOTTOM");

        let node = Tree::Seq(vec![
            Tree::Text("a".into()).into(),
            Tree::Text("b".into()).into(),
            Tree::Seq(vec![Tree::Text("c".into()).into()]).into(),
        ]);
        assert_eq!(node.to_string(), "s(&[t(\"a\"), t(\"b\"), s(&[t(\"c\")])])");

        let pruned_tree = Tree::Node {
            typename: "MyRule".into(),
            tree: Tree::Text("pruned_content".into()).into(),
        };
        assert_eq!(pruned_tree.to_string(), "n(MyRule, t(\"pruned_content\"))");
    }
}
