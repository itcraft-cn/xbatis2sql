#!/bin/zsh

cargo build --release
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target x86_64-apple-darwin

