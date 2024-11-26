use pest::Parser;
use pest_derive::Parser;

use vm::instruction::Instruction;
use vm::opcode::Opcode;

mod error;
use crate::error::ParseError;

#[derive(Parser)]
#[grammar = "assembly.pest"]
pub struct AssemblyParser;

macro_rules! parse_instruction {
    ($parser:expr, $opcode:ident, $pairs:expr) => {
        {
        let mut operands = Vec::new();
        for pair in $pairs.into_inner() {
            match pair.as_rule() {
                Rule::register => {
                    let register_str = pair.as_str();
                    let value: u8 = register_str[1..].parse().unwrap();
                    operands.push(value);
                }
                Rule::number => {
                    let number_str = pair.as_str();
                    let value: i32 = number_str[1..].parse().unwrap();
                    let bytes = value.to_le_bytes();
                    operands.push(bytes[1]);
                    operands.push(bytes[0]);
                }
                _ => {}
            }
        }
        Instruction::new(Opcode::$opcode,operands) 
    }
    };
}

impl AssemblyParser {
    pub fn parse_instruction(instruction: &str) -> Result<Instruction, ParseError> {
        let pairs = AssemblyParser::parse(Rule::instruction, instruction)
            .map_err(|e| ParseError::from(e))?
            .next()
            .ok_or_else(|| ParseError::custom("Failed to parse: No instruction found"))?
            .into_inner()
            .next()
            .ok_or_else(|| ParseError::custom("Failed to parse: No inner instructions"))?;

        let instruction = match pairs.as_rule() {
            Rule::load => parse_instruction!(AssemblerParser, LOAD, pairs),
            Rule::add => parse_instruction!(AssemblerParser, ADD, pairs),
            _ => {
                return Err(ParseError::UnknownInstruction);
            }
        };

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use vm::instruction::Instruction;

    use crate::AssemblyParser;

    #[test]
    fn test_load() {
        assert_eq!(
            Ok(Instruction::new(vm::opcode::Opcode::LOAD, vec![0, 1, 244])),
            AssemblyParser::parse_instruction("LOAD $0 #500")
        );
    }
}
