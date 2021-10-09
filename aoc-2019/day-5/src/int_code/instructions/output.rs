use crate::int_code::instructions::op_code::OpCode;
use crate::int_code::lexer;
use crate::int_code::parameters::ParameterMode;

const OP_CODE_ID: i64 = 4;
const INSTRUCTION_POINTER_OFFSET: i64 = 2;

#[derive(PartialEq, Debug)]
pub struct Output { 
    arg: lexer::Parameter
}

impl OpCode for Output {
    /// Parses an output instruction from a slice of a program
    fn parse_from_slice(program: &[i64]) -> Option<Output> {        
        let mut operation = match lexer::parse_instruction_type(program[0]) {
            Some(lexer::InstructionType{ op_code: OP_CODE_ID, a: _, b: _, c}) => Output{ arg: c},
            Some(_i) => return None,
            None => return None
        };
        operation.arg.value = program[1];
        Some(operation)
    }

    /// Applies an output operation on a program
    fn apply(&self, program: &mut [i64], instruction_pointer: i64, _input: fn() -> i64, output: fn(i64) -> ()) -> i64 {
        let value = lexer::get_parameter_value(&self.arg, &program);
        output(value);
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
fn test_parse_output_from_slice() {    
    let i1 = Output::parse_from_slice(&vec![4, 3, 2, 5]).unwrap();
    assert_eq!(i1, Output{ arg: lexer::Parameter{ mode: ParameterMode::Position, value: 3}});

    let i2 = Output::parse_from_slice(&vec![104, 1, 2, 3]).unwrap();
    assert_eq!(i2, Output{ arg: lexer::Parameter{ mode: ParameterMode::Immediate, value: 1}});    
}

#[test]
fn test_apply_output() {
    let mut program: Vec<i64> = vec![4, 3, 2, 0];
    let i1 = Output::parse_from_slice(&program).unwrap();    
    i1.apply(&mut program, 0, || 69, |x| assert_eq!(x, 0));    

    let mut program: Vec<i64> = vec![104, 1, 2, 0];
    let i1 = Output::parse_from_slice(&program).unwrap();
    i1.apply(&mut program, 0, || 69, |x| assert_eq!(x, 1));    
}