use super::{eightbit, nibbles, twelvebit};

pub enum Instruction {
    Nop,              // 0nnn, sys instruction on original machines but not used anymore
    Cls,              //00E0, clear display
    Ret,              //00EE, return from a subroutine
    Jmp(u16),         //1nnn, jump to address nnn
    Call(u16),        //2nnn, call subroutine at nnn
    Ldr(u8, u8),      //6xkk, put value kk in register x
    Add(u8, u8),      //7xkk, Adds the value kk to the value of register Vx
    Ldi(u16),         //Annn, load value nnn into index register
    Draw(u8, u8, u8), //Dxyn, display n byte-sprite starting at memory location I at coordinate (vx,vy), set VF = collision?
}

use Instruction::*;
pub fn decode(opcode: u16) -> Instruction {
    //the variable length operands
    //see http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
    let nnn = twelvebit(opcode);
    let kk = eightbit(opcode);
    match nibbles(opcode) {
        (0, 0, 0xE, 0xE) => Ret,
        (0, 0, 0xE, 0) => Cls,
        (0, _, _, _) => Nop,
        (1, _, _, _) => Jmp(nnn),
        (2, _, _, _) => Call(nnn),
        (6, x, _, _) => Ldr(x, kk),
        (7, x, _, _) => Add(x, kk),
        (0xA, _, _, _) => Ldi(nnn),
        (0xD, x, y, n) => Draw(x, y, n),
        _ => unimplemented!(),
    }
}
