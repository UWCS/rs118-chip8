mod cpu;
pub use cpu::Chip8Cpu;
pub struct Chip8VM<C: Chip8Cpu> {
    cpu: C,
    program_name: String,
    display: [[u8; 64]; 32],
    keys: [bool; 16],
}

impl<C: Chip8Cpu> Chip8VM<C> {
    pub fn new(filename: &str, speed: u32) -> Self {
        Chip8VM {
            cpu: Chip8Cpu::init(filename, speed).unwrap(),
            program_name: filename.to_owned(),
            display: [[0; 64]; 32],
            keys: [false; 16],
        }
    }

    pub fn run() -> ! {
        loop {}
    }
}
