// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

/// pub fn example_operation() {
///     // Logic starts here
///     let _guard = Finally(Some(|| {
///         // This is your "finally" block
///         println!("Cleanup complete.");
///     }));
///
///     // If any code here returns early or panics, _guard is dropped.
/// }
pub struct Finally<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Finally<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

pub fn finally<F: FnOnce()>(f: F) -> Finally<F> {
    Finally ( Some(f) )
}