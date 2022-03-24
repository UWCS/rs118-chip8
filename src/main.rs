mod cpu;
mod cpu_impl;

fn main() {
    let cpu = cpu_impl::Cpu::new(500)
        .load("roms/chip8-test-rom-with-audio.ch8")
        .expect("Could not load ROM");
    cpu::Cpu::run(cpu);
}
