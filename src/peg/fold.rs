// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use crate::util;
use crate::util::fold::Folds;

/// Folder trait for recursive traversal of PEG expressions.
/// Public API for user-defined tree transformations.
#[allow(dead_code)] // Public API -- used by downstream consumers, not within this crate.
pub trait Folder<O>: util::fold::Folder<Exp, O> {
    /// Folds an expression node with its already-folded children.
    fn fold(&mut self, item: &Exp, children: &[O]) -> O;
}

impl<O> Folds<Exp, O> for Exp {
    fn fold_with<F: util::fold::Folder<Exp, O> + ?Sized>(&self, folder: &mut F) -> O {
        let children = self.children();
        let folded: Vec<O> = children.iter().map(|c| c.fold_with(folder)).collect();
        folder.fold(self, &folded)
    }
}

impl Exp {
    fn children(&self) -> Vec<&Exp> {
        self.kind.children()
    }
}
