// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::ops::{Deref, DerefMut};

/// A ScopeGuard that owns a subject `T` and executes a rollback action
/// on `T` if dropped before being defused.
pub struct ScopeGuard<T, F: FnOnce(&mut T)> {
    subject: Option<T>,
    action: Option<F>,
}

/// Creates a new scope guard with the given subject and rollback action.
pub fn guard<T, F: FnOnce(&mut T)>(subject: T, action: F) -> ScopeGuard<T, F> {
    ScopeGuard::new(subject, action)
}

/// Allows the Guard to be used as if it were the subject.
impl<T, F: FnOnce(&mut T)> Deref for ScopeGuard<T, F> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.subject.as_ref().expect("Guard subject missing")
    }
}

impl<T, F: FnOnce(&mut T)> DerefMut for ScopeGuard<T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.subject.as_mut().expect("Guard subject missing")
    }
}

impl<T, F: FnOnce(&mut T)> Drop for ScopeGuard<T, F> {
    fn drop(&mut self) {
        if let (Some(mut subject), Some(action)) = (self.subject.take(), self.action.take()) {
            action(&mut subject);
        }
    }
}

impl<T, F: FnOnce(&mut T)> ScopeGuard<T, F> {
    pub fn new(subject: T, action: F) -> Self {
        Self {
            subject: Some(subject),
            action: Some(action),
        }
    }

    /// Defuses the guard and returns the original subject.
    /// The rollback action will NOT be executed.
    pub fn into_inner(mut self) -> T {
        self.action = None; // Disable the drop action
        self.subject.take().expect("Guard subject missing")
    }
}
