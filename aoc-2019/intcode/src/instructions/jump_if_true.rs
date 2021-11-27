use crate::instructions::op_code::OpCode;
use crate::lexer;

const OP_CODE_ID: i64 = 5;
const INSTRUCTION_POINTER_OFFSET: i64 = 3;

#[derive(PartialEq, Debug)]
pub struct JumpIfTrue { 
    arg1: lexer::Parameter,
    arg2: lexer::Parameter
}

impl OpCode for JumpIfTrue {
    /// Parses an output instruction from a slice of a program
    fn parse_from_slice(program: &[i64]) -> Option<JumpIfTrue> {        
        let mut operation = match lexer::parse_instruction_type(program[0]) {
            Some(lexer::InstructionType{ op_code: OP_CODE_ID, a: _, b, c}) => JumpIfTrue{ arg1: c, arg2: b},
            Some(_i) => return None,
            None => return None
        };
        operation.arg1.value = program[1];
        operation.arg2.value = program[2];
        Some(operation)
    }

    /// Applies a jump if true compare operation on a program
    fn apply(&self, program: &mut [i64], instruction_pointer: i64, _input: fn() -> i64, _output: fn(i64) -> ()) -> i64 {
        let cmp = lexer::get_parameter_value(&self.arg1, &program);
        let jump_to = lexer::get_parameter_value(&self.arg2, &program);
        if cmp > 0 {
            jump_to
        } else {
            instruction_pointer + INSTRUCTION_POINTER_OFFSET
        }
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