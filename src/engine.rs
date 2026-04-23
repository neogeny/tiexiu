// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

mod corectx;
pub mod ctx;
pub mod ctxproxy;
pub mod error;
pub mod memo;
pub mod state;
pub mod strctx;
pub mod trace;

use crate::{CfgA, Cursor};
pub use ctx::*;
pub use error::Error;
pub use strctx::StrCtx;

pub mod prelude {
    pub use super::ctx::*;
}

pub fn new_ctx<'c, U: Cursor + Clone + 'c>(cursor: U, cfga: &'c CfgA) -> impl Ctx {
    corectx::CoreCtx::new(cursor, cfga)
}
