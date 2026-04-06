// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use crate::trees::Tree;
use crate::trees::fold;

pub trait Compiler: fold::Translator<Exp> {
    fn translate(&mut self, item: &Tree) -> Exp {
        self.compile(item)
    }

    fn compile(&mut self, item: &Tree) -> Exp;
}

pub trait Compiles: fold::Translates<Exp> {
    fn compile_with<V: Compiler + ?Sized>(&self, compiler: &mut V) -> Exp {
        self.translate_with(compiler)
    }
}
