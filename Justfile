# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

self := "TieXiu"
shell := "xonsh"
set shell := [shell, "-c"]

default: check

check: fix clippy fmt test

push: pre-push
    git push

pre-push: clean book fix clippy fmt test

clippy:
    cargo clippy --lib --all-features --fix --allow-dirty -- -D warnings
    cargo clippy --all-targets --all-features -- -D warnings

fix:
    cargo fix --allow-dirty --allow-staged

fmt:
    cargo fmt --all
    cargo fmt --all --check

test: fix fmt clippy
    cargo nextest run --lib --no-fail-fast

build: fix fmt clippy
    cargo build

release_build:
    cargo build --release

book:
    mdbook build docs
    mdbook test docs

clean:
    cargo clean -p {{self}}

bench:
    cargo bench

update:
    cargo update

run:
    cargo run

@shell:
    {{shell}} --version


pytest: pyo3
    uv run pytest -vv

pyo3:
    uv run maturin build

release:
    gh workflow run release.yml -f publish=false

publish:
    gh workflow run release.yml -f publish=true
