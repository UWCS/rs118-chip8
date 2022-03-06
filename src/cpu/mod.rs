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
        match instruction {
            Instruction::Nop => None,
            Instruction::Cls => Some(clear_screen()),
            Instruction::Ret => {
                self.pc = self.stack.pop().unwrap_or(0);
                None
            }
            Instruction::Jmp(nnn) => {
                self.pc = nnn;
                None
            }
            Instruction::Call(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
                None
            }
            Instruction::Ldr(x, kk) => {
                self.registers[x as usize] = kk;
                None
            }
            Instruction::Add(x, kk) => {
                self.registers[x as usize] += kk;
                None
            }
            Instruction::Ldi(nnn) => {
                self.index = nnn;
                None
            }
            Instruction::Draw(x, y, n) => {
                let range = (self.index as usize)..((self.index + n as u16) as usize);
                let sprite = &self.memory[range];
                let x = self.registers[x as usize] & 63;
                let y = self.registers[y as usize] & 31;
                self.registers[0xf] = 0;
                Some(draw(sprite, (x, y)))
            }
        }
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
fn draw(sprite: &[u8], coords: (u8, u8)) -> Interrupt {
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
