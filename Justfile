# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

shell := "xonsh"
set shell := [shell, "-c"]

default: pre-push

pre-push: clippy fmt test

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

test:
    cargo nextest run

build:
    cargo build --release

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
