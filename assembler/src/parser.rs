use pest::Parser;
use pest_derive::Parser;

use vm::instruction::Instruction;
use vm::opcode::Opcode;

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
        Instruction::new(Opcode::$opcode, operands)
        }
    };
}

impl AssemblyParser {
    pub fn parse_instruction(instruction: &str) -> Result<Instruction, ParseError> {
        let mut inner = AssemblyParser::parse(Rule::instruction, instruction)
            .map_err(|e| ParseError::from(e))?
            .next()
            .ok_or_else(|| ParseError::custom("Failed to parse: No instruction found"))?
            .into_inner();

        let mut pairs = inner
            .next()
            .ok_or_else(|| ParseError::custom("Failed to parse: No inner instructions"))?;

        if pairs.as_rule() == Rule::label {
            // println!(
            //     "{}",
            //     pairs
            //         .into_inner()
            //         .next()
            //         .ok_or_else(|| ParseError::custom("Label"))?
            //         .as_str()
            // );
            pairs = inner
                .next()
                .ok_or_else(|| ParseError::custom("Failed to parse: No inner instructions"))?;
        }

        let instruction = match pairs.as_rule() {
            Rule::load => parse_instruction!(AssemblerParser, LOAD, pairs),
            Rule::add => parse_instruction!(AssemblerParser, ADD, pairs),
            Rule::sub => parse_instruction!(AssemblerParser, SUB, pairs),
            Rule::mul => parse_instruction!(AssemblerParser, MUL, pairs),
            Rule::div => parse_instruction!(AssemblerParser, DIV, pairs),
            Rule::halt => parse_instruction!(AssemblerParser, HLT, pairs),
            Rule::jmp => parse_instruction!(AssemblerParser, JMP, pairs),
            Rule::jmpf => parse_instruction!(AssemblerParser, JMPF, pairs),
            Rule::jmpb => parse_instruction!(AssemblerParser, JMPB, pairs),
            Rule::eq => parse_instruction!(AssemblerParser, EQ, pairs),
            Rule::neq => parse_instruction!(AssemblerParser, NEQ, pairs),
            Rule::gte => parse_instruction!(AssemblerParser, GTE, pairs),
            Rule::lte => parse_instruction!(AssemblerParser, LTE, pairs),
            Rule::gt => parse_instruction!(AssemblerParser, GT, pairs),
            Rule::lt => parse_instruction!(AssemblerParser, LT, pairs),
            Rule::jmpe => parse_instruction!(AssemblerParser, JMPE, pairs),
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
    use vm::opcode::Opcode;

    use super::AssemblyParser;

    #[test]
    fn test_load() {
        assert_eq!(
            Ok(Instruction::new(Opcode::LOAD, vec![0, 1, 244])),
            AssemblyParser::parse_instruction("LOAD $0 #500")
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Ok(Instruction::new(Opcode::ADD, vec![0, 1, 2])),
            AssemblyParser::parse_instruction("ADD $0 $1 $2")
        );
    }

    #[test]
    fn test_label() {
        assert_eq!(
            AssemblyParser::parse_instruction("a: LOAD $0 #1"),
            Ok(Instruction::new(Opcode::LOAD, vec![0, 0, 1]))
        );
    }
}
