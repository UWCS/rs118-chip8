pub mod cpu;
mod vm;

fn main() -> anyhow::Result<()> {
    let cpu = cpu::CPU::new(700).load("IBM Logo.ch8")?;
    let chip8 = vm::Chip8VM::new(cpu);
    chip8.run();
}


