// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt::Debug;
use std::sync::Arc;

/// A no-op heartbeat implementation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NullHeartbeat;

/// Trait for progress-reporting callbacks during long operations.
pub trait Heartbeat: Debug {
    fn tick(&self, _mark: usize, _total: usize) {}
}

impl Heartbeat for NullHeartbeat {}

impl Heartbeat for () {}

/// Thread-safe reference to a heartbeat.
pub type HeartbeatRef = Arc<dyn Heartbeat>;
