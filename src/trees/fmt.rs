// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::{KeyValue, Tree, TreeTags};
use std::fmt;
use std::ops::Deref;

impl fmt::Display for TreeTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Collect and sort keys for a stable, predictable string
        let mut keys: Vec<&str> = self.tags.keys().map(|k| k.deref()).collect();
        keys.sort();

        write!(f, "{{")?;
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            // Safe to unwrap because we just got the key from the map
            write!(f, "{}: {}", key, self.tags.get(*key).unwrap())?;
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
            Self::Leaf(s) => write!(f, "\"{}\"", s),
            Self::Tag(kv) | Self::BranchingTag(kv) => write!(f, "{}", kv),
            Self::Root(v) => write!(f, "!{}", v),
            Self::BranchingRoot(v) => write!(f, "!!{}", v),
            Self::TreeTags(tags) => write!(f, "{}", tags),
            Self::Stump => write!(f, "()"),
            Self::Nil => write!(f, "∅"),
            Self::Bottom => write!(f, "⊥"),
            Self::Branches(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Self::Pruned(info, tree) => {
                let params = info.params.join(", ");
                write!(f, "{}[{}]: {}", info.name, params, tree)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::{KeyValue, PruneInfo, TagMap, Tree, TreeTags};
    use indexmap::IndexMap;
    use std::rc::Rc;

    #[test]
    fn test_tree_tags_display() {
        let mut tags_map = TagMap::new();
        tags_map.insert("key1".into(), Tree::Leaf("value1".into()));
        tags_map.insert("key2".into(), Tree::Leaf("value2".into()));
        let tags = TreeTags { tags: tags_map };
        assert_eq!(tags.to_string(), "{key1: \"value1\", key2: \"value2\"}");

        let tags_empty = TreeTags {
            tags: IndexMap::new(),
        };
        assert_eq!(tags_empty.to_string(), "{}");
    }

    #[test]
    fn test_key_value_display() {
        let kv = KeyValue("name".into(), Tree::Leaf("value".into()));
        assert_eq!(kv.to_string(), "«name=\"value\"»");
    }

    #[test]
    fn test_tree_display() {
        // Leaf
        assert_eq!(Tree::Leaf("hello".into()).to_string(), "\"hello\"");

        // LeafTag
        let kv_leaf = KeyValue("tag".into(), Tree::Leaf("leaf_val".into()));
        assert_eq!(Tree::Tag(kv_leaf.into()).to_string(), "«tag=\"leaf_val\"»");

        // NodeTag
        let kv_node = KeyValue("node".into(), Tree::Leaf("node_val".into()));
        assert_eq!(
            Tree::BranchingTag(kv_node.into()).to_string(),
            "«node=\"node_val\"»"
        );

        // RootLeaf
        assert_eq!(
            Tree::Root(Tree::Leaf("root".into()).into()).to_string(),
            "!\"root\""
        );

        // RootNode
        assert_eq!(
            Tree::BranchingRoot(Tree::Branches(vec![Tree::Leaf("item".into())].into()).into())
                .to_string(),
            "!![\"item\"]"
        );

        // Tags
        let mut tags_map = TagMap::new();
        tags_map.insert("a".into(), Tree::Leaf("1".into()));
        let tags = TreeTags { tags: tags_map };
        assert_eq!(Tree::TreeTags(tags.into()).to_string(), "{a: \"1\"}");

        // Stump
        assert_eq!(Tree::Stump.to_string(), "()");

        // Nil
        assert_eq!(Tree::Nil.to_string(), "∅");

        // Bottom
        assert_eq!(Tree::Bottom.to_string(), "⊥");

        // Node
        let node = Tree::Branches(
            vec![
                Tree::Leaf("a".into()),
                Tree::Leaf("b".into()),
                Tree::Branches(vec![Tree::Leaf("c".into())].into()),
            ]
            .into(),
        );
        assert_eq!(node.to_string(), "[\"a\", \"b\", [\"c\"]]");

        // Pruned
        let prune_info = Rc::new(PruneInfo {
            name: "MyRule".to_string(),
            params: vec!["param1".to_string(), "param2".to_string()].into(),
        });
        let pruned_tree = Tree::Pruned(prune_info, Tree::Leaf("pruned_content".into()).into());
        assert_eq!(
            pruned_tree.to_string(),
            "MyRule[param1, param2]: \"pruned_content\""
        );
    }
}
