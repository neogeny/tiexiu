// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::atomic::{AtomicUsize, Ordering};
use tiexiu::Heartbeat;

/// CLI heartbeat that updates a progress bar position.
#[derive(Debug)]
pub struct CliHeartbeat {
    pb: indicatif::ProgressBar,
    last_mark: AtomicUsize,
}

impl CliHeartbeat {
    /// Creates a new `CliHeartbeat` that drives the given progress bar.
    pub fn new(pb: indicatif::ProgressBar) -> Self {
        Self {
            pb,
            last_mark: AtomicUsize::new(0),
        }
    }
}

impl Heartbeat for CliHeartbeat {
    fn tick(&self, mark: usize, _total: usize) {
        let prev = self.last_mark.swap(mark, Ordering::Relaxed);
        if mark > prev {
            self.pb.set_position(mark as u64);
        }
    }
}
