pub trait Chip8Cpu: Sized {
    fn step(&mut self, display: &mut [[u8; 64]; 32], keys: &[bool; 16]);
    fn init(filename: &str, speed: u32) -> anyhow::Result<Self>;
}
