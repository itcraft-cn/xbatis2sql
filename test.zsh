#!/bin/zsh

target/debug/xbatis2sql -m -t Oracle -s . -o /tmp
grep -F "__INCLUDE_" /tmp/result.sql
target/debug/xbatis2sql -i -t Oracle -s . -o /tmp
grep -F "__INCLUDE_" /tmp/result.sql

