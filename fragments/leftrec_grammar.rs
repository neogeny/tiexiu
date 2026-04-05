// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Element;
use super::Grammar;
use std::collections::HashMap;

impl Grammar {
    pub fn mark_left_recursion(grammar: &mut Grammar) {
        if grammar.analyzed {
            return;
        }
        let mut analysis = LeftRecursionAnalysis::new(grammar);
        analysis.run();
        grammar.analyzed = true;
    }
}


