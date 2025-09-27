# Structures

This crate provides different types of heap allocated data structures.

[![Build Status][build-img]][build-url]
[![Documentation][doc-img]][doc-url]

[build-img]: https://github.com/sandesh-sanjeev/structures/actions/workflows/ci.yml/badge.svg?branch=master
[build-url]: https://github.com/sandesh-sanjeev/structures/actions/workflows/ci.yml
[doc-img]: https://img.shields.io/badge/docs.rs-aoc-4d76ae?style=for-the-badge
[doc-url]: https://sandesh-sanjeev.github.io/structures/structures/index.html

## Tests

Run unit and doc tests.

```bash
# Run all tests.
$ cargo test
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
