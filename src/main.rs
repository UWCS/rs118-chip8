mod cpu;
mod interpreter;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let cpu = cpu::Cpu::new(500)
        .load("roms/chip8-test-rom-with-audio.ch8")
        .context("Could not load ROM!")?;
    interpreter::Cpu::run(cpu);
}
