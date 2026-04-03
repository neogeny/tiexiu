// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::rule::{Rule, RuleMap};
use crate::model::leftrec::mark_left_recursion;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub rulemap: RuleMap,
}

impl Grammar {
    pub fn new(name: &str, rules: &[Rule]) -> Self {
        let rulemap = rules.iter().cloned().map(|r| (r.name.clone(), r)).collect();

        let mut grammar = Self {
            name: name.to_string(),
            rulemap,
        };
        mark_left_recursion(&mut grammar);
        grammar
    }
}

impl Default for Grammar {
    #[inline]
    fn default() -> Self {
        let mut grammar = Self {
            name: "default".to_string(),
            rulemap: RuleMap::default(),
        };
        mark_left_recursion(&mut grammar);
        grammar
    }
}
