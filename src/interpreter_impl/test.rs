#![cfg(test)]
use super::*;
use instruction::Instruction::*;

// test that nop does nothing
#[test]
fn test_nop() {
    let mut vm = VM::new(100);
    vm.execute(Nop, &[false; 16]);

    let vm_2 = VM::new(100);
    assert_eq!(vm, vm_2);
}

//TODO: add more tests
