pub mod op_code;
pub mod add;
pub mod multiply;
pub mod complete;
pub mod input;
pub mod output;
pub mod jump_if_true;
pub mod jump_if_false;
pub mod less_than;
pub mod equals;

use crate::lexer;
use num_traits::FromPrimitive;

use crate::instructions::{
    op_code::{OpCode, OpCodeId},
    add::Add,
    multiply::Multiply,
    input::Input, 
    output::Output,
    complete::Complete,
    jump_if_true::JumpIfTrue,
    jump_if_false::JumpIfFalse,
    less_than::LessThan,
    equals::Equals,
};

/// Creates a dynamic trait object from an instruction type and the current program context
pub fn parse_from_slice(instruction: lexer::InstructionType, program_context: &[i64]) -> Option<Box<dyn OpCode>> {
    let op: Box<dyn OpCode> = match FromPrimitive::from_i64(instruction.op_code) {
        Some(OpCodeId::Add)         => Box::new(Add::parse_from_slice(program_context).unwrap()),
        Some(OpCodeId::Multiply)    => Box::new(Multiply::parse_from_slice(program_context).unwrap()),
        Some(OpCodeId::Input)       => Box::new(Input::parse_from_slice(program_context).unwrap()),
        Some(OpCodeId::Output)      => Box::new(Output::parse_from_slice(program_context).unwrap()),
        Some(OpCodeId::JumpIfTrue)  => Box::new(JumpIfTrue::parse_from_slice(program_context).unwrap()),
        Some(OpCodeId::JumpIfFalse) => Box::new(JumpIfFalse::parse_from_slice(program_context).unwrap()),
        Some(OpCodeId::LessThan)    => Box::new(LessThan::parse_from_slice(program_context).unwrap()),
        Some(OpCodeId::Equals)      => Box::new(Equals::parse_from_slice(program_context).unwrap()),
        Some(OpCodeId::Complete)    => Box::new(Complete::parse_from_slice(program_context).unwrap()),
        None => return None
    };
    Some(op)    
}