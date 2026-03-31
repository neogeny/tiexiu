// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::rule::Rule;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Grammar<'g> {
    pub name: &'g str,
    pub rulemap: HashMap<&'g str, Rule<'g>>,
}
