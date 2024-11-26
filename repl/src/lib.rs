use assembler::AssemblyParser;
use std::{io, io::Write};
use vm::VM;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }
}

impl REPL {
    pub fn run_loop(&mut self) {
        println!("Welcome to Susuro!");
        loop {
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");
            let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("Unable to read line from user");
            let command = command.trim();
            self.command_buffer.push(command.to_string());

            match command {
                "quit" => {
                    println!("Bye");
                    std::process::exit(0);
                }
                "register" => {
                    println!("{:?}", self.vm.registers);
                }
                "program" => {
                    println!("{:?}", self.vm.program);
                }
                "history" => {
                    println!("{:#?}", self.command_buffer);
                }
                "clear" => {
                    self.command_buffer.clear();
                }
                _ => {
                    if let Ok(instruction) = AssemblyParser::parse_instruction(command) {
                        self.vm.add_bytes(&instruction.to_bytes());
                        self.vm.run_once();
                    } else {
                        println!("Invalid input");
                    }
                }
            }
        }
    }
}
