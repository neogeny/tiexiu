use crate::trees::{KeyValue, Tree, TreeTags};
use std::fmt;

impl fmt::Display for TreeTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Collect and sort keys for a stable, predictable string
        let mut keys: Vec<&String> = self.tags.keys().collect();
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
            Self::Node(items) => {
                let bracket = if matches!(self, Self::Node(_)) {
                    ("[", "]")
                } else {
                    ("{", "}")
                };
                write!(f, "{}", bracket.0)?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "{}", bracket.1)
            }
            Self::LeafTag(kv) | Self::NodeTag(kv) => write!(f, "{}", kv),
            Self::RootLeaf(v) => write!(f, "!{}", v),
            Self::RootNode(v) => write!(f, "!!{}", v),
            Self::Tags(tags) => write!(f, "{}", tags),
            Self::Stump => write!(f, "()"),
            Self::Nil => write!(f, "∅"),
            Self::Bottom => write!(f, "⊥"),
        }
    }
}
