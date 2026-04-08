// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use super::exp::ExpKind;
use crate::trees;
use crate::trees::Tree;

pub trait Compiler: trees::fold::Translator<Exp> {
    fn translate(&mut self, item: &Tree) -> Exp {
        self.compile(item)
    }

    fn compile(&mut self, item: &Tree) -> Exp;
}

pub trait Compiles: trees::fold::Translates<Exp> {
    fn compile_with<V: Compiler + ?Sized>(&self, compiler: &mut V) -> Exp {
        self.translate_with(compiler)
    }
}

pub trait Linker {
    fn link(&mut self, exp: &mut Exp) {
        self.walk(exp);
    }

    fn walk(&mut self, exp: &mut Exp) {
        match &mut exp.kind {
            ExpKind::Call { .. } => self.link_call(exp),
            ExpKind::RuleInclude { .. } => self.link_rule_include(exp),

            ExpKind::Named(_, inner)
            | ExpKind::NamedList(_, inner)
            | ExpKind::Override(inner)
            | ExpKind::OverrideList(inner)
            | ExpKind::Group(inner)
            | ExpKind::SkipGroup(inner)
            | ExpKind::Lookahead(inner)
            | ExpKind::NegativeLookahead(inner)
            | ExpKind::SkipTo(inner)
            | ExpKind::Alt(inner)
            | ExpKind::Optional(inner)
            | ExpKind::Closure(inner)
            | ExpKind::PositiveClosure(inner) => self.walk(inner),

            ExpKind::Sequence(items) | ExpKind::Choice(items) => {
                for item in items.iter_mut() {
                    self.walk(item);
                }
            }

            ExpKind::Join { exp, sep }
            | ExpKind::PositiveJoin { exp, sep }
            | ExpKind::Gather { exp, sep }
            | ExpKind::PositiveGather { exp, sep } => {
                self.walk(exp);
                self.walk(sep);
            }

            ExpKind::Nil
            | ExpKind::Cut
            | ExpKind::Void
            | ExpKind::Fail
            | ExpKind::Dot
            | ExpKind::Eof
            | ExpKind::Token(_)
            | ExpKind::Pattern(_)
            | ExpKind::Constant(_)
            | ExpKind::Alert(_, _) => {}
        }
    }

    fn link_call(&mut self, _exp: &mut Exp) {}
    fn link_rule_include(&mut self, _exp: &mut Exp) {}
}
