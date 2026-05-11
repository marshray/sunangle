#!/usr/bin/sh
# vim: set sw=4 et ai :

# Runs various CI-like checks in a convenient way.

unset -v IFS

set -eux

toplevel=$(git rev-parse --show-toplevel)
printf 'cd %s\n' "$toplevel"
cd "$toplevel" || return 10
(set -x; pwd)

cargo check --workspace --all-targets
cargo check --workspace --all-features --lib --target wasm32-unknown-unknown
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --  -D warnings -W clippy::all
cargo test --workspace --all-targets --all-features
cargo test --workspace --doc

trunk build
