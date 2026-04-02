// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::corectx::CoreCtx;
use crate::input::strcursor::StrCursor;

pub type StrCtx<'c, P> = CoreCtx<'c, StrCursor<'c, P>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::strcursor::DefaultPatterns;
    use std::mem::size_of;
    use crate::grammars::Grammar;
    use crate::input::strcursor::StrCursor;
    use crate::contexts::{Cst, Ctx};


    const TARGET: usize = 16;

    #[test]
    fn test_ctx_size() {
        let size = size_of::<StrCtx<DefaultPatterns>>();
        // 24 bytes: Box (8) + Rc (8) + bool/padding (8)
        assert!(size <= TARGET, "StrCtx size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_cursor_size() {
        let size = size_of::<StrCursor<DefaultPatterns>>();
        // StrCursor contains &str (16) and usize (8) = 24 bytes.
        assert!(
            size <= 24,
            "StrCursor size is {} > {} bytes",
            size,
            TARGET
        );
    }
    #[test]
    fn test_ctx_handle_size() {
        let size = size_of::<CoreCtx<StrCursor<DefaultPatterns>>>();
        // CoreCtx should be 16 bytes: Rc<State> (8) + Rc<HeavyState> (8)
        assert!(size <= TARGET, "CoreCtx handle size is {} > 16 bytes", size);
    }

    #[test]
    fn test_cow_behavior() {
        let grammar = Grammar::default(); // Assuming default/mock grammar
        let text = "calculate 1 + 2";
        let cursor: StrCursor<DefaultPatterns> = StrCursor::new(text);

        let mut ctx1 = CoreCtx::new(cursor, &grammar);

        // Advance ctx1 to offset 10
        ctx1.reset(10);
        assert_eq!(ctx1.cursor().mark(), 10);

        // Clone ctx1. At this point, they share the same Rc<State>.
        let mut ctx2 = ctx1.clone();
        assert_eq!(ctx2.cursor().mark(), 10, "Clone should inherit offset");

        // Mutate ctx2. This triggers Rc::make_mut(), allocating a new State.
        ctx2.reset(5);

        assert_eq!(ctx2.cursor().mark(), 5, "Ctx2 should update to 5");
        assert_eq!(ctx1.cursor().mark(), 10, "Ctx1 should remain at 10 (CoW)");

        // Verify cutseen isolation
        ctx2.cut();
        assert!(ctx2.cut_seen(), "Ctx2 should be cut");
        assert!(!ctx1.cut_seen(), "Ctx1 should remain uncut (CoW)");
    }

    #[test]
    fn test_shared_memoization_semantics() {
        let grammar = Grammar::default();
        let text = "abc";
        let cursor: StrCursor<DefaultPatterns> = StrCursor::new(text);
        let mut ctx1 = CoreCtx::new(cursor, &grammar);

        // Clone the context. ctx1 and ctx2 now have independent Rc<State> handles
        // but point to the same Rc<HeavyState>.
        let mut ctx2 = ctx1.clone();

        let key = ctx1.key("hello");

        // 1. Memoize a result using ctx1
        ctx1.memoize(&key, &Cst::Void);

        // 2. Retrieve the result using ctx2
        // This proves that the HeavyState was NOT cloned/isolated.
        let retrieved = ctx2.memo(&key);

        assert!(
            retrieved.is_some(),
            "ctx2 failed to see the memoization entry from ctx1"
        );
        assert_eq!(
            retrieved.unwrap().cst,
            Cst::Void,
            "Memoization data mismatch between shared contexts"
        );
    }

    // #[test]
    // fn test_state_isolation_preserves_shared_cache() {
    //     let grammar = Grammar::default();
    //     let cursor = StrCursor::new("abc");
    //     let mut ctx1 = CoreCtx::new(cursor, &grammar);
    //     let mut ctx2 = ctx1.clone();
    //
    //     // Trigger CoW on ctx2 by mutating volatile state
    //     ctx2.reset(1);
    //
    //     // Verify state is isolated
    //     assert_ne!(ctx1.cursor().mark(), ctx2.cursor().mark());
    //
    //     // Verify cache is STILL shared after CoW trigger
    //     let key = 99;
    //     let entry = MemoEntry::new(1, false);
    //
    //     ctx2.memoize(key, 1, entry.clone());
    //     assert!(
    //         ctx1.memoized(key, 1).is_some(),
    //         "Shared cache link was broken by a Copy-on-Write state split"
    //     );
    // }
}