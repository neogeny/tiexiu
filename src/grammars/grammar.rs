// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::rule::Rule;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Grammar<'g> {
    name: &'g str,
    rulemap: HashMap<&'g str, Rule<'g>>,
}
