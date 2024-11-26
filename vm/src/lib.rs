#![allow(dead_code)]

mod opcode;
mod instruction;

use opcode::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: usize,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        Self { registers: [0; 32], pc: 0, program: vec![], remainder: 0, equal_flag: false }
    }
}

impl VM {
    pub fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    pub fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
        self.pc += 2;
        result
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.next_8_bits());
        opcode
    }
}

impl VM {
    pub fn run_loop(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }
}

impl VM {
    pub fn execute_instruction(&mut self) -> bool {
        let opcode = self.decode_opcode();
        match opcode {
            Opcode::LOAD => {
                let register_index = self.next_8_bits() as usize;
                let number = self.next_16_bits();
                self.registers[register_index] = number as i32;
                true
            }
            Opcode::ADD => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = value1 + value2;
                true
            }
            _ => { false }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::VM;

    #[test]
    fn create_vm() {
        let vm = VM::new();
        assert_eq!(0, vm.pc);
    }

    #[test]
    fn test_load() {
        let mut vm = VM::new();
        vm.program = vec![0, 0, 1, 244];
        vm.run_once();
        assert_eq!(vm.registers[0], 500);
    }

    #[test]
    fn test_add() {
        let mut vm = VM::new();
        vm.registers[0] = 1;
        vm.registers[1] = 2;
        vm.program = vec![1, 0, 1, 2];
        vm.run_once();
        assert_eq!(vm.registers[2], 3);
    }
}
