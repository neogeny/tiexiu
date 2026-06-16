// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::types::Str;
use crate::types::Ref;
use std::collections::LinkedList;

/// A linked list of tree references.
pub type TreeList = LinkedList<Tree>;

/// A key-value pair for named elements in a tree.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue(pub Str, pub Tree);

pub type Tree = serde_json::Value;

pub struct TreeBuild();

// The abstract syntax tree representation for parsed input.
// #[derive(Clone, Debug, PartialEq)]
// pub enum Tree {
//     Value(serde_json::Value),
//     /// A text/leaf node from tokens or patterns.
//     Text(Str),
//     /// A non-mergeable list of values.
//     Array(Vec<TreeRef>),
//     /// A mapping of named elements.
//     Object(TreeMap),
//
//     // NOTE these variants don't survive fold()
//     /// Parsing that didn't consume any input (internal).
//     Nil,
//     /// The result of parsing a rule call.
//     Node {
//         typename: Str,
//         tree: TreeRef,
//     },
//     /// A sequence of values (mergeable).
//     Seq(Vec<TreeRef>),
//     /// Failure marker used in memoization (internal).
//     Bottom,
//     /// A named element for tree merging (internal).
//     Named(KeyValue),
//     /// A named element forced into a list (internal).
//     NamedAsList(KeyValue),
//     /// Override value for merged tree (internal).
//     Override(TreeRef),
//     /// Override value forced into a list (internal).
//     OverrideAsList(TreeRef),
// }

/// Creates a KeyValue pair from a name and a tree.
pub fn keyval(name: &str, tree: Tree) -> KeyValue {
    KeyValue(name.into(), tree.into())
}

