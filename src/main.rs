mod cpu;
mod interpreter;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let cpu = cpu::Cpu::new(500)
        .load("roms/br8kout.ch8")
        .context("Could not load ROM!")?;
    interpreter::Cpu::run(cpu);
}
