#!/bin/zsh

rm -rf /tmp/xbatis2sql.7z
rm -rf /tmp/xbatis2sql

mkdir -p /tmp/xbatis2sql/linux_x64
mkdir -p /tmp/xbatis2sql/linux_x64_musl
mkdir -p /tmp/xbatis2sql/darwin_intel

cargo build --release
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target x86_64-apple-darwin

cp target/release/xbatis2sql /tmp/xbatis2sql/linux_x64/xbatis2sql
cp target/x86_64-unknown-linux-musl/release/xbatis2sql /tmp/xbatis2sql/linux_x64_musl/xbatis2sql
cp target/x86_64-apple-darwin/release/xbatis2sql /tmp/xbatis2sql/darwin_intel/xbatis2sql

7zz a -sdel /tmp/xbatis2sql.7z /tmp/xbatis2sql

