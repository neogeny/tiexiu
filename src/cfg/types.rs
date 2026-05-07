// SPDX-License-Identifier: MIT OR Apache-2.0

use ahash::RandomState;
use indexmap::{IndexMap, IndexSet};
use std::rc::Rc;

pub type Ref<T> = Rc<T>;
pub type Str = Rc<str>;
pub type Define = (Str, bool);

pub type FastIndexMap<K, V> = IndexMap<K, V, RandomState>;
pub type FastIndexSet<T> = IndexSet<T, RandomState>;
pub type StrSet = IndexSet<Str>;
pub type FlagMap = IndexMap<Str, bool>;
pub type DefineSet = IndexSet<Define>;
