#![allow(dead_code)]

pub mod instruction;
pub mod opcode;

use opcode::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    heap: Vec<u8>,
    remainder: usize,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            heap: vec![],
            remainder: 0,
            equal_flag: false,
        }
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
    pub fn add_bytes(&mut self, bytes: &[u8; 4]) {
        self.program.extend_from_slice(bytes);
    }
}

impl VM {
    pub fn run_loop(&mut self) {
        let mut is_done = false;
        while !is_done && self.pc < self.program.len() {
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
        dbg!(&opcode);
        match opcode {
            Opcode::LOAD => {
                let register_index = self.next_8_bits() as usize;
                let number = self.next_16_bits();
                self.registers[register_index] = number as i32;
            }
            Opcode::ADD => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = value1 + value2;
            }
            Opcode::SUB => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = value1 - value2;
            }
            Opcode::MUL => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = value1 * value2;
            }
            Opcode::DIV => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = value1 / value2;
                self.remainder = (value1 % value2) as usize;
            }
            Opcode::HLT => {
                return true;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            }
            Opcode::EQ => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 == value2;

                self.next_8_bits();
            }
            Opcode::NEQ => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 != value2;

                self.next_8_bits();
            }
            Opcode::GTE => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 >= value2;

                self.next_8_bits();
            }
            Opcode::LTE => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 <= value2;

                self.next_8_bits();
            }
            Opcode::LT => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 < value2;

                self.next_8_bits();
            }
            Opcode::GT => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 > value2;

                self.next_8_bits();
            }
            Opcode::JMPE => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::ALOC => {
                let bytes = self.registers[self.next_8_bits() as usize];
                let new_end = (self.heap.len() as i32) + bytes;
                self.heap.resize(new_end as usize, 0);
                self.next_16_bits();
            }
            _ => {
                return true;
            }
        }
        false
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

    #[test]
    fn test_sub() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 1;
        vm.program = vec![2, 0, 1, 2];
        vm.run_once();
        assert_eq!(vm.registers[2], 1);
    }
    #[test]
    fn test_mul() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 3;
        vm.program = vec![3, 0, 1, 2];
        vm.run_once();
        assert_eq!(vm.registers[2], 6);
    }
    #[test]
    fn test_div() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 3;
        vm.program = vec![4, 0, 1, 2];
        vm.run_once();
        assert_eq!(vm.registers[2], 2);
        assert_eq!(vm.remainder, 0);
    }

    #[test]
    fn test_remiander() {
        let mut vm = VM::new();
        vm.registers[0] = 11;
        vm.registers[1] = 4;
        vm.program = vec![4, 0, 1, 2];
        vm.run_once();
        assert_eq!(vm.registers[2], 2);
        assert_eq!(vm.remainder, 3);
    }
    #[test]
    fn test_jmp() {
        let mut vm = VM::new();
        vm.registers[0] = 3;
        vm.program = vec![6, 0, 5, 0, 0, 0, 1];
        // jmp $0 hlt load $0 #1
        vm.run_loop();
        assert_eq!(vm.registers[0], 1);
    }
    #[test]
    fn test_jmpf() {
        let mut vm = VM::new();
        vm.registers[0] = 1;
        vm.program = vec![7, 0, 5, 0, 0, 0, 2];
        // jmpf $0 hlt load $0 #1
        vm.run_loop();
        assert_eq!(vm.registers[0], 2);
    }
    #[test]
    fn test_jmpb() {
        let mut vm = VM::new();
        vm.registers[0] = 1;
        vm.program = vec![8, 0];
        // jmpb $0
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_eq() {
        let mut vm = VM::new();
        vm.registers[0] = 1;
        vm.registers[1] = 1;
        vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        vm.registers[0] = 2;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
    }

    #[test]
    fn test_neq() {
        let mut vm = VM::new();
        vm.registers[1] = 2;
        vm.program = vec![10, 0, 1, 0];
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
    }

    #[test]
    fn test_aloc() {
        let mut vm = VM::new();
        vm.registers[0] = 1024;
        vm.program = vec![17, 0, 0, 0];
        vm.run_once();
        assert_eq!(vm.heap.len(), 1024);
    }
}
