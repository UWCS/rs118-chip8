mod interpreter;

use clap::Parser;

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let filename: &str = &cli.rom;
    let cpu = interpreter::VM::new(700)
        .load(filename)
        .unwrap_or_else(|_| panic!("Could not load ROM: {}", filename));
    chip8_base::run(cpu);
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// A CHIP-8 ROM to load into the interpreter
    #[clap(validator = rom_exists)]
    rom: String,
}

fn rom_exists(f: &str) -> Result<(), &'static str> {
    let p = std::path::Path::new(f);
    if !p.is_file() {
        Err("File does not exist.")
    } else {
        Ok(())
    }
}
