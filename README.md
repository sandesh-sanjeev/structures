# Structures

This crate provides different types of heap allocated data structures.

[![Build](https://github.com/sandesh-sanjeev/structures/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/sandesh-sanjeev/structures/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/sandesh-sanjeev/structures/badge.svg?branch=master)](https://coveralls.io/github/sandesh-sanjeev/structures?branch=master)

## Array

## Tests

Run tests.

```bash
$ cargo test
```

### Coverage

Run tests and gather coverage data.

```bash
# Install tarpaulin
$ cargo install cargo-tarpaulin

# Run tests with coverage.
$ cargo tarpaulin
```

### Miri

Run Miri interpreter with tests to check for undefined behavior.

```bash
# Install Miri on nightly rust
$ rustup +nightly component add miri

# Override workspace to nightly
$ rustup override set nightly

# Run miri on tests
$ MIRIFLAGS=-Zmiri-disable-isolation cargo miri test

# Remove workspace override.
$ rustup override remove
```
