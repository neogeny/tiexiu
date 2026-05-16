// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

/// A trait for folding/transforming items with children.
pub trait Folder<I, O> {
    /// Fold an item given its processed children.
    fn fold(&mut self, item: &I, children: &[O]) -> O;
}

/// A trait for types that can be folded by a Folder.
pub trait Folds<I, O> {
    /// Fold this value using the given folder.
    fn fold_with<V: Folder<I, O> + ?Sized>(&self, folder: &mut V) -> O;
}
