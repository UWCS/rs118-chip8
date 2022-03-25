# RS118 CHIP-8

Yet another CHIP-8 Interpreter in Rust, built for use in [RS118][https://rs118.uwcs.co.uk]. Provides both a complete emulator, and a library to use a base for building your own.

# Usage

`cargo install RS118-CHIP8` to install the `chip-8` executable.

# Building your own

Add the following to your `Cargo.toml`:

```toml
[dependencies]
`chip8_base` = "0.1.0"
```

See [the CHIP-8 workshop](https://rs118.uwcs.co.uk) and [docs.rs](https://docs.rs/crates/RS118-CHIP8/latest/chip8_base) for details.
