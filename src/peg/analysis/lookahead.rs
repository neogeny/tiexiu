// copyright (c) 2026 juancarlo añez (apalala@gmail.com)
// spdx-license-identifier: mit or apache-2.0

use crate::SYM_ETX;
use crate::cfg::types::{Str, StrSet};
use crate::exp::{Exp, ExpKind};

impl Exp {
    pub(in crate::peg) fn cache_lookahead(&mut self) -> StrSet {
        let mut lookaheads = StrSet::new();

        match &self.kind {
            ExpKind::Token(s) | ExpKind::Constant(s) | ExpKind::Alert(s, _) => {
                lookaheads.insert(s.clone());
            }
            ExpKind::Pattern(s) => {
                lookaheads.insert(s.clone());
            }
            ExpKind::Eof => {
                lookaheads.insert(SYM_ETX.into());
            }
            _ => {}
        }

        for exp in self.callable_from_mut() {
            lookaheads.extend(exp.cache_lookahead());
        }

        let mut vec: Vec<Str> = lookaheads.iter().cloned().collect();
        vec.sort();
        self.la = Some(vec.into());

        lookaheads
    }
}
