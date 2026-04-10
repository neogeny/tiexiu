# AGENTS.md

## Project Overview

TieXiu is a Rust port of TatSu PEG parser generator with Python bindings via PyO3/maturin.

## Rules

- AI Agents will study `README.md`, `SYNTAX.md`, `./tatsu/README.rst` and all the `*.rst` documents in `./tatsu/docs/` to gain context about the project.
- AI Agents will study all the files belonging to the current Rust project to understand its structure and semantics.
- AI Agents will study the Python source code for TatSu found in the modules under `./tatsu/tatsu/` and `./tatsu/tests/` to understand the background of the TieXiu project and its goals. 
- AI Agents should study `./corpus/SESSIONS.md` at the start of each session to learn from past work.

## Dev Commands

```bash
# Quality gates (used by `just pre-push`)
cargo fmt --all                          # format check
cargo clippy --all-targets --all-features -- -D warnings  # strict lint
cargo test --verbose                     # tests

# All checks in order (default target)
just pre-push

# Other commands
cargo build --release                    # release build
cargo bench                              # benchmarks
cargo run                                # run CLI (requires cli feature)
maturin develop                           # build & install Python package
```

## Testing

- Tests use `cargo test --verbose` (not `cargo nextest` locally; CI uses default)
- Benchmark harness is criterion-based with codspeed compat
- Test fixtures in `tests/fixtures.rs` provide `boot_grammar()` and `parse_ebnf()`

## Python Bindings

- Build system: maturin (configured in `pyproject.toml`)
- `.cargo/config.toml` sets `PYO3_PYTHON` to `.venv/bin/python` for local venv
- Create venv: `uv venv`
- Install package: `maturin develop`

## Architecture

- `src/api.rs`: High-level API (`parse`, `compile`, `load`, `pretty`)
- `src/peg/`: Grammar model (Exp, Grammar, Rule)
- `src/state/`: Parser context and memoization
- `src/input/`: Cursor trait for text/byte streams
- `src/trees/`: CST/AST tree types
- `src/json/`: TatSu JSON grammar import

## Key Conventions

- Rust edition 2024
- Features: `default`, `bootstrap`, `transient`, `serde`, `serde_json`, `cli`, `fancy-regex`
- Python requires >=3.12
- Use `unindent` utility when inlining EBNF strings in tests

## CI Workflows

- `default.yml`: fmt → clippy → test (sequential)
- `codspeed.yml`: benchmark on main/PR
- `publish.yml`: PyPI release
