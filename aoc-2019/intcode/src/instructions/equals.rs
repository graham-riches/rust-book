use crate::instructions::op_code::OpCode;
use crate::lexer;

const OP_CODE_ID: i64 = 8;
const INSTRUCTION_POINTER_OFFSET: i64 = 4;

#[derive(PartialEq, Debug, Default)]
pub struct Equals { 
    arg1: lexer::Parameter,
    arg2: lexer::Parameter,
    output: lexer::Parameter
}

impl OpCode for Equals {
    /// Parses an output instruction from a slice of a program
    fn parse_from_slice(program: &[i64]) -> Option<Equals> {        
        let mut operation = match lexer::parse_instruction_type(program[0]) {
            Some(lexer::InstructionType{ op_code: OP_CODE_ID, a, b, c}) => Equals{ arg1: c, arg2: b, output: a},
            Some(_i) => return None,
            None => return None
        };
        operation.arg1.value = program[1];
        operation.arg2.value = program[2];
        operation.output.value = program[3];                
        Some(operation)
    }

    /// Applies a less than compare store operation on a program
    fn apply(&self, program: &mut [i64], instruction_pointer: i64, _input: fn() -> i64, _output: fn(i64) -> ()) -> i64 {
        let value_1 = lexer::get_parameter_value(&self.arg1, &program);
        let value_2 = lexer::get_parameter_value(&self.arg2, &program);        
        if value_1 == value_2 {
            program[self.output.value as usize] = 1;
        } else {
            program[self.output.value as usize] = 0;
        }
        instruction_pointer + INSTRUCTION_POINTER_OFFSET
    }

    /// Gets the instruction pointer offset for a complete
    fn get_instruction_pointer_offset(&self) -> i64 {
        INSTRUCTION_POINTER_OFFSET
    }

    /// Gets the operation code
    fn get_op_code(&self) -> i64 {
        OP_CODE_ID
    }
}