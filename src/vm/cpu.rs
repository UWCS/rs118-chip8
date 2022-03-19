pub trait Cpu: Sized {
    fn step(&mut self);
    fn init(filename: &str) -> anyhow::Result<Self>;
}
