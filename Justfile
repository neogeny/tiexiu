# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

shell := "xonsh"
set shell := [shell, "-c"]

default: check

check: clippy fmt test

push: pre-push
    git push

pre-push: clean book clippy fmt test

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

test:
    cargo nextest run

build:
    cargo build --release

book:
    mdbook build docs
    mdbook test docs

clean:
    cargo clean

bench:
    cargo bench

update:
    cargo update

run:
    cargo run

@shell:
    {{shell}} --version
