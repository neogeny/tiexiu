// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait Folder<I, O> {
    fn fold(&mut self, item: &I, children: &[O]) -> O;
}

pub trait Folds<I, O> {
    fn fold_with<V: Folder<I, O> + ?Sized>(&self, folder: &mut V) -> O;
}
