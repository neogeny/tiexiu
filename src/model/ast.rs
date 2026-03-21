// Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use super::cst::{Cst, cst_add};
use super::parsed::Parsed;

/// A structured mapping for AST nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ast {
    pub fields: HashMap<String, Option<Cst>>,
}

impl Ast {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pre-defines keys to ensure they exist in the resulting mapping.
    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safekey(k);
            self.fields.entry(key).or_insert(None);
        }

        for &k in list_keys {
            let key = self.safekey(k);
            // Initialize list keys with an empty Cst::List
            self.fields.entry(key).or_insert(Some(Cst::List(Vec::new())));
        }
    }

    /// Sets a value using our internal Parsed type.
    pub fn set(&mut self, key: &str, item: Parsed, as_list: bool) {
        let key = self.safekey(key);

        // Take ownership of the current value to process it via cst_add
        // Note: .flatten() is useful here to collapse Option<Option<Cst>>
        let current = self.fields.remove(&key).flatten();
        let updated = cst_add(current, item, as_list);

        self.fields.insert(key, Some(updated));
    }

    /// Protects the Python boundary by ensuring keys don't collide.
    fn safekey(&self, key: &str) -> String {
        let mut k = key.to_string();
        while self.is_unsafe(&k) {
            k.push('_');
        }
        k
    }

    fn is_unsafe(&self, key: &str) -> bool {
        matches!(key, "items" | "keys" | "values" | "get" | "update" | "pop" | "clear")
    }
}