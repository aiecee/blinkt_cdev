# blinkt_cdev

[![crates.io](https://meritbadge.herokuapp.com/blinkt_cdev)](https://crates.io/crates/blinkt_cdev)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-v1.39.0-lightgray.svg)](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html)

blinkt_cdev is a Rust library that allows you to control [Pimoroni Blinkt!](https://shop.pimoroni.com/products/blinkt) on the Raspberry Pi. This borrows heavily from the awesome [Blinkt](https://github.com/golemparts/blinkt) by golemparts. The difference is that this uses the Rust Embedded [gpio-cdev](https://github.com/rust-embedded/gpio-cdev) library.

## Documentation

- Latest Release: [docs.rs/blinkt_cdev/](https://docs.rs/blinkt_cdev/)

## Usage

Add dependency `blinkt_cdev` to your `Cargo.toml`

```toml
[dependencies]
blinkt_cdev = "0.1.0"
```

```rust
use blinkt_cdev::*;

let mut blinkt = Blinkt::new()?;
blinkt.set_all_pixels(255, 0, 0, 1.0);
blinkt.show()?;
```

## Examples

To run an example use the `cargo run --example [name]` command.

Current examples:

- `cargo run --example fade`
