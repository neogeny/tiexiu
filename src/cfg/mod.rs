// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod constants;
pub mod keys;

pub use keys::{Cfg, CfgA, CfgBox, Configurable};

pub fn cfg(input: &CfgA) -> CfgBox {
    CfgBox::load_from_env(constants::ENV_PREFIX).merge(&CfgBox::new(input))
}
