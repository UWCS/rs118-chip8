//!`chip8-base` provides everything you need to get started building your own CHIP-8 interpreter.
//! See the documentation for the [`Interpreter`][Interpreter] trait to get started.

mod interpreter;

/// CHIP-8 displays are black and white, so each pixel can be in only one of two states.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Pixel {
    #[default]
    Black = 0,
    White = 1,
}

/// The Interpreter's representation of the CHIP-8 display.
/// The display is 64x32 pixels, each pixel being either Black or White.
pub type Display = [[Pixel; 64]; 32];

/// This type is how keyboard input is presented to the Interpreter.
/// Each of the 16 keys can either be down (`true`) or up (`false`).
pub type Keys = [bool; 16];

/// CHIP-8 interpreters can be built using this trait.
/// [`step`][Interpreter::step] should be implemented on a type representing a CHIP-8 Interpreter to run the interpreter one clock cycle at a time, such that calling it in a loop runs the interpreter.
pub trait Interpreter {
    /// Executes the next CHIP-8 Instruction, modifying the state of the virtual machine/CPU accordingly.
    /// This is the main driver function, running the interpreter one clock cycle at a time.
    /// # Return
    /// If the instruction modified the state of the display, then an updated [`Display`][Display] should be returned.
    /// # Panics
    /// Should panic if an unrecognised instruction is encountered
    fn step(&mut self, keys: &Keys) -> Option<Display>;

    /// Returns the duration of a single clock cycle, so the interpreter can keep the time steps uniform.
    /// See [`std::time`][std::time] for more information on [`Duration`][std::time::Duration].
    fn speed(&self) -> std::time::Duration;

    /// Indicates if the sound buzzer is currently active, such that the interpreter can handle sound accordingly.
    fn buzzer_active(&self) -> bool;
}

pub use interpreter::run;
