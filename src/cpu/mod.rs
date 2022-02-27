mod font;
mod instruction;
mod test;
struct CPU {
    memory: [u8; 4096],
    pc: u16,
    stack: Vec<u16>,
    delay: u8,
    sound: u8,
    registers: [u8; 16],
    clock_speed: u32,
}

pub enum Interrupt {
    DisplayUpdate(Vec<(u8, u8)>),
}

impl CPU {
    fn init(&mut self, speed: u32) -> Self {
        let mut memory = [0_u8; 4096];
        //font is 80 bytes, should lie at 0x50
        memory[0x50..0xA0].copy_from_slice(&font::FONT);

        CPU {
            memory: [0; 4096],
            pc: 0,
            delay: 0,
            sound: 0,
            stack: Vec::new(),
            registers: [0; 16],
            clock_speed: speed,
        }
        //load font and other defaults into memory
    }

    pub fn fetch(&mut self) -> u16 {
        let mut instruction: u16 = 0;
        instruction &= (self.memory[self.pc as usize] as u16) << 8;
        instruction &= self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        instruction
    }

    pub fn exectute(&mut self, instruction: u16) -> Option<Interrupt> {
        None
    }

    pub fn step(&mut self) {}
}
//helpers here

//generate an interrupt to clear screen
fn clear_screen() -> Interrupt {
    Interrupt::DisplayUpdate(Vec::new())
}
