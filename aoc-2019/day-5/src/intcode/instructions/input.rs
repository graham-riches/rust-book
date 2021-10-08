use crate::intcode::instructions::op_code::OpCode;
use crate::intcode::lexer;
use crate::intcode::parameters::ParameterMode;

const OP_CODE_ID: i64 = 3;
const INSTRUCTION_POINTER_OFFSET: i64 = 2;

#[derive(PartialEq, Debug)]
pub struct Input { 
    arg: lexer::Parameter
}

impl OpCode for Input {
    /// Parses an input instruction from a slice of a program
    fn parse_from_slice(program: &[i64]) -> Option<Input> {        
        let mut operation = match lexer::parse_instruction_type(program[0]) {
            Some(lexer::InstructionType{ op_code: OP_CODE_ID, a: _, b: _, c}) => Input{ arg: c},
            Some(_i) => return None,
            None => return None
        };
        operation.arg.value = program[1];
        Some(operation)
    }

    /// Applies an input operation on a program
    fn apply(&self, program: &mut [i64], instruction_pointer: i64, input: fn() -> i64, _output: fn(i64) -> ()) -> i64 {
        let output = lexer::get_parameter_value(&self.arg, &program);        
        program[self.arg.value as usize] = input();
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
fn test_parse_input_from_slice() {    
    let i1 = Input::parse_from_slice(&vec![3, 3, 2, 5]).unwrap();
    assert_eq!(i1, Input{ arg: lexer::Parameter{ mode: ParameterMode::Position, value: 3}});

    let i2 = Input::parse_from_slice(&vec![103, 1, 2, 3]).unwrap();
    assert_eq!(i2, Input{ arg: lexer::Parameter{ mode: ParameterMode::Immediate, value: 1}});    
}

#[test]
fn test_apply_input() {
    let mut program: Vec<i64> = vec![3, 3, 2, 0];
    let i1 = Input::parse_from_slice(&program).unwrap();
    i1.apply(&mut program, 0, || 69, |_x| ());
    assert_eq!(program[3], 69);

    let mut program: Vec<i64> = vec![3, 1, 2, 0];
    let i1 = Input::parse_from_slice(&program).unwrap();
    i1.apply(&mut program, 0, || 69, |_x| ());
    assert_eq!(program[1], 69);
}