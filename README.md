# Structures

This crate provides different types of heap allocated data structures.

[![Build](https://github.com/sandesh-sanjeev/structures/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/sandesh-sanjeev/structures/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/sandesh-sanjeev/structures/badge.svg?branch=master)](https://coveralls.io/github/sandesh-sanjeev/structures?branch=master)

## Array

## Coverage

Run tests and generate coverage data.

```bash
cargo clean
cargo install grcov
rustup component add llvm-tools
export RUSTFLAGS="-Cinstrument-coverage" 
cargo build
mkdir ./target/debug/coverage
export LLVM_PROFILE_FILE="./target/debug/coverage/structures-%p-%m.profraw" 
cargo test
grcov ./target/debug/coverage/ -s . --binary-path ./target/debug/ -t html --branch -o ./target/debug/coverage/ --llvm --ignore-not-existing
grcov ./target/debug/coverage/ -s . --binary-path ./target/debug/ -t lcov --branch -o ./target/debug/coverage/lcov.info --llvm --ignore-not-existing
rm ./target/debug/coverage/*.profraw
unset RUSTFLAGS
unset LLVM_PROFILE_FILE
```

## Miri

Run Miri interpreter with tests to check for undefined behavior.

```bash
cargo clean
rustup +nightly component add miri
rustup override set nightly
MIRIFLAGS=-Zmiri-disable-isolation cargo miri test
rustup override remove
```
