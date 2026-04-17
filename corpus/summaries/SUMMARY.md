# TieXiu Project Study Summary

## Project Overview
TieXiu is a Rust port of the TatSu PEG (Parsing Expression Grammar) parser generator with Python bindings via PyO3/maturin. It implements the flexibility and power of TatSu in a memory-safe, high-concurrency architecture optimized for modern CPU caches.

## Key Architectural Components
1. **Core Rust Library** (`src/lib.rs`) - Main library structure and PyO3 bindings
2. **API Layer** (`src/api.rs`) - High-level parsing/compiling functions with JSON variants
3. **PEG Parsing Engine** (`src/peg/`) - Grammar model, parser, left recursion support
4. **State Management** (`src/state/`) - Parser context, memoization, tracing
5. **Input Handling** (`src/input/`) - Cursor trait for text/byte streams
6. **Tree Structures** (`src/trees/`) - CST/AST types and manipulation utilities
7. **JSON Import/Export** (`src/json/`) - TatSu JSON grammar import/export
8. **Python Bindings** (`src/python/`) - API wrappers and tree conversion

## Key Technical Features
- 16-byte handle size (optimized for L1 cache)
- 32-byte state changes (efficient CoW allocation)
- Complete PEG parsing engine
- Left recursion support (analysis and runtime)
- Efficient memoization (O(N) packrat complexity)
- Object-safe cursors (flexible input handling)
- Copy-on-write state transitions (lazy backtracking)

## Development Workflow
- Quality gates: `cargo fmt`, `cargo clippy`, `cargo test`
- Dev commands: `cargo build --release`, `maturin develop`, `cargo bench`
- Testing: `cargo test --verbose` with fixtures
- Python bindings require >=3.12, built with maturin

## Current Status (Alpha)
- Core execution engine complete
- Grammar model and parsing engine implemented
- Left recursion support finished
- Advancing toward high-level tree construction
- Bootstrap roadmap for self-hosting capability

TieXiu preserves TatSu's grammar syntax and semantics while providing a modern Rust implementation with Python interoperability.