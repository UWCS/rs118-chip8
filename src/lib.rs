//!`chip8_base` provides everything you need to get started building your own CHIP-8 interpreter.
//! See the documentation for the [`Interpreter`][Interpreter] trait to get started.

mod interpreter;
pub use {interpreter::run, interpreter::Display, interpreter::Interpreter, interpreter::Keys};
