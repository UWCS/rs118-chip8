# RS118 CHIP-8

Yet another CHIP-8 Interpreter in Rust, built for use in [rs118](https://rs118.uwcs.co.uk). Provides both a complete emulator, and a library to use a base for building your own.

# Usage

`cargo install rs118-chip8` to install the `chip8` executable. `chip8 <ROM>` will run the rom file provided.

# Building your own

The `rs118-chip8` crate exports the `chip8_base` library, which you can use as a starting point. Add the following to your `Cargo.toml`:

```toml
[dependencies]
rs118-chip8 = "0.1.0"
```

See [the CHIP-8 workshop](https://rs118.uwcs.co.uk/chip8) and [docs.rs](https://docs.rs/crates/rs118-chip8/latest/chip8_base) for details.
