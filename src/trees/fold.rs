// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;

pub trait Translator<O> {
    fn translate(&mut self, tree: &Tree, branches: &[O]) -> O;
}s

pub trait Translates<O> {
    fn translate_with<T: Translator<O> + ?Sized>(&self, trans: &mut T) -> O;
}

impl<O> Translates<O> for Tree {
    fn translate_with<T: Translator<O> + ?Sized>(&self, trans: &mut T) -> O {
        match &self {
            Tree::Nil | Tree::Bottom | Tree::Stump => trans.translate(self, &[]),
            Tree::Node(nodes) | Tree::Pruned(nodes) => {
                let outputs = nodes
                    .iter()
                    .map(|branch| branch.translate_with(trans))
                    .collect::<Vec<_>>();
                trans.translate(self, outputs.as_slice())
            }
            // ...
            _ => trans.translate(self, &[]),
        }
    }
}
