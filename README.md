[![Crates.io](https://img.shields.io/crates/d/rs118-chip8?style=flat-square)](https://crates.io/crates/rs118-chip8)
[![docs.rs](https://img.shields.io/docsrs/chip8_base/latest?style=flat-square)](https://docs.rs/chip8_base/latest/chip8_base/)
# RS118 CHIP-8

Yet another CHIP-8 Interpreter in Rust, built for use in [rs118](https://rs118.uwcs.co.uk). Provides both a complete emulator, and a library to use a base for building your own.

# Usage

`cargo install rs118-chip8` to install the `chip8` executable. `chip8 <ROM>` will run the rom file provided.

# Building your own

The `chip8_base` crate library is designed for use as a starting point. Add the following to your `Cargo.toml`:

```toml
[dependencies]
chip8_base = "0.2"
```

See [the CHIP-8 workshop](https://rs118.uwcs.co.uk/chip8.html) and [docs.rs](https://docs.rs/rs118-chip8/latest/chip8_base/) for details.
