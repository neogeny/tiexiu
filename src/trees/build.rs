// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tags::TreeTags;
use super::tree::{KeyValue, PruneInfo, Tree};

impl Tree {
    pub fn stump() -> Tree {
        Self::Stump
    }

    pub fn leaf(value: &str) -> Tree {
        Self::Leaf(value.into())
    }

    pub fn branches(branches: &[Tree]) -> Tree {
        Self::Branches(branches.into())
    }

    pub fn tags(tags: TreeTags) -> Tree {
        Self::TreeTags(tags.into())
    }

    pub fn tag(key: &str, value: Tree) -> Tree {
        let keyval = KeyValue(key.into(), value);
        Self::Tag(keyval.into())
    }

    pub fn branching_tag(key: &str, value: Tree) -> Tree {
        let keyval = KeyValue(key.into(), value);
        Self::BranchingTag(keyval.into())
    }

    pub fn root(tree: Tree) -> Tree {
        Self::Root(tree.into())
    }

    pub fn branching_root(tree: Tree) -> Tree {
        Self::BranchingRoot(tree.into())
    }

    pub fn pruned(name: &str, params: &[String], tree: Tree) -> Tree {
        let pi = PruneInfo {
            name: name.into(),
            params: params.into(),
        };
        Self::Pruned(pi.into(), tree.into())
    }

    pub fn bottom() -> Tree {
        Self::Bottom
    }

    pub fn nil() -> Tree {
        Self::Nil
    }
}
