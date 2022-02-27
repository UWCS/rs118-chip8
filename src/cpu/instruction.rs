pub enum Instruction {
    Nop,        // 0nnn, sys instruction on original machines but not used anymore
    Cls,        //00E0, clear display
    Ret,        //00EE, return from a subroutine
    Jmp(u16),   //1nnn, jump to address nnn
    Ld(u8, u8), //6xkk, put value kk in register x
}

use Instruction::*;
pub fn decode(opcode: u16) -> Instruction {
    Nop
}
