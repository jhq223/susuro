#![allow(dead_code)]

mod opcode;
mod instruction;
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: usize,
    equal_flag: bool,
}
