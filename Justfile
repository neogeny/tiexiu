# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

self := "TieXiu"
shell := "xonsh"
set shell := ["xonsh", "-c"]

default: check

tools:
    cargo install --locked cargo-nextest
    rustup component add clippy-preview

check: fix clippy fmt test

push: pre-push
    git push

pre-push: clean book fix clippy fmt test

clippy:
    cargo clippy --lib --all-features --fix --allow-dirty -- -D warnings
    cargo clippy --all-targets --all-features -- -D warnings

fix:
    cargo fix --allow-dirty --allow-staged --all-features

fmt:
    cargo fmt --all
    cargo fmt --all --check

test: fix fmt clippy
    cargo nextest run --lib --all-features

test-all: fix fmt clippy
    cargo nextest run --all --all-features

build: fix fmt clippy
    cargo build

build-release:
    cargo build --release

book:
    mdbook build docs
    mdbook test docs

doc:
    cargo doc

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


py-build: build
    uv run maturin build --features pyo3

py-develop:
    uv run maturin develop --features pyo3

py-test: py-develop
    uv run pytest -vv

py-release:
    uv run maturin build --release --features pyo3

py-test-publish: py-release
    gh workflow run release.yml -f publish=false

py-publish: py-release
    gh workflow run release.yml -f publish=true
