// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

mod corectx;
/// Context traits for reading parser state.
pub mod ctx;
/// Context error types.
pub mod error;
pub(crate) mod memo;
/// Semantics actions for post-rule transformation.
pub mod semantics;
pub(crate) mod state;
/// String-backed parsing context.
pub mod strctx;
/// Tracing infrastructure for parse debugging.
pub mod trace;

// #[allow(dead_code)]
// pub mod stackctx;
// pub mod ctxproxy;

use crate::{CfgA, Cursor};
/// Traits for reading parser state (Ctx, CtxI).
pub use ctx::*;
/// Context operation failures.
pub use error::Error;
/// Semantics action types.
pub use semantics::*;
/// Reads parser state from a string cursor.
pub use strctx::StrCtx;

/// Prelude module re-exporting common context traits.
pub mod prelude {
    pub use super::ctx::*;
}

/// Create a new parsing context from a cursor and config.
pub fn new_ctx<'c, U: Cursor + 'c>(cursor: U, cfga: &'c CfgA) -> impl CtxSem {
    corectx::CoreCtx::new(cursor, cfga)
}
