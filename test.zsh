#!/bin/zsh

cargo build

target/debug/xbatis2sql -m -t Oracle -s . -o /tmp -e
grep -F "__INCLUDE_" /tmp/result.sql
mv /tmp/result.sql /tmp/m.o.result.sql
target/debug/xbatis2sql -m -t MySQL -s . -o /tmp -e
grep -F "__INCLUDE_" /tmp/result.sql
mv /tmp/result.sql /tmp/m.m.result.sql
target/debug/xbatis2sql -i -t Oracle -s . -o /tmp -e
grep -F "__INCLUDE_" /tmp/result.sql
mv /tmp/result.sql /tmp/i.o.result.sql
target/debug/xbatis2sql -i -t MySQL -s . -o /tmp -e
grep -F "__INCLUDE_" /tmp/result.sql
mv /tmp/result.sql /tmp/i.m.result.sql

