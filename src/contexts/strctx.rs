// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::corectx::CoreCtx;
use crate::input::StrCursor;

pub type StrCtx<'c> = CoreCtx<'c, StrCursor<'c>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    const TARGET: usize = 16;

    #[test]
    fn test_ctx_size() {
        let size = size_of::<StrCtx>();
        // 24 bytes: Box (8) + Rc (8) + bool/padding (8)
        assert!(size <= TARGET, "StrCtx size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_cursor_size() {
        let size = size_of::<StrCursor>();
        // StrCursor contains &str (16) and usize (8) = 24 bytes.
        assert!(
            size <= TARGET,
            "StrCursor size is {} > {} bytes",
            size,
            TARGET
        );
    }
}
