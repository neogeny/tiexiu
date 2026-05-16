// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::atomic::{AtomicU64, Ordering};

/// A globally-unique identifier based on an atomic counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Memento(u64);

impl Default for Memento {
    fn default() -> Self {
        Self::new()
    }
}

impl Memento {
    /// Creates a new globally-unique Memento.
    pub fn new() -> Self {
        // A global, static counter that lives for the duration of the program
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        // Fetch the current value and increment it by 1 atomically
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        Memento(id)
    }

    /// Returns the unique identifier.
    pub fn id(&self) -> u64 {
        self.0
    }
}
