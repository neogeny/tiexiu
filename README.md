[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/neogeny/TieXiu?utm_source=badge)

# 铁修 TieXiu

A high-performance port of **TatSu** to Rust.

**TieXiu** (铁修) is a PEG (Parsing Expression Grammar) engine that implements the flexibility and power of the original **[TatSu][]** lineage into a memory-safe, high-concurrency architecture optimized for modern CPU caches.

[TatSu]: https://tatsu.readthedocs.io/en/stable/

### Beta

**TieXiu** is functionally complete, and correct respect its predecessor **TatSu**. A _beta_ period will allow for adjusting the API and its signatures to the user experience.

## About

**TieXiu** is a tool that takes grammars in extended `EBNF`_ as input, and
outputs _memoizing_ (_Packrat_) _PEG_ parsers as a Rust model. The classic
variations of EBNF (Tomassetti, EasyExtend, Wirth) and _ISO EBNF_ are
supported as input grammar formats.

The [TatSu Documentation][] provides a vision of where the **TieXiu** project is heading. A copy of the grammar syntax can can be accessed locally in the [SYNTAX](SYNTAX.md) document.

[TatSu Documentation]: https://tatsu.readthedocs.io/

**TieXiu** is foremost a _Rust library_ that is also published as a Python library with the help of _PyO3/Maturin_. The Rust API may return objects of types in the internal parser or tree model. The Python API has strings as input and `json.dumps()` compatible Python objects as output.

**TatSu** is a mature project with an important user base so it's difficult to make certain changes even if they are improvements or fixes for long-standing quirks (as well known within experienced software engineers, a long-lived quirk becomes a feature). **TieXiu** is an opportunity to start from scratch, with a modern approach, even if the grammar syntax and its semantics are preserved.

## On The (Non-Blazing) Speed

**TieXiu** was started with the aim of learning `Rust` and applying AI Agents over a meaningful project (versus books or simple exercises). **TatSu** has rich parser-generator semantics that had to be replicated in `Rust` for completeness and compatibility. 

 Not a primary objective, it was expected that parsing with otpimized `Rust` over its runtime would run circles around the `Python` implementation. 
 
 **It was not so**.

 As if implementing the semantics wasn't difficult enough on a languate so strict about memory management and with so little reasonable and efficient defaults, the first complete runs of parser generation and parsing were `3x` times _slower_ than the best `Python` counterpart.
 
 It took an important amount of _Rust-specific_ optimizations and some algorithm redesign to reach the current `1.08x` speed.`Rust` is not friendly to the deep recursion required to parse a language like, for example, `Java`, and its default data structures, like `Vec`, don't behave well when used as short-lived containers. The complete history of optimizations that include an imported _heap manager_ figure in the `Git` logs.

   The `PyO3` interface is there, but it's easier and more convenient to use **TatSu** directly when working with `Python`.

 **TieXiu** is today a powerful PEG parser generator in `Rust`, so it may find a home among the _rustacean_ community wanting to convert flat streams into semantic structures.

### The Theoretical Ceiling: Why Rust Didn't "Run Circles"

The humble `1.08x` speedup over highly optimized Python isn't a failure of Rust—it is a lesson in computer architecture governed by **Amdahl's Law** and the physical constraints of hardware.

When applied to execution cost, Amdahl's Law dictates that the total time $T$ of a process across an optimization scale $n$ is bound by a fixed, unalterable baseline $b$:

$$T(n) = b + \frac{T(1) - b}{n}$$

In a hyper-optimized PEG engine like TatSu, the algorithmic design is already incredibly refined. The serial baseline $b$ represents the unavoidable physical mechanics of processing a formal language:

1. **The Linear-Scan Lower Bound:** A parser cannot predict a grammar structure without reading the data. Every single byte must travel from RAM, through the cache hierarchy, and into a CPU register.
2. **The Memory Wall:** Once code optimization strips away linguistic bloat (interpreter lookups, heavy object wrapping), the execution becomes entirely *Stream-Bound*. The CPU spends its clock cycles waiting on memory bus bandwidth.

Because TatSu's core execution loop pushes those operations down into optimized C primitives and bitmaps, the remaining runtime overhead left to optimize ($T(1) - b$) is incredibly small.

Rewriting the engine in Rust completely eliminates Python's memory management friction, heavy object boxing, and Garbage Collection pauses, but it cannot bypass the physical limits of silicon. Once an engine achieves true **Mechanical Sympathy** with the underlying hardware, the language it is written in becomes secondary—the physics of the text stream call the shots.



## Non-Features

Most features of **TatSu** are available in **TieXiu**. Some features have not yet been implemented, and a few never will:

* [ ] Generation of synthetic classes from grammar parameters will not be implemented in Rust.
* [ ] Generation of source code with an object model for deifinitions in the grammar may be implemented if a way is found to make the parser or postprocessing bind the Tree output of a parse to the model.
* [ ] Code generation of a parser recently moved in **TatSu** to the loading of a model of the Grammar and using it as parser. Although the generated procedural parser may produce 1.3x increased throughput in Python, supporting generated code is hard and it complicates the internal interfaces. For Rust, **TieXiu** _alreay knows_ how to load _fast_ a Grammar model from **TatSu** JSON. A generated copy of the grammar model constructor could be precompiled by Rust.
* [ ] Parsing of boolean and numeric values happens in **TatSu** through synthetic actions, which call the constructors for those types passing the parsed strings. For **TieXiu** the preferred way of transformig a tree (semantics) is through post-processing (folding), but basic numeric types and booleans could be supported.
* [ ] Semantic actions (transformations) during parse are not implemented. Python is friendly to objects of type `Any`, so semantic actions during parse in **TatSu** can produce a _tree_ of any type. Rust is different, and trying to have structures of an _any_ type is not rustacean. The result of a parse is a well-defined Tree which is a small-enough enum that writing a walker for it is easy, so type transformations can be done in postprocessing by folding. See the `fold` modules in **TieXiu** for examples and useful trait definitions.
* [ ] Interpolation and evaluation of _\`constant\`_ expressions hasn't had any known use cases with **TatSu**. They will not be implemented in **TieXiu** until a use case appears.
* [ ] The `@@include` directive for textual includes was always a bad idea.

[serde_json]: https://docs.rs/serde_json/latest/serde_json/

## API

The needs of most users are met by parsing input with the rules in a grammar and reciving the structure output as a JSON-compatible value. For other use cases, **TieXiu** exposes its internal model and APIs (to be docummented).


## The Python API

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

These functions are available from package `tiexiu`.

```python
def parse(grammar: str, text: str, **kwargs: Any) -> Any
def parse_grammar(grammar: str, **kwargs: Any) -> Any:
def parse_grammar_to_json(grammar: str, **kwargs: Any) -> Any:
def parse_to_json(grammar: str, text: str, **kwargs: Anyt) -> Any:
def pretty(grammar: str, **kwargs: Any) -> str:
def compile_to_json(grammar: str, **kwargs: Any) -> Any:
```

## The Rust API

```rust
pub fn parse_grammar(grammar: &str, cfg: &CfgA) -> Result<Tree>;
pub fn parse_grammar_to_json(grammar: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn parse_grammar_to_json_string(grammar: &str, cfg: &CfgA) -> Result<String>;
pub fn parse_grammar_with<U>(cursor: U, cfg: &CfgA) -> Result<Tree>
pub fn parse_grammar_to_json_with<U>(cursor: U, cfg: &CfgA) -> Result<serde_json::Value>
pub fn compile(grammar: &str, cfg: &CfgA) -> Result<Grammar>;
pub fn compile_to_json(grammar: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn compile_to_json_string(grammar: &str, cfg: &CfgA) -> Result<String>;
pub fn compile_with<U>(cursor: U, cfg: &CfgA) -> Result<Grammar>
pub fn compile_to_json_with<U>(cursor: U, cfg: &CfgA) -> Result<serde_json::Value>
pub fn load(json: &str, _cfg: &CfgA) -> Result<Grammar>;
pub fn load_to_json(json: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn load_tree(json: &str, _cfg: &CfgA) -> Result<Tree>;
pub fn load_tree_to_json(json: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn grammar_pretty(grammar: &str, cfg: &CfgA) -> Result<String>;
pub fn pretty_tree(tree: &Tree, _cfg: &CfgA) -> Result<String>;
pub fn pretty_tree_json(tree: &Tree, _cfg: &CfgA) -> Result<String>;
pub fn parse(grammar: &str, text: &str, cfg: &CfgA) -> Result<Tree>;
pub fn parse_to_json(grammar: &str, text: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn parse_to_json_string(grammar: &str, text: &str, cfg: &CfgA) -> Result<String>;
pub fn parse_input(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<Tree>;
pub fn parse_input_to_json(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn parse_input_to_json_string(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<String>;
```

## Roadmap

The project is functionally complete. Comments about the implementation strategies and possible improvements are now in [RODADMAP](ROADMAP.md).

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
