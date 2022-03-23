use super::{eightbit, nibbles, twelvebit};

type Reg = u8;
type Addr = u16;
#[derive(Debug)]
pub enum Instruction {
    Nop,                //0nnn, sys instruction on original machines but not used anymore
    Cls,                //00E0, clear display
    Rts,                //00EE, return from a subroutine
    Jmp(Addr),          //1nnn, jump to address nnn
    Call(Addr),         //2nnn, call subroutine at nnn
    Ske(Reg, u8),       //3xkk, skip the next instruction if Vx == kk
    Skne(Reg, u8),      //4xkk, skip the next instruction if Vx != kk
    Skre(Reg, Reg),     //5xy0, skip the next instruction if Vx == Vy
    Loadr(Reg, u8),     //6xkk, put value kk in register x
    Add(Reg, u8),       //7xkk, Adds the value kk to the value of register Vx
    Move(Reg, Reg),     //8xy0, Vx = Vy,
    Or(Reg, Reg),       //8xy1, Vx = Vx OR Vy
    And(Reg, Reg),      //8xy2, Vx = Vx AND Vy
    Xor(Reg, Reg),      //8xy3, Vx = Vx XOR Vy
    Addr(Reg, Reg),     //8xy4, Vx = Vx + Vy
    Sub(Reg, Reg),      //8xy5, Vx = Vx - Vy
    Shr(Reg, Reg),      //8xy6, Vx = Vy >> 1
    Ssub(Reg, Reg),     //8xy7 Vx = Vy - Vx
    Shl(Reg, Reg),      //8xyE, Vx = Vy << 1
    Skrne(Reg, Reg),    //9xy0, skip the next instruction if Vx != Vy
    Loadi(u16),         //Annn, load value nnn into index register
    Jumpi(u16),         //Bnnn, jump to the instruction in index register, + offset nnn
    Rand(Reg, u8),      //Cxkk, Vx = rand() & kk
    Draw(Reg, Reg, u8), //Dxyn, display n byte-sprite starting at memory location I at coordinate (vx,vy), set VF = collision?
    Skp(Reg), //Ex9E, skip the next instruction if the key with the value in Vx is currently pressed down
    Sknp(Reg), //ExA1, skip the next instruction if the key with the value in Vx is NOT currently pressed down
    Moved(Reg), //Fx07, Vx = display timer
    Key(Reg),  //Fx0A, block while waiting for keypress, then store the key pressed in Vx
    Loadd(Reg), //Fx15, display timer = Vx
    Loads(Reg), //Fx18, sound timer = Vy
    Addi(Reg), //Fx1E, index = Vx + index
    Ldfnt(Reg), //Fx29, load index register with address of the font charachter in Vx
    Bcd(Reg), //Store three bytes represnting the binary-coded decimal value of Vx to the address at index
    Store(Reg), //Store all the registers at the address in the index register
    Load(Reg), //Load all the registers with values from the address at the index register
}

use Instruction::*;
pub fn decode(opcode: u16) -> Instruction {
    //the variable length operands
    //see http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
    let nnn = twelvebit(opcode);
    let kk = eightbit(opcode);

    //parse the instruction into a structured representation
    //big match
    match nibbles(opcode) {
        (0, 0, 0xE, 0xE) => Rts,
        (0, 0, 0xE, 0) => Cls,
        (0, _, _, _) => Nop,
        (1, _, _, _) => Jmp(nnn),
        (2, _, _, _) => Call(nnn),
        (6, x, _, _) => Loadr(x, kk),
        (7, x, _, _) => Add(x, kk),
        (0xA, _, _, _) => Loadi(nnn),
        (0xD, x, y, n) => Draw(x, y, n),
        _ => unimplemented!(),
    }
}
