// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod constants;

pub use constants::*;

pub use crate::util::cfg::*;

pub fn cfg(input: CfgA) -> Cfg {
    Cfg::fromenv(ENV_PREFIX).merge(&Cfg::new(input))
}
