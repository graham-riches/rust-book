use crate::int_code::instructions;
use num_derive::FromPrimitive;

/// Common trait methods that an operation must satisfy
pub trait OpCode {
    /// Parses an instruction type from a slice of a program    
    fn parse_from_slice(program: &[i64]) -> Option<Self> where Self: Sized;

    /// Gets an offset to increase the instruction pointer by
    fn get_instruction_pointer_offset() -> i64;

    /// Applies an instruction to a program
    fn apply(&self, program: &mut [i64], instruction_pointer: i64, input: fn() -> i64, output: fn(i64) -> ()) -> i64;

    /// Gets the op code from an operation
    fn get_op_code() -> i64;
}

/// Enumeration of op code IDs
#[derive(FromPrimitive)]
pub enum OpCodeId {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Complete = 99
}

/// Variant of generic operations that implement the OpCode trait
pub enum Operation {
    Add {i: instructions::add::Add},
    Multiply {i: instructions::multiply::Multiply},
    Input {i: instructions::input::Input},
    Output {i: instructions::output::Output},
    Complete {i: instructions::complete::Complete},
    JumpIfTrue {i: instructions::jump_if_true::JumpIfTrue},
    JumpIfFalse {i: instructions::jump_if_false::JumpIfFalse},
    LessThan {i: instructions::less_than::LessThan},
    Equals {i: instructions::equals::Equals}
}
