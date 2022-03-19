mod cpu;
use cpu::Cpu;
pub struct Chip8<C: Cpu> {
    cpu: C,
    program_name: String,
    display: [[u8; 64]; 32],
}

impl<C: Cpu> Chip8<C> {
    pub fn new(filename: &str) -> Self {
        Chip8 {
            cpu: Cpu::init(filename).unwrap(),
            program_name: filename.to_owned(),
            display: [[0; 64]; 32],
        }
    }

    pub fn run(speed: usize) -> ! {
        loop {}
    }
}
