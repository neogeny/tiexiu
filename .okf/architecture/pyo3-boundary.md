---
type: Architecture
title: PyO3 Boundary
description: Architecture for crossing the Rust-Python boundary with PyO3.
tags: [pyo3, python, ffi, architecture]
timestamp: 2026-07-12T00:00:00Z
---

# PyO3 Boundary

Architecture for the context, caching, and PyO3 boundary in TieXiu.

## Core Library: The Context Pattern

Instead of a global singleton, the library uses a `GrammarContext`. This struct acts as the owner of the grammar cache and the primary entry point for the parsing tools.

Key benefits:

- **No Global Locks:** Eliminates the need for a static Mutex, preventing bottlenecks.
- **Resource Lifecycle:** The cache is dropped automatically when the context goes out of scope.
- **Internal Simplicity:** By moving away from global statics, internal structures can use `Box<T>` instead of `Arc<T>`, provided they aren't shared across threads manually.

## Shared Ownership with Arc::make_mut

When the library requires shared ownership (e.g., within a multi-threaded parser), Arc is used with a Copy-on-Write (CoW) strategy.

`Arc::make_mut(&mut x)` provides mutable access to the inner data:

- **Unique Owner:** Returns a mutable reference directly (zero-cost).
- **Shared Owner:** Silently clones the data before returning the reference, ensuring the original remains immutable.

## Crossing the PyO3 Boundary

To expose the Rust `GrammarContext` to Python, a wrapper pattern leverages Python's garbage collector as the primary owner.

### Wrapper Pattern

```rust
#[pyclass]
pub struct PyGrammarContext {
    pub inner: GrammarContext,
}

#[pymethods]
impl PyGrammarContext {
    #[new]
    fn new() -> Self {
        PyGrammarContext { inner: GrammarContext::new() }
    }

    fn compile(&mut self, text: &str) -> PyResult<String> {
        let grammar = self.inner.get_or_compile(text);
        Ok(grammar.name.clone())
    }
}
```

### Passing Handles Back to Rust

When Python passes a `PyGrammarContext` back to a Rust function, PyO3 performs a zero-copy borrow using the GIL for synchronization:

```rust
#[pyfunction]
fn run_parser(context_handle: PyRefMut<'_, PyGrammarContext>, input: &str) {
    let context = &mut context_handle.inner;
    context.parse(input);
}
```

## Implementation Checklist

1. **Purge Rc and RefCell:** Ensure all internal types are `Send + Sync` (using `Arc` and `Mutex` where necessary) to allow them to live inside the Python-managed heap.
2. **Handle-Based API:** Ensure both the UI and Python modules instantiate a Context rather than reaching for a global variable.
3. **Feature Flags:** Use `Cargo.toml` to make `pyo3` and `clap` optional dependencies to keep the core library lean.
