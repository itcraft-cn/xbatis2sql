#!/bin/zsh

target/debug/xbatis2sql -m -t Oracle -s . -o /tmp
grep -F "__INCLUDE_" /tmp/result.sql
mv /tmp/result.sql /tmp/m.result.sql
target/debug/xbatis2sql -i -t Oracle -s . -o /tmp
grep -F "__INCLUDE_" /tmp/result.sql
mv /tmp/result.sql /tmp/i.result.sql

