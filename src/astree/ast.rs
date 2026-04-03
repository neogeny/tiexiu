// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::cst::Cst;
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

/// A structured mapping for AST nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ast {
    pub fields: HashMap<String, Cst>,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Collect and sort keys for a stable, predictable string
        let mut keys: Vec<&String> = self.fields.keys().collect();
        keys.sort();

        write!(f, "{{")?;
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            // Safe to unwrap because we just got the key from the map
            write!(f, "{}: {}", key, self.fields.get(*key).unwrap())?;
        }
        write!(f, "}}")
    }
}

impl Ast {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Cst> {
        self.fields.get(key)
    }

    pub fn update(&mut self, other: &Ast) {
        for (key, value) in &other.fields {
            self.fields.insert(key.clone(), value.clone());
        }
    }

    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safekey(k);
            self.fields.entry(key).or_insert(Cst::Nil);
        }

        for &k in list_keys {
            let key = self.safekey(k);
            self.fields.entry(key).or_insert(Cst::List([].into()));
        }
    }

    pub fn set(&mut self, key: &str, item: Cst) {
        let key = self.safekey(key);
        if let Some(current) = self.fields.remove(&key) {
            let new = current.add(item);
            self.fields.insert(key, new);
        } else {
            self.fields.insert(key, item);
        }
    }

    pub fn set_list(&mut self, key: &str, item: Cst) {
        let key = self.safekey(key);
        if let Some(current) = self.fields.remove(&key) {
            let new = current.addlist(item);
            self.fields.insert(key, new);
        } else {
            self.fields.insert(key, item);
        }
    }

    fn safekey(&self, key: &str) -> String {
        let mut k = key.to_string();
        while self.is_unsafe(&k) {
            k.push('_');
        }
        k
    }

    fn is_unsafe(&self, key: &str) -> bool {
        matches!(
            key,
            "items" | "keys" | "values" | "get" | "update" | "pop" | "clear"
        )
    }
}
