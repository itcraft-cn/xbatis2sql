#!/bin/zsh

cargo build --release
cargo build --release --target x86_64-unknown-linux-musl

