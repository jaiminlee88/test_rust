#!/bin/bash
for toml in $(find . -name Cargo.toml); do
    dir=$(dirname "$toml")
    echo "Cleaning $dir"
    (cd "$dir" && cargo clean)
done