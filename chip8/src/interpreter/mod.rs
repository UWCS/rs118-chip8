mod font;
mod instruction;
mod test;

use chip8_base::{
    Display, Keys,
    Pixel::{self, *},
};
use instruction::{decode, Instruction};
use rand::random;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub struct VM {
    memory: [u8; 4096],
    pc: u16,
    index: u16,
    stack: Vec<u16>,
    registers: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    speed: Duration,
    ticker: u32,
    max_ticks: u32,
    display: [[Pixel; 64]; 32],
}

impl chip8_base::Interpreter for VM {
    //this should execute in the time 1/speed
    fn step(&mut self, keys: &Keys) -> Option<Display> {
        let opcode = self.fetch();
        let instruction = decode(opcode);
        let update = self.execute(instruction, keys);

        //ticker counts up to max_ticks, and at max_ticks the timers are decremented
        self.ticker += 1;
        if self.ticker == self.max_ticks {
            self.ticker = 0;
            self.delay_timer = self.delay_timer.saturating_sub(1);
            self.sound_timer = self.sound_timer.saturating_sub(1);
        }

        update
    }

    fn speed(&self) -> Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        self.sound_timer != 0
    }
}

impl VM {
    pub fn new(speed: u32) -> Self {
        let mut memory = [0_u8; 4096];

        //font is 80 bytes, should lie at 0x50
        memory[0x50..(0x50 + 80)].copy_from_slice(&font::FONT);

        VM {
            memory: [0; 4096],
            pc: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::new(),
            registers: [0; 16],
            speed: Duration::from_secs_f64(1_f64 / speed as f64),
            ticker: 0,
            max_ticks: (speed as f64 / 60_f64).round() as u32,
            display: [[Pixel::default(); 64]; 32],
        }
    }

    pub fn load(mut self, filename: &str) -> std::io::Result<Self> {
        let program = std::fs::read(filename)?;
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
        self.pc = 0x200;
        Ok(self)
    }

    fn fetch(&mut self) -> u16 {
        let instruction = u16::from_be_bytes([
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        ]);
        self.inc_pc();
        instruction
    }

    fn execute(&mut self, instruction: Instruction, keys: &Keys) -> Option<Display> {
        match instruction {
            Instruction::Nop => (),
            Instruction::Cls => {
                self.display = [[Black; 64]; 32];
                return Some(self.display);
            }
            Instruction::Ret => {
                self.pc = self.stack.pop().unwrap_or(0);
            }
            Instruction::Jmp(addr) => {
                self.pc = addr;
            }
            Instruction::Call(addr) => {
                self.stack.push(self.pc);
                self.pc = addr;
            }
            Instruction::Setr(r, byte) => {
                self.registers[r as usize] = byte;
            }
            Instruction::Addr(r, byte) => {
                self.registers[r as usize] = self.registers[r as usize].wrapping_add(byte)
            }
            Instruction::Seti(nnn) => {
                self.index = nnn;
            }
            Instruction::Draw(rx, ry, n) => {
                let range = (self.index as usize)..((self.index + n as u16) as usize);
                let sprite = &self.memory[range];
                let x = self.registers[rx as usize] % 64;
                let y = self.registers[ry as usize] % 32;
                self.registers[0xf] = 0;
                for (i, row) in sprite.iter().enumerate() {
                    if y + i as u8 > 31 {
                        break;
                    }
                    for (j, sprite_px) in (0..8).zip(PixIterator::new(row)) {
                        if x + j as u8 > 63 {
                            break;
                        }
                        let display_px = &mut self.display[(y as usize + i)][(x as usize + j)];

                        //set vf high on collide
                        if (*display_px & sprite_px).into() {
                            self.registers[0xf] = 1;
                        }

                        //xor onto display
                        *display_px ^= sprite_px;
                    }
                }
                return Some(self.display);
            }
            Instruction::Ske(x, byte) => {
                if self.registers[x as usize] == byte {
                    self.inc_pc();
                }
            }
            Instruction::Skne(x, byte) => {
                if self.registers[x as usize] != byte {
                    self.inc_pc();
                }
            }
            Instruction::Skre(x, y) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.inc_pc();
                }
            }
            Instruction::Move(x, y) => self.registers[x as usize] = self.registers[y as usize],
            Instruction::Or(x, y) => self.registers[x as usize] |= self.registers[y as usize],
            Instruction::And(x, y) => self.registers[x as usize] &= self.registers[y as usize],
            Instruction::Xor(x, y) => self.registers[x as usize] ^= self.registers[y as usize],
            Instruction::Add(x, y) => {
                let (result, overflow) =
                    self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                self.registers[x as usize] = result;
                self.registers[0xf] = overflow.into();
            }
            Instruction::Sub(x, y) => {
                let (result, overflow) =
                    self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                self.registers[x as usize] = result;
                self.registers[0xf] = overflow.into();
            }
            Instruction::Shr(x, _) => {
                //y is ignored
                self.registers[0xf] = 1 & self.registers[x as usize];
                self.registers[x as usize] >>= 1;
            }
            Instruction::Ssub(x, y) => {
                let (result, overflow) =
                    self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
                self.registers[x as usize] = result;
                self.registers[0xf] = overflow.into();
            }
            Instruction::Shl(x, _) => {
                //y is ignored
                self.registers[0xf] = 0x80 & &self.registers[x as usize];
                self.registers[x as usize] <<= 1;
            }
            Instruction::Skrne(x, r2) => {
                if self.registers[x as usize] != self.registers[r2 as usize] {
                    self.inc_pc();
                }
            }
            Instruction::Jumpi(nnn) => self.pc = (nnn + self.registers[0] as u16) & 0xfff, //u12 wrap
            Instruction::Rand(x, byte) => self.registers[x as usize] = random::<u8>() & byte,
            Instruction::Skp(x) => {
                if keys[self.registers[x as usize] as usize] {
                    self.inc_pc()
                }
            }
            Instruction::Sknp(x) => {
                if !keys[self.registers[x as usize] as usize] {
                    self.inc_pc()
                }
            }
            Instruction::Moved(x) => self.registers[x as usize] = self.delay_timer,
            Instruction::Key(x) => {
                //waiting is implemented by just re-running the instruction until a keypress is detected
                //might be bad if run at slower speeds

                //if no keys held
                if keys.iter().all(|k| !k) {
                    self.pc -= 2
                } else {
                    //at least one key is pressed, so get the index of the first one from the array thats held down
                    self.registers[x as usize] = keys
                        .iter()
                        .enumerate()
                        .filter_map(|(k, b)| if *b { Some(k) } else { None })
                        .next()
                        .unwrap() as u8;
                }
                dbg!(&keys);
            }
            Instruction::Setrd(x) => self.delay_timer = self.registers[x as usize],
            Instruction::Setrs(x) => self.sound_timer = self.registers[x as usize],
            Instruction::Addi(x) => {
                //weird wrapping arithmetic, u16+u8 but has to wrap to a u12
                self.index = (self.index + (self.registers[x as usize] as u16)) & 0xfff;
            }
            Instruction::Ldfnt(x) => {
                //font starts at 0x50 in memory
                let char_offset = self.registers[x as usize] as u16 * 5;
                self.index = 0x50 + char_offset;
            }
            Instruction::Bcd(x) => {
                let slice = &mut self.memory[(self.index as usize)..(self.index as usize + 3)];
                //binary encoded decimal conversion
                let val = self.registers[x as usize];
                slice[0] = val / 100;
                slice[1] = val % 100 / 10;
                slice[2] = val % 10;
            }
            Instruction::Store(x) => {
                for reg in 0..=x as usize {
                    self.memory[self.index as usize + reg] = self.registers[reg];
                }
            }
            Instruction::Load(y) => {
                for reg in 0..=y as usize {
                    self.registers[reg] = self.memory[self.index as usize + reg];
                }
            }
        };
        None
    }

    //helpers
    //wrapping pc incremement so we dont forget to do it anywhere
    fn inc_pc(&mut self) {
        self.pc += 2;
        self.pc &= 0xfff;
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
//an iterator over the bits of a byte, as pixels
//this is totally unnecessary but I thought it was neat
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
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < 8 {
            let bit = self.byte >> (7 - self.idx) & 1;
            self.idx += 1;
            assert!(bit == 1 || bit == 0);
            Some(bit.try_into().unwrap()) //safe to unwrap because we assert
        } else {
            None
        }
    }
}
