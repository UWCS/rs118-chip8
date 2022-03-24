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
    Bcd(Reg), //Fx33 Store three bytes represnting the binary-coded decimal value of Vx to the address at index
    Store(Reg), //Fx55 Store all the registers at the address in the index register
    Load(Reg), //Fx65 Load all the registers with values from the address at the index register
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
        (3, x, _, _) => Ske(x, kk),
        (4, x, _, _) => Skne(x, kk),
        (5, x, y, 0) => Skre(x, y),
        (6, x, _, _) => Loadr(x, kk),
        (7, x, _, _) => Add(x, kk),
        (8, x, y, 0) => Move(x, y),
        (8, x, y, 1) => Or(x, y),
        (8, x, y, 2) => And(x, y),
        (8, x, y, 3) => Xor(x, y),
        (8, x, y, 4) => Addr(x, y),
        (8, x, y, 5) => Sub(x, y),
        (8, x, y, 6) => Shr(x, y),
        (8, x, y, 7) => Ssub(x, y),
        (8, x, y, 0xE) => Shl(x, y),
        (9, x, y, 0) => Skrne(x, y),
        (0xA, _, _, _) => Loadi(nnn),
        (0xB, _, _, _) => Jumpi(nnn),
        (0xC, x, _, _) => Rand(x, kk),
        (0xD, x, y, n) => Draw(x, y, n),
        (0xE, x, 9, 0xE) => Skp(x),
        (0xE, x, 0xA, 1) => Sknp(x),
        (0xF, x, 0, 7) => Moved(x),
        (0xF, x, 0, 0xA) => Key(x),
        (0xF, x, 1, 5) => Loadd(x),
        (0xF, x, 1, 8) => Loads(x),
        (0xF, x, 1, 0xE) => Addi(x),
        (0xF, x, 2, 9) => Ldfnt(x),
        (0xF, x, 3, 3) => Bcd(x),
        (0xF, x, 5, 5) => Store(x),
        (0xF, x, 6, 5) => Load(x),
        _ => panic!("Invalid instruction encountered!: {:#06X}", opcode),
    }
}
