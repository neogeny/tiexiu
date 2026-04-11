// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ExpKind};
use crate::trees;
use crate::trees::Tree;
use crate::util;
use crate::util::fold::Folds;

pub trait Folder<O>: util::fold::Folder<Exp, O> {
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
        match &self.kind {
            ExpKind::Nil
            | ExpKind::Cut
            | ExpKind::Void
            | ExpKind::Fail
            | ExpKind::Dot
            | ExpKind::Eof
            | ExpKind::Token(_)
            | ExpKind::Pattern(_)
            | ExpKind::Constant(_)
            | ExpKind::Alert(_, _)
            | ExpKind::Call { .. } => vec![],

            ExpKind::Named(_, e) => vec![e],
            ExpKind::NamedList(_, e) => vec![e],
            ExpKind::Override(e) => vec![e],
            ExpKind::OverrideList(e) => vec![e],
            ExpKind::Group(e) => vec![e],
            ExpKind::SkipGroup(e) => vec![e],
            ExpKind::Lookahead(e) => vec![e],
            ExpKind::NegativeLookahead(e) => vec![e],
            ExpKind::SkipTo(e) => vec![e],
            ExpKind::Alt(e) => vec![e],
            ExpKind::Optional(e) => vec![e],
            ExpKind::Closure(e) => vec![e],
            ExpKind::PositiveClosure(e) => vec![e],

            ExpKind::Sequence(arr) | ExpKind::Choice(arr) => arr.iter().collect(),

            ExpKind::Join { exp, sep } => vec![exp, sep],
            ExpKind::PositiveJoin { exp, sep } => vec![exp, sep],
            ExpKind::Gather { exp, sep } => vec![exp, sep],
            ExpKind::PositiveGather { exp, sep } => vec![exp, sep],

            ExpKind::RuleInclude { exp, .. } => exp.as_ref().map_or(vec![], |e| vec![e.as_ref()]),
        }
    }
}
