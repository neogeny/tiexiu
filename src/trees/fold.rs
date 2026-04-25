// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;
use crate::trees::KeyValue;

pub trait Translator<O> {
    fn translate(&mut self, tree: &Tree, branches: &[O]) -> O;
}

pub trait Translates<O> {
    fn translate_with<T: Translator<O> + ?Sized>(&self, trans: &mut T) -> O;
}

impl<O> Translates<O> for Tree {
    fn translate_with<T: Translator<O> + ?Sized>(&self, trans: &mut T) -> O {
        match &self {
            Tree::Nil | Tree::Bottom => trans.translate(self, &[]),
            Tree::Text(_) => trans.translate(self, &[]),
            Tree::Seq(nodes) | Tree::Closed(nodes) => {
                let outputs = nodes
                    .iter()
                    .map(|branch| branch.translate_with(trans))
                    .collect::<Vec<_>>();
                trans.translate(self, outputs.as_slice())
            },
            Tree::Map(map) => {
                let outputs: Vec<O> = map
                    .iter()
                    .map(
                        |(k, v)|
                            Tree::Named(
                                KeyValue(
                                    k.clone(),
                                    v.clone().into()
                                )
                            )
                        .translate_with(trans)
                    )
                    .collect();
                trans.translate(self, outputs.as_slice())
            }
            Tree::Node {
                typename: _,
                tree,
            } => {
                let child = vec![tree.translate_with(trans)];
                trans.translate(self,  child.as_slice())
            },
            Tree::NamedAsList(keyval) |
            Tree::Named(keyval) => {
                let child = vec![keyval.1.translate_with(trans)];
                trans.translate(self, child.as_slice())
            },
            Tree::Override(tree) |        // Sets the value of the whole 
            Tree::OverrideAsList(tree) => {
                let child = vec![tree.translate_with(trans)];
                trans.translate(self, child.as_slice())
            }
        }
    }
}
