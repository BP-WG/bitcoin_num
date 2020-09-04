[![Status](https://travis-ci.org/rust-bitcoin/bitcoin_num.png?branch=master)](https://travis-ci.org/rust-bitcoin/bitcoin_num)

# Bitcoin Numeric Library

This is a simple, no-dependency library which implements utility functions
for working with big integers, hex encodings etc needed by Bitcoin. It
differs from other existing cragtes in the field in it's focus on
supporting the same versions of Rust compilers as `bitcoin` and
`bitcoin_hashes` crates and not using any external dependencies in order
to reduce potential attack surface.

[Documentation](https://docs.rs/bitcoin_num/)

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on 
**Rust 1.29**.


## Contributions

Contributions are welcome, including additional hash function implementations.
