// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::rule::Rule;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Grammar<'g> {
    pub name: &'g str,
    pub rulemap: HashMap<&'g str, Rule<'g>>,
}

impl<'g> Grammar<'g> {
    pub fn new(name: &'g str, rules: &[Rule<'g>]) -> Self {
        let rulemap = rules.iter().cloned().map(|r| (r.name, r)).collect();

        let grammar = Self { name, rulemap };
        super::leftrec::mark_left_recursion(grammar)
    }
}
