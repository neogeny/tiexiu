// SPDX-License-Identifier: MIT OR Apache-2.0

use ahash::RandomState;
use indexmap::{IndexMap, IndexSet};
use std::sync::Arc;

/// Reference-counted pointer.
pub type Ref<T> = Arc<T>;
/// Reference-counted string slice.
pub type Str = String;
/// A definition pair (name, is_flag).
pub type Define = (Str, bool);

/// Index map with fast hashing.
pub type FastIndexMap<K, V> = IndexMap<K, V, RandomState>;
/// Index set with fast hashing.
pub type FastIndexSet<T> = IndexSet<T, RandomState>;
/// Set of strings with fast hashing.
pub type StrSet = IndexSet<Str>;
/// Map of string flags.
pub type FlagMap = IndexMap<Str, bool>;
/// Set of definition pairs.
pub type DefineSet = IndexSet<Define>;
