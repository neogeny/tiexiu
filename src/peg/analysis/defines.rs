// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::types::DefineSet;
use crate::exp::{Exp, ExpKind};

impl Exp {
    pub(in crate::peg) fn cache_defines(&mut self) {
        let mut names: DefineSet = DefineSet::new();
        self._defines(&mut names);
        self.df = Some(names.into_iter().collect::<Vec<_>>().into());
    }

    fn _defines(&self, names: &mut DefineSet) {
        match &self.kind {
            ExpKind::Named(name, _) => {
                names.insert((name.clone(), false));
            }
            ExpKind::NamedList(name, _) => {
                names.insert((name.clone(), true));
            }
            _ => {}
        }
        for child in self.kind.children() {
            child._defines(names);
        }
    }
}
