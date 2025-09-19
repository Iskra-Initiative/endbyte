# endbyte

[![Crates.io](https://img.shields.io/crates/v/endbyte.svg)](https://crates.io/crates/endbyte)
[![Documentation](https://docs.rs/endbyte/badge.svg)](https://docs.rs/endbyte)
[![Build Status](https://github.com/Iskra-Initiative/endbyte/actions/workflows/rust.yml/badge.svg)](https://github.com/Iskra-Initiative/endbyte/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![no_std](https://img.shields.io/badge/no__std-compatible-green.svg)](https://docs.rust-embedded.org/book/intro/no-std.html)

a `no_std` compatible crate for handling byte order conversions between different endianness formats.

## features

- **no_std compatible**: usable in embedded environments
- **zero dependencies**: no external crates required
- **comprehensive type support**: supports integer types (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`)
- **efficient**: uses rust's built-in `swap_bytes()` methods for optimal performance
- **compile-time endianness detection**: zero runtime overhead for endianness checks

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

this library is designed to also work in no_std environments:

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

> note: single-byte types (`u8`, `i8`) have zero-cost implementations since byte swapping is not needed.

## methods

- `host_to_big_endian()`: convert from host byte order to big endian
- `host_to_little_endian()`: convert from host byte order to little endian
- `big_endian_to_host()`: convert from big endian to host byte order
- `little_endian_to_host()`: convert from little endian to host byte order

## testing

the library includes tests that work on both big and little endian systems:

```bash
cargo test
```

for embedded targets:

```bash
cargo build --target thumbv7em-none-eabihf
cargo build --target thumbv6m-none-eabi
```

## contribute

contributions are welcome! please open an issue or pull request if you have any improvements or bug fixes.

## license

this project is licensed under the mit license - see the license file for details.
