# sw-align-rs

[Smith-Waterman alignment](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm) algorithm implemented in Rust for DNA sequences.

## Build

Build the tool by running:

```
cargo build --release
```

This should create `sw-align-rs` in the `target/release` directory.

## Run

Run the tool by running

```
./target/release/sw-align-rs TGTTACGG GGTTGACTA
```

You should see the following:

```
GTT-AC
GTTGAC
```
