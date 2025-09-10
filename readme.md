# endbyte

[![Crates.io](https://img.shields.io/crates/v/endbyte.svg)](https://crates.io/crates/endbyte)
[![Documentation](https://docs.rs/endbyte/badge.svg)](https://docs.rs/endbyte)
[![Build Status](https://github.com/Iskra-Initiative/endbyte/actions/workflows/rust.yml/badge.svg)](https://github.com/Iskra-Initiative/endbyte/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![no_std](https://img.shields.io/badge/no__std-compatible-green.svg)](https://docs.rust-embedded.org/book/intro/no-std.html)

a no_std compatible rust library for handling byte order conversions between different endianness formats.

## features

- **no_std compatible**: works in embedded environments without the standard library
- **zero dependencies**: no external crates required
- **comprehensive type support**: supports all standard integer types (u8, u16, u32, u64, u128, i8, i16, i32, i64, i128)
- **efficient**: uses rust's built-in `swap_bytes()` methods for optimal performance
- **compile-time endianness detection**: zero runtime overhead for endianness checks
- **well tested**: comprehensive test suite including embedded target testing

## usage

add this to your `Cargo.toml`:

```toml
[dependencies]
endbyte = "0.1.0"
```

### basic usage

```rust
use endbyte::Endianness;

let value = 0x1234u16;

// convert to specific endianness
let big_endian = value.host_to_big_endian();
let little_endian = value.host_to_little_endian();

// convert from specific endianness back to host
let from_big = big_endian.big_endian_to_host();
let from_little = little_endian.little_endian_to_host();

assert_eq!(from_big, value);
assert_eq!(from_little, value);
```

### embedded usage

this library is designed to work in no_std environments:

```rust,ignore
#![no_std]

use endbyte::Endianness;

fn process_network_data(data: u32) -> u32 {
    // convert from network byte order (big endian) to host
    let host_value = data.big_endian_to_host();

    // process the value...
    let result = host_value * 2;

    // convert back to network byte order
    result.host_to_big_endian()
}
```

## supported types

the `Endianness` trait is implemented for all standard integer types:

- unsigned: `u8`, `u16`, `u32`, `u64`, `u128`
- signed: `i8`, `i16`, `i32`, `i64`, `i128`

note: single-byte types (`u8`, `i8`) have zero-cost implementations since byte swapping is not needed.

## methods

- `host_to_big_endian()`: convert from host byte order to big endian
- `host_to_little_endian()`: convert from host byte order to little endian
- `big_endian_to_host()`: convert from big endian to host byte order
- `little_endian_to_host()`: convert from little endian to host byte order

## performance

this library is designed for maximum performance:

- uses rust's built-in `swap_bytes()` methods which compile to optimal assembly
- compile-time endianness detection means zero runtime overhead
- single-byte types have zero-cost implementations
- all methods are marked `#[inline]` for optimal inlining

## testing

the library includes comprehensive tests that work on both big and little endian systems:

```bash
cargo test
```

for embedded targets:

```bash
cargo build --target thumbv7em-none-eabihf
cargo build --target thumbv6m-none-eabi
```

## releases

this crate uses automated publishing to crates.io via github actions. to create a new release:

1. update the version in `Cargo.toml`
2. commit the changes: `git commit -am "bump version to x.y.z"`
3. create and push a git tag: `git tag vx.y.z && git push origin vx.y.z`
4. the github action will automatically run tests and publish to crates.io

the automated workflow ensures that:
- the git tag version matches the `Cargo.toml` version
- all tests pass (including no_std compatibility tests)
- code formatting and clippy checks pass
- documentation builds successfully

## license

this project is licensed under the mit license - see the license file for details.
