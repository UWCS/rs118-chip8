pub mod cpu;
mod vm;

fn main() -> anyhow::Result<()> {
    let cpu = cpu::CPU::new(700).load("IBM Logo.ch8")?;
    let chip8 = vm::Chip8VM::new(cpu);
    chip8.run();
}

// pub fn update(pixels: &mut Pixels, buffer: [[u8; 64]; 32]) {
//     pixels
//         .get_frame()
//         .write_all(&buffer.concat())
//         .expect("Could not update Pixels buffer");
// }
