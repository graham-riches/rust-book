use crate::int_code::instructions::op_code::OpCode;
use crate::int_code::lexer;

const OP_CODE_ID: i64 = 99;
const INSTRUCTION_POINTER_OFFSET: i64 = 1;

#[derive(PartialEq, Debug)]
pub struct Complete { }

impl OpCode for Complete {
    /// Parses a complete instruction from a slice of a program
    fn parse_from_slice(program: &[i64]) -> Option<Complete> {        
        match lexer::parse_instruction_type(program[0]) {
            Some(lexer::InstructionType{ op_code: OP_CODE_ID, a: _, b: _, c: _}) => Some(Complete{}),
            Some(_i) => None,
            None => None
        }        
    }

    /// Applies a complete operation on a program
    fn apply(&self, _program: &mut [i64], instruction_pointer: i64, _input: fn() -> i64, _output: fn(i64) -> ()) -> i64 {
        instruction_pointer + INSTRUCTION_POINTER_OFFSET
    }

    /// Gets the instruction pointer offset for a complete
    fn get_instruction_pointer_offset() -> i64 {
        INSTRUCTION_POINTER_OFFSET
    }

    /// Gets the operation code
    fn get_op_code() -> i64 {
        OP_CODE_ID
    }
}


#[test]
fn test_parse_complete_from_slice() {    
    Complete::parse_from_slice(&vec![11199, 1, 2, 3]).unwrap();
    Complete::parse_from_slice(&vec![10199, 1, 2, 3]).unwrap();
    Complete::parse_from_slice(&vec![99, 1, 2, 3]).unwrap();
}
