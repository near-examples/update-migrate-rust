#!/bin/sh

echo ">> Building contracts"

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release
