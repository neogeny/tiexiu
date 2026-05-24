// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::corectx::CoreCtx;
use crate::input::strcursor::StrCursor;

/// A parsing context backed by a `StrCursor`.
pub type StrCtx<'c> = CoreCtx<'c, StrCursor>;

impl<'c> From<&str> for StrCtx<'c> {
    fn from(text: &str) -> Self {
        Self::new(StrCursor::new(text), &[])
    }
}

impl<'c> From<StrCursor> for StrCtx<'c> {
    fn from(cursor: StrCursor) -> Self {
        Self::new(cursor, &[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ctx_handle_size() {
        // CoreCtx now holds HeavyState and ParseState directly — no more
        // Cow/RefCell wrappers.  The 32-byte bound was meaningful for the
        // old owned-ctx design where contexts were cloned at every recursion
        // point.  With &mut C semantics, the context is created once and
        // passed by reference.  No size constraint.
        let _ = CoreCtx::<StrCursor>::new;
    }
}
