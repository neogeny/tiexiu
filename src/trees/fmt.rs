// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::{KeyValue, Tree, TreeMap};
use std::fmt;
use std::ops::Deref;

impl fmt::Display for TreeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Collect and sort keys for a stable, predictable string
        let mut keys: Vec<&str> = self.entries.keys().map(|k| k.deref()).collect();
        keys.sort();

        write!(f, "{{")?;
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            // Safe to unwrap because we just got the key from the map
            write!(f, "{}: {}", key, self.entries.get(*key).unwrap())?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "«{}={}»", self.0, self.1)
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{}", s),
            Self::Named(kv) | Self::NamedAsList(kv) => write!(f, "{}", kv),
            Self::Override(v) => write!(f, "!{}", v),
            Self::OverrideAsList(v) => write!(f, "!!{}", v),
            Self::Map(tags) => write!(f, "{}", tags),
            Self::Nil => write!(f, "∅"),
            Self::Bottom => write!(f, "⊥"),
            Self::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Self::Node { meta, tree } => {
                let params = meta.params.join(", ");
                write!(f, "{}[{}]: {}", meta.name, params, tree)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::{FlagMap, KeyValue, MapEntries, NodeMeta, Tree, TreeMap};
    use indexmap::IndexMap;
    use std::rc::Rc;

    #[test]
    fn test_tree_tags_display() {
        let mut tags_map = MapEntries::new();
        tags_map.insert("key1".into(), Tree::Text("value1".into()));
        tags_map.insert("key2".into(), Tree::Text("value2".into()));
        let tags = TreeMap { entries: tags_map };
        assert_eq!(tags.to_string(), "{key1: value1, key2: value2}");

        let tags_empty = TreeMap {
            entries: IndexMap::new(),
        };
        assert_eq!(tags_empty.to_string(), "{}");
    }

    #[test]
    fn test_key_value_display() {
        let kv = KeyValue("name".into(), Tree::Text("value".into()));
        assert_eq!(kv.to_string(), "«name=value»");
    }

    #[test]
    fn test_tree_display() {
        // Leaf
        assert_eq!(Tree::Text("hello".into()).to_string(), "hello");

        // LeafTag
        let kv_leaf = KeyValue("tag".into(), Tree::Text("leaf_val".into()));
        assert_eq!(Tree::Named(kv_leaf.into()).to_string(), "«tag=leaf_val»");

        // NodeTag
        let kv_node = KeyValue("node".into(), Tree::Text("node_val".into()));
        assert_eq!(
            Tree::NamedAsList(kv_node.into()).to_string(),
            "«node=node_val»"
        );

        // RootLeaf
        assert_eq!(
            Tree::Override(Tree::Text("root".into()).into()).to_string(),
            "!root"
        );

        // RootNode
        assert_eq!(
            Tree::OverrideAsList(Tree::List(vec![Tree::Text("item".into())].into()).into())
                .to_string(),
            "!![item]"
        );

        // Tags
        let mut tags_map = MapEntries::new();
        tags_map.insert("a".into(), Tree::Text("1".into()));
        let tags = TreeMap { entries: tags_map };
        assert_eq!(Tree::Map(tags.into()).to_string(), "{a: 1}");

        // Nil
        assert_eq!(Tree::Nil.to_string(), "∅");

        // Bottom
        assert_eq!(Tree::Bottom.to_string(), "⊥");

        // Node
        let node = Tree::List(
            vec![
                Tree::Text("a".into()),
                Tree::Text("b".into()),
                Tree::List(vec![Tree::Text("c".into())].into()),
            ]
            .into(),
        );
        assert_eq!(node.to_string(), "[a, b, [c]]");

        // Pruned
        let meta = Rc::new(NodeMeta {
            name: "MyRule".into(),
            params: ["param1", "param2"].map(|s| s.into()).into(),
            flags: FlagMap::new(),
        });
        let pruned_tree = Tree::Node {
            meta,
            tree: Tree::Text("pruned_content".into()).into(),
        };
        assert_eq!(
            pruned_tree.to_string(),
            "MyRule[param1, param2]: pruned_content"
        );
    }
}
