# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

self := "TieXiu"
shell := "xonsh"
set shell := ["xonsh", "-c"]

default: check

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
    maturin build

build-release:
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


pyo3: build
    uv run maturin build --features pyo3

pyo3-develop:
    uv run maturin develop --features pyo3

pytest: pyo3-develop
    uv run pytest -vv

pyo3-release:
    uv run maturin build --release --features pyo3

release: pyo3-release
    gh workflow run release.yml -f publish=false

publish: pyo3-release
    gh workflow run release.yml -f publish=true
