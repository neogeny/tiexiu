// Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::any::Any;
use std::collections::HashMap;
use super::cst::{Cst, cst_add};

/// The unified return type for any grammar rule.
pub enum RuleResult {
    Cst(Cst),
    Ast(Ast),
}

/// A structured mapping for AST nodes.
pub struct Ast {
    fields: HashMap<String, Option<Cst>>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Pre-defines keys to ensure they exist in the resulting mapping.
    /// This satisfies the TatSu expectation that all named elements are present.
    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safekey(k);
            // Only insert if not already present to avoid overwriting 
            // values during complex choice matches.
            self.fields.entry(key).or_insert(None);
        }

        for &k in list_keys {
            let key = self.safekey(k);
            self.fields.entry(key).or_insert_with(|| Some(Cst::List(Vec::new())));
        }
    }

    /// Sets a value using CST addition semantics.
    pub fn set<T: Any>(&mut self, key: &str, node: T, as_list: bool) {
        let key = self.safekey(key);
        
        // Take ownership of the current value (if any) to process it.
        let current = self.fields.remove(&key).flatten();
        let updated = cst_add(current, node, as_list);
        
        self.fields.insert(key, Some(updated));
    }

    /// Handles Python-style attribute shadowing and "unsafe" names.
    fn safekey(&self, key: &str) -> String {
        let mut k = key.to_string();
        while self.is_unsafe(&k) {
            k.push('_');
        }
        k
    }

    fn is_unsafe(&self, key: &str) -> bool {
        // Includes standard Python dict methods to prevent collision
        // when the AST eventually crosses the boundary.
        matches!(key, "items" | "keys" | "values" | "get" | "update" | "pop" | "clear")
    }
}