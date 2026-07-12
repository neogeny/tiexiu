---
type: Architecture
title: Performance
description: Hardware-bound PEG parsing — Amdahl's Law and Mechanical Sympathy.
tags: [performance, amdahl, hardware, optimization]
timestamp: 2026-07-12T00:00:00Z
---

# Performance

TieXiu runs at the hardware-bound performance ceiling for PEG parsing. The engine achieves **Mechanical Sympathy** — the CPU spends its cycles matching input bytes against grammar rules, not fighting the language or runtime.

## Amdahl's Law

When applied to execution cost, Amdahl's Law dictates that the total time $T$ of a process across an optimization scale $n$ is bound by a fixed, unalterable baseline $b$:

$$T(n) = b + \frac{T(1) - b}{n}$$

In an optimized PEG engine, the algorithmic design is already refined. The serial baseline $b$ represents the unavoidable physical mechanics of processing a formal language.

## The Hardware Ceiling

### Linear-Scan Lower Bound

A parser cannot predict the input structure without scanning _all_ the input. Every single byte must travel from RAM, through the cache hierarchy, and into a CPU register, and _it must be matched_ against the grammar.

### The Memory Wall

Once code optimization strips away linguistic bloat, the execution becomes entirely *Stream-Bound*. The CPU spends its clock cycles waiting on memory bus bandwidth, or moving through the states of a regexp automaton.

## Cross-Language Confirmation

Because TatSu's core execution loop pushes those operations down into optimized C primitives and bitmaps, the remaining runtime overhead left to optimize ($T(1) - b$) is incredibly small.

Rewriting the engine in Rust completely eliminates Python's memory management friction, heavy object boxing, and Garbage Collection pauses, but it cannot bypass the physical limits of silicon. Once an engine achieves true Mechanical Sympathy with the underlying hardware, the language it is written in becomes secondary — the physics of the text stream call the shots.

The humble `1.08x` speedup over TatSu is not a limitation — it is proof that both engines have reached the same silicon ceiling. [OgoPEGo](https://github.com/neogeny/ogopego) is a brand-new implementation of the semantics in Go that independently reached the same asymptotic bound, confirming that the bottleneck is the hardware, not the language.

## Optimization History

The journey from an initial `3x` slowdown to `1.08x` required Rust-specific optimizations:

- Algorithm redesign for deep recursion
- Careful short-lived container management
- Removing unnecessary allocations
- NullTracer hot-path override (eliminate String allocations when tracing is off)
- FlagMap to u8 bitfield (6 hash lookups → 6 bitwise AND ops)
- TreeMapBuilder (eliminate O(n²) insert pattern)
- Tree merge accumulator pattern (eliminate O(n²) fold)
- Arc clone removal in parsing hot path
- Single-pass `pos_at()` in cursor

The complete history is documented in the Git logs.

## Citations

[1] Amdahl, G. M. (1967). "Validity of the single processor approach to achieving large scale computing capabilities."
[2] [OgoPEGo](https://github.com/neogeny/ogopego) — Go implementation confirming the same asymptotic bound.
