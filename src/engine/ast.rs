// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;
use std::collections::HashMap;
use super::cst::{Cst, CstRc};

pub const __AT__: &str = "&";

/// A structured mapping for AST nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ast {
    pub fields: HashMap<String, CstRc>,
}

impl Ast {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&CstRc> {
        self.fields.get(key)
    }

    pub fn update(&mut self, other: &Ast) {
        for (key, value) in &other.fields {
            self.fields.insert(key.clone(), Rc::clone(value));
        }
    }

    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safekey(k);
            self.fields.entry(key).or_insert(Rc::new(Cst::Void));
        }

        for &k in list_keys {
            let key = self.safekey(k);
            self.fields.entry(key).or_insert(Rc::new(Cst::List(Vec::new())));
        }
    }

    pub fn set(&mut self, key: &str, item: Cst) {
        let key = self.safekey(key);
        if let Some(current) = self.fields.remove(&key) {
            let new = (*current).clone().add(item);
            self.fields.insert(key, new);
        }
        else {
            self.fields.insert(key, Rc::new(item));
        }
    }
    
    pub fn set_list(&mut self, key: &str, item: Cst) {
        let key = self.safekey(key);
        if let Some(current) = self.fields.remove(&key) {
            let new = (*current).clone().addlist(item);
            self.fields.insert(key, new);
        } else {
            self.fields.insert(key, Rc::new(item));
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
        matches!(key, "items" | "keys" | "values" | "get" | "update" | "pop" | "clear")
    }
}