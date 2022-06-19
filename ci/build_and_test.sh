#!/bin/bash

set -euo pipefail

export RUSTFLAGS="-D warnings"

# Print version information
rustc -Vv
cargo -V

# Build and test main crate
cargo build --locked
cargo test

# Build and test other crates
cd config_proc_macro
cargo build --locked
cargo test
