#!/bin/sh
set -e

# fail fast if we won't pass a simple cargo check
cargo check --all-targets

# run the unit tests
cargo test

# run integration tests
just test

# run benchmarks
just bench-all

# build the cli
just build-cli
