use anyhow::Context;
mod cpu;
mod vm;

fn main() -> anyhow::Result<()> {
    let cpu = cpu::Cpu::new(700)
        .load("roms/IBM Logo.ch8")
        .context("Could not load ROM!")?;
    let chip8 = vm::Chip8VM::new(cpu);
    chip8.run();
}
