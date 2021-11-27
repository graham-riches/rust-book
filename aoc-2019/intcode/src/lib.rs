pub mod instructions;
pub mod lexer;
pub mod parameters;

use crate::instructions::parse_from_slice;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Loads an intcode program from a file path
/// 
/// # Examples
/// ```ignoreuse crate::parameters::ParameterMode;
/// let program = intcode::load_program_from_file("file.txt").expect("Could not load program");
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
/// ```ignore
/// let lines = load_lines_from_file("data.txt").expect("Could not load lines");
/// ```
fn load_lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}

/// Parses an intcode program from a vector of lines that contains an intcode program
/// 
/// # Examples
/// ```ignore
/// let program = intcode::parse_program_from_lines(vec![1002,4,3,4]);
/// ```
fn parse_program_from_lines(program: &String) -> Vec<i64> {    
    program.split(",")
           .map(|x| x.parse::<i64>().unwrap())
           .collect::<Vec<i64>>()   
}

/// Run the intcode interpreter with a given input and output
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
        // TODO: probably remove the unwrap here
        let op = parse_from_slice(instruction, &v[ip..]).unwrap();
        ip = op.apply(&mut v[..], ip as i64, input, output) as usize;
    }
}
