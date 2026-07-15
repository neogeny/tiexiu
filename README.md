[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/neogeny/TieXiu?utm_source=badge)

# 铁修 TieXiu

A high-performance port of **TatSu** to Rust.

**TieXiu** (铁修) is a PEG (Parsing Expression Grammar) engine that implements the flexibility and power of the original **[TatSu][]** lineage into a memory-safe, high-concurrency architecture optimized for modern CPU caches.

[TatSu]: https://tatsu.readthedocs.io/en/stable/

## About

**TieXiu** is a tool that takes grammars in extended `EBNF`_ as input, and
outputs _memoizing_ (_Packrat_) _PEG_ parsers as a Rust model. The classic
variations of EBNF (Tomassetti, EasyExtend, Wirth) and _ISO EBNF_ are
supported as input grammar formats.

The [TatSu Documentation][] provides a vision of where the **TieXiu** project is heading. A copy of the grammar syntax can be accessed locally in the [SYNTAX](SYNTAX.md) document.

[TatSu Documentation]: https://tatsu.readthedocs.io/

**TieXiu** is a _Rust library_ published as a Python library with _PyO3/Maturin_. The Rust API exposes the internal parser and tree model directly. The Python API has strings as input and `json.dumps()` compatible Python objects as output.

**TatSu** is a mature project with an important user base. It's difficult to make certain changes to it even if they are improvements or fixes for long-standing quirks. **TieXiu** preserves the grammar syntax and semantics while allowing the implementation to evolve freely.

## Performance

**TieXiu** runs at the hardware-bound performance ceiling for PEG parsing. The engine achieves **Mechanical Sympathy** — the CPU spends its cycles matching input bytes against grammar rules, not fighting the language or runtime.

When applied to execution cost, **Amdahl's Law** dictates that the total time $T$ of a process across an optimization scale $n$ is bound by a fixed, unalterable baseline $b$:

$$T(n) = b + \frac{T(1) - b}{n}$$

In an optimized PEG engine, the algorithmic design is already refined. The serial baseline $b$ represents the unavoidable physical mechanics of processing a formal language:

1. **The Linear-Scan Lower Bound:** A parser cannot predict the input structure without scanning _all_ the input. Every single byte must travel from RAM, through the cache hierarchy, and into a CPU register, and _it must be matched_ against the grammar.
2. **The Memory Wall:** Once code optimization strips away linguistic bloat, the execution becomes entirely *Stream-Bound*. The CPU spends its clock cycles waiting on memory bus bandwidth, or moving through the states of a regexp automaton.

Because **TatSu**'s core execution loop pushes those operations down into optimized C primitives and bitmaps, the remaining runtime overhead left to optimize ($T(1) - b$) is incredibly small.

Rewriting the engine in Rust completely eliminates Python's memory management friction, heavy object boxing, and Garbage Collection pauses, but it cannot bypass the physical limits of silicon. Once an engine achieves true Mechanical Sympathy with the underlying hardware, the language it is written in becomes secondary — the physics of the text stream call the shots.

The humble `1.08x` speedup over **TatSu** is not a limitation — it is proof that both engines have reached the same silicon ceiling. **[OgoPEGo][]** is a brand-new implementation of the semantics in `Go` that independently reached the same asymptotic bound, confirming that the bottleneck is the hardware, not the language.

[OgoPEGo]: https://github.com/neogeny/ogopego

The journey from an initial `3x` slowdown to `1.08x` required Rust-specific optimizations: algorithm redesign for deep recursion, careful short-lived container management, and removing unnecessary allocations. The complete history of optimizations is documented in the `Git` logs.

The `PyO3` interface is available, but for Python-only workflows **TatSu** is more convenient. **TieXiu**'s strength is as a Rust library or CLI tool.

## Non-Features

Most features of **TatSu** are available in **TieXiu**. Some features have not yet been implemented, and a few never will:

* [ ] Generation of synthetic classes from grammar parameters will not be implemented in Rust.
* [ ] Generation of source code with an object model for definitions in the grammar may be implemented if a way is found to make the parser or postprocessing bind the Tree output of a parse to the model.
* [ ] Code generation of a parser recently moved in **TatSu** to the loading of a model of the Grammar and using it as parser. Although the generated procedural parser may produce 1.3x increased throughput in Python, supporting generated code is hard, and it complicates the internal interfaces. For Rust, **TieXiu** _already knows_ how to load _fast_ a Grammar model from **TatSu** JSON. A generated copy of the grammar model constructor could be precompiled by Rust.
* [ ] Parsing of boolean and numeric values happens in **TatSu** through synthetic actions, which call the constructors for those types passing the parsed strings. For **TieXiu** the preferred way of transforming a tree (semantics) is through post-processing (folding), but basic numeric types and booleans could be supported.
* [ ] Semantic actions (transformations) during parse are not implemented. Python is friendly to objects of type `Any`, so semantic actions during parse in **TatSu** can produce a _tree_ of any type. Rust is different, and trying to have structures of an _any_ type is not rustacean. The result of a parse is a well-defined Tree which is a small-enough enum that writing a walker for it is easy, so type transformations can be done in postprocessing by folding. See the `fold` modules in **TieXiu** for examples and useful trait definitions.
* [ ] Interpolation and evaluation of _\`constant\`_ expressions hasn't had any known use cases with **TatSu**. They will not be implemented in **TieXiu** until a use case appears.
* [ ] The `@@include` directive for textual includes was always a bad idea.

## CLI

The CLI exercises everything currently implemented and is the best starting place to learn about the library.

```
tiexiu run <grammar> <inputs...>     # Parse files with a grammar
tiexiu boot --pretty                 # Pretty-print the boot grammar
tiexiu grammar <grammar> --json      # Grammar transformations
```

The `run` command supports concurrent parsing (`-n`), JSON output (`-j`), and tracing (`--trace`). The full help is available with `tiexiu --help`.

## API

The needs of most users are met by parsing input with the rules in a grammar and receiving the structure output as a JSON-compatible value. For other use cases, **TieXiu** exposes its internal model and APIs.

### The Rust API

```rust
pub fn pegapi() -> TieXiu;
pub fn parse_grammar(grammar: &str, cfg: &CfgA) -> Result<Tree>;
pub fn parse_grammar_with<U>(cursor: U, cfg: &CfgA) -> Result<Tree>;
pub fn compile(grammar: &str, cfg: &CfgA) -> Result<Grammar>;
pub fn compile_with<U>(cursor: U, cfg: &CfgA) -> Result<Grammar>;
pub fn load_grammar_from_json(json: &str) -> Result<Grammar>;
pub fn load_tree_from_json(json: &str) -> Result<Tree>;
pub fn grammar_pretty(grammar: &str, cfg: &CfgA) -> Result<String>;
pub fn parse(grammar: &str, text: &str, cfg: &CfgA) -> Result<Tree>;
pub fn parse_input(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<Tree>;
pub fn boot_grammar() -> Result<Grammar>;
pub fn load_boot() -> Result<Grammar>;
pub fn boot_grammar_pretty() -> Result<String>;
```

The `TieXiu` struct (available via `pegapi()`) provides an object-oriented API with grammar caching and `&self` method signatures:

```rust
let tx = pegapi();
let grammar = tx.compile("start: /hello/")?;
let tree = tx.parse("start: /hello/", "hello")?;
```

### The Python API

The return values of `Any` are of the basic Python types, as defined in the `json` module documentation (see [Encoders and Decoders][] ).

[Encoders and Decoders]: https://docs.python.org/3/library/json.html#json-to-py-table

| JSON          | Python |
|---------------|--------|
| object        | dict   |
| array         | list   |
| string        | str    |
| number (int)  | int    |
| number (real) | float  |
| true          | True   |
| false         | False  |
| null          | None   |

Keyword arguments can be passed for runtime configuration. The only recognized argument as of writing is `trace=`.

#### OO API (recommended)

```python
from tiexiu import pegapi

tx = pegapi()
tree = tx.parse("start: /hello/", "hello")
json_tree = tx.parse_to_json("start: /hello/", "hello")
grammar = tx.compile("start: /hello/")
```

The `TieXiu` instance caches compiled grammars. Use `get(grammar)` to check the cache, or `get_or_compile(grammar)` to compile on cache miss.

#### Functional API

```python
from tiexiu import parse, compile, pretty, boot_grammar

tree = parse("start: /hello/", "hello")
json_tree = parse_to_json("start: /hello/", "hello")
grammar = compile("start: /hello/")
```

All parse and compile functions have `_to_json` and `_to_json_string` variants that return JSON-compatible Python objects or strings directly.

#### Compiled Grammar

```python
grammar = compile("start: /hello/")
tree = grammar.parse("hello")
```

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
