#!/usr/bin/env sh

cargo clippy --release
echo "==TODO=="
grep --color=auto -r "TODO" src
