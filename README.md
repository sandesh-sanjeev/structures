# Structures

## Array

## Coverage

Run tests and generate coverage data.

```bash
cargo clean
cargo install grcov
rustup component add llvm-tools
export RUSTFLAGS="-Cinstrument-coverage" 
cargo build
export LLVM_PROFILE_FILE="structures-%p-%m.profraw" 
cargo test
mkdir ./target/debug/coverage
grcov . -s . --binary-path ./target/debug/ -t html --branch -o ./target/debug/coverage/ --llvm --ignore-not-existing
grcov . -s . --binary-path ./target/debug/ -t lcov --branch -o ./target/debug/coverage/lcov.info --llvm --ignore-not-existing
rm *.profraw
unset RUSTFLAGS
unset LLVM_PROFILE_FILE
```

## Miri

Run Miri interpreter to check for undefined behavior.

```bash
cargo clean
rustup +nightly component add miri
rustup override set nightly
MIRIFLAGS=-Zmiri-disable-isolation cargo miri test
rustup override remove
```
