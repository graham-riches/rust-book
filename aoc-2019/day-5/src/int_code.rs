pub mod instructions;
pub mod lexer;
pub mod parameters;

use num_traits::FromPrimitive;

use crate::int_code::instructions::{
    op_code::{OpCode, OpCodeId, Operation},
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

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Loads an int_code program from a file path
/// 
/// # Examples
/// ```
/// let program = int_code::load_program_from_file("file.txt").expect("Could not load program");
/// ```
pub fn load_program_from_file(filename: impl AsRef<Path>) -> Result<Vec<i64>, String> {
    let lines = match load_lines_from_file(filename) {
        Ok(l) => l,
        Err(e) => return Err(e.to_string())
    };

    let instructions = match lines.first() {
        Some(i) => i,
        None => return Err(String::from("No instructions found"))
    };
    Ok(parse_program_from_lines(&instructions))
}

/// Loads a vector of strings from a file
/// 
/// # Arguments
/// * `filename` - The path filename
/// 
/// # Examples
/// ```
/// let lines = load_lines_from_file("data.txt").expect("Could not load lines");
/// ```
fn load_lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}

/// Parses an int_code program from a vector of lines that contains an int_code program
/// 
/// # Examples
/// ```
/// let program = int_code::parse_program_from_lines(vec![1002,4,3,4]);
/// ```
fn parse_program_from_lines(program: &String) -> Vec<i64> {    
    program.split(",")
           .map(|x| x.parse::<i64>().unwrap())
           .collect::<Vec<i64>>()   
}

/// Run the int_code interpreter with a given input and output
/// 
/// # Arguments
/// * `v`      - Slice containing the program data
/// * `input`  - Function pointer for data input 
/// * `output` - Function pointer for data output
pub fn run_interpreter(v: &mut[i64], input: fn() -> i64, output: fn(i64) -> ()) -> () {
    let mut ip = 0;
    loop {        
        let instruction = match lexer::parse_instruction_type(v[ip]) {
            Some(i) => i,
            None    => return
        };

        let op: Operation = match FromPrimitive::from_i64(instruction.op_code) {
            Some(OpCodeId::Add)         => Operation::Add {i: Add::parse_from_slice(&v[ip..]).unwrap()},
            Some(OpCodeId::Multiply)    => Operation::Multiply {i: Multiply::parse_from_slice(&v[ip..]).unwrap()},
            Some(OpCodeId::Input)       => Operation::Input {i: Input::parse_from_slice(&v[ip..]).unwrap()},
            Some(OpCodeId::Output)      => Operation::Output {i: Output::parse_from_slice(&v[ip..]).unwrap()},
            Some(OpCodeId::JumpIfTrue)  => Operation::JumpIfTrue {i: JumpIfTrue::parse_from_slice(&v[ip..]).unwrap()},
            Some(OpCodeId::JumpIfFalse) => Operation::JumpIfFalse {i: JumpIfFalse::parse_from_slice(&v[ip..]).unwrap()},
            Some(OpCodeId::LessThan)    => Operation::LessThan {i: LessThan::parse_from_slice(&v[ip..]).unwrap()},
            Some(OpCodeId::Equals)      => Operation::Equals {i: Equals::parse_from_slice(&v[ip..]).unwrap()},
            Some(OpCodeId::Complete)    => Operation::Complete {i: Complete::parse_from_slice(&v[ip..]).unwrap()},            
            None => return
        };
        
        let new_ip = match op {
            Operation::Add{i}         => i.apply(&mut v[..], ip as i64, input, output),
            Operation::Multiply{i}    => i.apply(&mut v[..], ip as i64, input, output),
            Operation::Input{i}       => i.apply(&mut v[..], ip as i64, input, output),
            Operation::Output{i}      => i.apply(&mut v[..], ip as i64, input, output),
            Operation::JumpIfTrue{i}  => i.apply(&mut v[..], ip as i64, input, output),
            Operation::JumpIfFalse{i} => i.apply(&mut v[..], ip as i64, input, output),
            Operation::LessThan{i}    => i.apply(&mut v[..], ip as i64, input, output),
            Operation::Equals{i}      => i.apply(&mut v[..], ip as i64, input, output),
            Operation::Complete{i: _} => return
        };
        ip = new_ip as usize; 
    }
}
