// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

/// ```
/// pub fn example_operation() {
///     // Logic starts here
///     let _guard = Finally(Some(|| {
///         // This is your "finally" block
///         println!("Cleanup complete.");
///     }));
///
///     // If any code here returns early or panics, _guard is dropped.
/// }
/// ```
pub struct Finally<F: FnOnce()> {
    action: Option<F>,
}

impl<F: FnOnce()> Finally<F> {
    pub fn new(action: F) -> Self {
        Self {
            action: Some(action),
        }
    }

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

pub fn finally<F: FnOnce()>(f: F) -> Finally<F> {
    Finally::new(f)
}
