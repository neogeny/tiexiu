// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::corectx::CoreCtx;
use crate::input::strcursor::StrCursor;

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
    use crate::input::strcursor::StrCursor;
    use crate::state::{Ctx, CtxI};
    use crate::trees::Tree;
    use std::mem::size_of;

    const TARGET: usize = 32;

    #[test]
    fn test_ctx_size() {
        let size = size_of::<StrCtx>();
        assert!(size <= TARGET, "StrCtx size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_cursor_size() {
        let size = size_of::<StrCursor>();
        assert!(
            size <= TARGET,
            "StrCursor size is {} > {} bytes",
            size,
            TARGET
        );
    }
    #[test]
    fn test_ctx_handle_size() {
        let size = size_of::<CoreCtx<StrCursor>>();
        assert!(
            size <= TARGET,
            "CoreCtx handle size is {} > {} bytes",
            size,
            TARGET
        );
    }

    #[test]
    fn test_cow_behavior() {
        let text = "calculate 1 + 2";
        let cursor: StrCursor = StrCursor::new(text);

        let mut ctx1 = CoreCtx::new(cursor, &[]);

        ctx1.reset(10);
        assert_eq!(ctx1.cursor().mark(), 10);

        let mut ctx2 = ctx1.clone();
        assert_eq!(ctx2.cursor().mark(), 10, "Clone should inherit offset");

        ctx2.reset(5);

        assert_eq!(ctx2.cursor().mark(), 5, "Ctx2 should update to 5");
        assert_eq!(ctx1.cursor().mark(), 10, "Ctx1 should remain at 10 (CoW)");

        ctx2.cut();
        assert!(ctx2.cut_seen(), "Ctx2 should be cut");
        assert!(!ctx1.cut_seen(), "Ctx1 should remain uncut (CoW)");
    }

    #[test]
    fn test_shared_memoization_semantics() {
        let text = "abc";
        let cursor: StrCursor = StrCursor::new(text);
        let mut ctx1 = CoreCtx::new(cursor, &[]);

        let mut ctx2 = ctx1.clone();

        let key = ctx1.key("hello");

        ctx1.memoize(&key, &Tree::Nil);

        let retrieved = ctx2.memo(&key);

        assert!(
            retrieved.is_some(),
            "ctx2 failed to see the memoization entry from ctx1"
        );
        assert_eq!(
            retrieved.unwrap().tree,
            Tree::Nil,
            "Memoization data mismatch between shared contexts"
        );
    }

    #[test]
    fn test_state_isolation_preserves_shared_cache() {
        let cursor: StrCursor = StrCursor::new("abc");
        let mut ctx1 = CoreCtx::new(cursor, &[]);
        let mut ctx2 = ctx1.clone();

        ctx2.reset(1);

        assert_ne!(ctx1.cursor().mark(), ctx2.cursor().mark());

        let entry = Tree::Bottom;
        let key = ctx1.key("world");
        ctx2.memoize(&key, &entry);
        assert!(
            ctx1.memo(&key).is_some(),
            "Shared cache link was broken by a Copy-on-Write state split"
        );
    }
}
