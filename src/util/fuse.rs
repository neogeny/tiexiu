// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

/// A one-shot fuse that enforces lifecycle ordering.
#[derive(Debug, Clone)]
pub struct Fuse(pub Option<()>);

impl Default for Fuse {
    fn default() -> Self {
        Self(Some(()))
    }
}

impl Fuse {
    /// Creates a new, unburnt Fuse.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns true if the fuse has not been burnt.
    pub fn is_good(&self) -> bool {
        self.0.is_some()
    }

    /// Returns true if the fuse has been burnt.
    pub fn is_burnt(&self) -> bool {
        self.0.is_none()
    }

    /// Burns the fuse, panicking if already burnt.
    #[track_caller]
    pub fn burn(&mut self) {
        if self.0.is_none() {
            panic!("Fuse already burnt");
        }
        self.0 = None;
    }
}

impl Drop for Fuse {
    #[track_caller]
    fn drop(&mut self) {
        // Failing to burn a fuse before dropping it represents an unhandled lifecycle.
        if self.is_good() {
            if std::thread::panicking() {
                return; // Prevent double panics during stack unwinding
            }
            panic!("Fuse dropped while still active (must be explicitly burnt)");
        }
    }
}
