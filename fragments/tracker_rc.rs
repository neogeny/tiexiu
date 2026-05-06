// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::any::type_name;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TrackedFuse<T> {
    // The shared counter for this specific instance family
    counter: Rc<()>,
    _marker: PhantomData<T>,
}

impl<T> Default for TrackedFuse<T> {
    fn default() -> Self {
        let this = Self {
            counter: Rc::new(()),
            _marker: PhantomData,
        };
        println!(
            "[CREATE] {} | Refs: {}",
            type_name::<T>(),
            this.ref_count()
        );
        this
    }
}

impl<T> TrackedFuse<T> {
    /// Returns the number of active clones of this specific object
    pub fn ref_count(&self) -> usize {
        Rc::strong_count(&self.counter)
    }
}

impl<T> Drop for TrackedFuse<T> {
    fn drop(&mut self) {
        // We subtract 1 because self is about to be destroyed,
        // showing the remaining count after this drop completes.
        let remaining = self.ref_count() - 1;
        println!(
            "[DROP] {} | Remaining Refs: {}",
            type_name::<T>(),
            remaining
        );
    }
}
