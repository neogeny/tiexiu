// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Tree;
use crate::trees::TreeRef;
use crate::types::Str;

pub trait Folds {
    /// Reshapes and transforms the given tree, returning the folded result.
    fn fold(&self, tree: TreeRef) -> TreeRef;
}

pub struct FoldsNamed(pub Str);

impl Folds for FoldsNamed {
    fn fold(&self, tree: TreeRef) -> TreeRef {
        Tree::named(self.0.clone(), tree).into()
    }
}

pub struct FoldsOverride;

impl Folds for FoldsOverride {
    fn fold(&self, tree: TreeRef) -> TreeRef {
        Tree::override_with(tree).into()
    }
}
