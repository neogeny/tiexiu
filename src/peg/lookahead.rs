// copyright (c) 2026 juancarlo añez (apalala@gmail.com)
// spdx-license-identifier: mit or apache-2.0

use super::exp::{Exp, ExpKind, Str};
use std::collections::HashSet;

impl Exp {
    pub(super) fn compute_lookahead(&mut self) -> HashSet<Box<str>> {
        let mut lookaheads = HashSet::new();

        match &self.kind {
            ExpKind::Token(s) | ExpKind::Constant(s) | ExpKind::Alert(s, _) => {
                lookaheads.insert(s.clone());
            }
            ExpKind::Pattern(s) => {
                lookaheads.insert(s.clone());
            }
            ExpKind::Eof => {
                lookaheads.insert("EOF".into());
            }
            ExpKind::Dot => {
                lookaheads.insert(".".into());
            }
            _ => {}
        }

        for exp in self.callable_from_mut() {
            lookaheads.extend(exp.compute_lookahead());
        }

        if !lookaheads.is_empty() {
            let mut vec: Vec<Str> = lookaheads.iter().cloned().collect();
            vec.sort();
            self.la = vec.into_boxed_slice();
        }

        lookaheads
    }
}
