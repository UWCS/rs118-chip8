//!`chip8_base` provides everything you need to get started building your own CHIP-8 interpreter.
//! See the documentation for the [`Cpu`][Cpu] trait to get started.

mod interpreter;
pub use {interpreter::Display, interpreter::Interpreter, interpreter::Keys};
