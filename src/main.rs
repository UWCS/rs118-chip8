mod cpu;
mod vm;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let cpu = cpu::Cpu::new(1)
        .load("roms/snake.ch8")
        .context("Could not load ROM!")?;
    let chip8 = vm::Chip8VM::new(cpu);
    chip8.run();
}
