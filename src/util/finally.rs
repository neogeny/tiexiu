// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

/// A scope guard that runs an action on drop.
pub struct Finally<F: FnOnce()> {
    action: Option<F>,
}

impl<F: FnOnce()> Finally<F> {
    /// Creates a new Finally with the given action.
    pub fn new(action: F) -> Self {
        Self {
            action: Some(action),
        }
    }

    /// Prevents the action from running on drop.
    pub fn defuse(&mut self) {
        self.action = None; // Clear the action
    }
}

impl<F: FnOnce()> Drop for Finally<F> {
    fn drop(&mut self) {
        // If action is still Some, it means we didn't defuse.
        if let Some(action) = self.action.take() {
            action();
        }
    }
}

/// Creates a scope guard that runs the given action on drop.
pub fn finally<F: FnOnce()>(f: F) -> Finally<F> {
    Finally::new(f)
}
