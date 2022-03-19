mod font;
mod instruction;
mod test;

use anyhow::Result;

use instruction::{decode, Instruction};

pub struct CPU {
    memory: [u8; 4096],
    pc: u16,
    index: u16,
    stack: Vec<u16>,
    delay: u8,
    sound: u8,
    registers: [u8; 16],
    speed: u32,
}

impl crate::vm::Chip8Cpu for CPU {
    //this should execute in the time 1/speed
    fn step(&mut self, display: &mut [[u8; 64]; 32], keys: &[bool; 16]) {
        let opcode = self.fetch();
        let instruction = decode(opcode);

        dbg!(&instruction);
        self.exectute(instruction, display, keys);
    }
}

impl CPU {
    pub fn new(speed: u32) -> Self {
        let mut memory = [0_u8; 4096];

        //font is 80 bytes, should lie at 0x50
        memory[0x50..0xA0].copy_from_slice(&font::FONT);

        CPU {
            memory: [0; 4096],
            pc: 0x200,
            index: 0,
            delay: 0,
            sound: 0,
            stack: Vec::new(),
            registers: [0; 16],
            speed,
        }
    }

    pub fn load(mut self, filename: &str) -> Result<Self> {
        let program = std::fs::read(filename)?;
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
        Ok(self)
    }

    fn fetch(&mut self) -> u16 {
        let mut instruction: u16 = 0;
        instruction &= (self.memory[self.pc as usize] as u16) << 8;
        instruction &= self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        //wrapping
        if self.pc >= 4096 {
            self.pc = 0;
        }
        instruction
    }

    fn exectute(
        &mut self,
        instruction: Instruction,
        display: &mut [[u8; 64]; 32],
        keys: &[bool; 16],
    ) {
        match instruction {
            Instruction::Nop => (),
            Instruction::Cls => (),
            Instruction::Rts => {
                self.pc = self.stack.pop().unwrap_or(0);
            }
            Instruction::Jmp(nnn) => {
                self.pc = nnn;
            }
            Instruction::Call(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            Instruction::Loadr(x, kk) => {
                self.registers[x as usize] = kk;
            }
            Instruction::Add(x, kk) => {
                self.registers[x as usize] += kk;
            }
            Instruction::Loadi(nnn) => {
                self.index = nnn;
            }
            Instruction::Draw(x, y, n) => {
                let range = (self.index as usize)..((self.index + n as u16) as usize);
                let sprite = &self.memory[range];
                let mut x = self.registers[x as usize] & 63;
                let mut y = self.registers[y as usize] & 31;
                self.registers[0xf] = 0;

                for row in sprite {
                    for sprite_px in PixIterator::new(row) {
                        let display_px = display[x as usize][y as usize];

                        //set vf on collide
                        if display_px == 1 && sprite_px == 1 {
                            self.registers[0xf] = 1;
                        }
                        //xor onto display
                        display[x as usize][y as usize] ^= sprite_px;

                        //are we at the right edge of the screen?
                        if x == 63 {
                            break;
                        } else {
                            x += 1;
                        }
                    }
                    // are we at the bottom of the screen?
                    if y == 31 {
                        break;
                    } else {
                        y += 1;
                    }
                }
            }
            Instruction::Ske(_, _) => todo!(),
            Instruction::Skne(_, _) => todo!(),
            Instruction::Skre(_, _) => todo!(),
            Instruction::Move(_, _) => todo!(),
            Instruction::Or(_, _) => todo!(),
            Instruction::And(_, _) => todo!(),
            Instruction::Xor(_, _) => todo!(),
            Instruction::Addr(_, _) => todo!(),
            Instruction::Sub(_, _) => todo!(),
            Instruction::Shr(_, _) => todo!(),
            Instruction::Ssub(_, _) => todo!(),
            Instruction::Shl(_, _) => todo!(),
            Instruction::Skrne(_, _) => todo!(),
            Instruction::Jumpi(_) => todo!(),
            Instruction::Rand(_, _) => todo!(),
            Instruction::Skp(_) => todo!(),
            Instruction::Sknp(_) => todo!(),
            Instruction::Moved(_) => todo!(),
            Instruction::Key(_) => todo!(),
            Instruction::Loadd(_) => todo!(),
            Instruction::Loads(_) => todo!(),
            Instruction::Addi(_) => todo!(),
            Instruction::Ldfnt(_) => todo!(),
            Instruction::Bcd(_) => todo!(),
            Instruction::Store(_) => todo!(),
            Instruction::Load(_) => todo!(),
        }
    }
}
//helpers here

//break a u16 into its nibbles
fn nibbles(n: u16) -> (u8, u8, u8, u8) {
    let n3 = (n >> 12) as u8;
    let n2 = ((n >> 8) & 0b1111) as u8;
    let n1 = ((n >> 4) & 0b1111) as u8;
    let n0 = (n & 0b1111) as u8;
    (n3, n2, n1, n0)
}

//get the lower 12 bits of a u16
fn twelvebit(n: u16) -> u16 {
    n & 0xfff
}

//get the lower 8 bits of a u16
fn eightbit(n: u16) -> u8 {
    (n & 0xff) as u8
}

//helpers
//an iterator over the bits of a byte
struct PixIterator {
    byte: u8,
    idx: u8,
}

impl PixIterator {
    pub fn new(byte: &u8) -> Self {
        Self {
            byte: *byte,
            idx: 0,
        }
    }
}

impl Iterator for PixIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < 8 {
            let bit = self.byte >> (7 - self.idx) & 1;
            self.idx += 1;
            assert!(bit == 1 || bit == 0);
            Some(bit)
        } else {
            None
        }
    }
}
