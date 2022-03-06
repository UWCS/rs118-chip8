mod font;
mod instruction;
mod test;

use instruction::{decode, Instruction};

pub struct CPU {
    memory: [u8; 4096],
    pc: u16,
    index: u16,
    stack: Vec<u16>,
    delay: u8,
    sound: u8,
    registers: [u8; 16],
}

pub enum Interrupt {
    DisplayUpdate(Vec<(u8, u8)>),
}

impl CPU {
    pub fn init() -> Self {
        let mut memory = [0_u8; 4096];
        //font is 80 bytes, should lie at 0x50
        memory[0x50..0xA0].copy_from_slice(&font::FONT);

        CPU {
            memory: [0; 4096],
            pc: 0,
            index: 0,
            delay: 0,
            sound: 0,
            stack: Vec::new(),
            registers: [0; 16],
        }
    }

    fn fetch(&mut self) -> u16 {
        let mut instruction: u16 = 0;
        instruction &= (self.memory[self.pc as usize] as u16) << 8;
        instruction &= self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        instruction
    }

    fn exectute(&mut self, instruction: Instruction) -> Option<Interrupt> {
        None
    }

    pub fn step(&mut self) -> Option<Interrupt> {
        let opcode = self.fetch();
        let instruction = decode(opcode);
        self.exectute(instruction)
    }
}
//helpers here

//generate an interrupt to clear screen
fn clear_screen() -> Interrupt {
    Interrupt::DisplayUpdate(Vec::new())
}

//break a u16 into its nibbles
fn nibbles(n: u16) -> (u8, u8, u8, u8) {
    let n3 = (n >> 12) as u8;
    let n2 = ((n >> 8) & 0b1111) as u8;
    let n1 = ((n >> 4) & 0b1111) as u8;
    let n0 = (n & 0b1111) as u8;
    (n3, n2, n1, n0)
}

//get the lower 12 bytes of a u16
fn twelvebit(n: u16) -> u16 {
    n & 0xfff
}

//get the lower 8 bytes of a u16
fn eightbit(n: u16) -> u8 {
    (n & 0xff) as u8
}

//helpers
