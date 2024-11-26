use crate::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    operands: Vec<u8>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<u8>) -> Self {
        Self { opcode, operands }
    }
}

impl Instruction {
    pub fn to_bytes(&self) -> [u8; 4] {
        let mut bytes = [0u8; 4];
        bytes[0] = self.opcode as u8;
        bytes[1..4].copy_from_slice(&self.operands[0..3]);
        bytes
    }
}
