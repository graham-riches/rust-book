use crate::instructions::op_code::OpCode;
use crate::lexer;
#[allow(unused_imports)]
use crate::parameters::ParameterMode;

const OP_CODE_ID: i64 = 2;
const INSTRUCTION_POINTER_OFFSET: i64 = 4;

#[derive(PartialEq, Debug)]
pub struct Multiply {
    arg1: lexer::Parameter,
    arg2: lexer::Parameter,
    output: lexer::Parameter
}

impl OpCode for Multiply {
    /// Parses a multiply instruction from a slice of a program
    fn parse_from_slice(program: &[i64]) -> Option<Multiply> {
        if program.len() < 4 {
            return None
        }

        let mut operation = match lexer::parse_instruction_type(program[0]) {
            Some(lexer::InstructionType{ op_code: OP_CODE_ID, a, b, c}) => Multiply{arg1: c, arg2: b, output: a},
            Some(_i) => return None,
            None => return None
        };

        operation.arg1.value = program[1];
        operation.arg2.value = program[2];
        operation.output.value = program[3];        
        Some(operation)
    }

    /// Applies a multiply operation on a program
    fn apply(&self, program: &mut [i64], instruction_pointer: i64, _input: fn() -> i64, _output: fn(i64) -> ()) -> i64 {
        let result = lexer::get_parameter_value(&self.arg1, &program) * lexer::get_parameter_value(&self.arg2, &program);     
        program[self.output.value as usize] = result;
        instruction_pointer + INSTRUCTION_POINTER_OFFSET
    }

    /// Gets the instruction pointer offset for a multiply
    fn get_instruction_pointer_offset(&self) -> i64 {
        INSTRUCTION_POINTER_OFFSET
    }

    /// Gets the operation code
    fn get_op_code(&self) -> i64 {
        OP_CODE_ID
    }
}


#[test]
fn test_parse_multiply_from_slice() {    
    let i1 = Multiply::parse_from_slice(&vec![1002, 1, 2, 3]).unwrap();
    assert_eq!(i1, Multiply{ arg1: lexer::Parameter{mode: ParameterMode::Position, value: 1}, 
                             arg2: lexer::Parameter{mode: ParameterMode::Immediate, value: 2}, 
                             output: lexer::Parameter{mode: ParameterMode::Position, value: 3}});

    let i2 = Multiply::parse_from_slice(&vec![1001, 1, 2, 3]);
    assert_eq!(true, i2.is_none());

    let i3 = Multiply::parse_from_slice(&vec![10002, 1, 2, 3]).unwrap();
    assert_eq!(i3, Multiply{ arg1: lexer::Parameter{mode: ParameterMode::Position, value: 1}, 
                             arg2: lexer::Parameter{mode: ParameterMode::Position, value: 2}, 
                             output: lexer::Parameter{mode: ParameterMode::Immediate, value: 3}});
}

#[test]
fn test_apply_multiply() {
    let mut program: Vec<i64> = vec![10002, 1, 2, 0];
    let instruction = Multiply::parse_from_slice(&program).unwrap();
    instruction.apply(&mut program, 0, || 0, |_x| () );
    assert_eq!(2, program[0]);

    let mut program: Vec<i64> = vec![11102, 5, 10, 0];
    let instruction = Multiply::parse_from_slice(&program).unwrap();
    instruction.apply(&mut program, 0, || 0, |_x| () );
    assert_eq!(50, program[0]);
}