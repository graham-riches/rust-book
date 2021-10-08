use std::{
   fs::File,
   io::{self, BufRead, BufReader},
   path::Path,
};

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

static OP_JUMP_DISTANCE: i64 = 4;

#[derive(FromPrimitive)]
enum OpCode
{
    Add = 1,
    Multiply = 2,
    Complete = 99,
}

fn main() {
    let initial_program = lines_from_file("data.txt").expect("Could not read lines from file")
                         .first().expect("Could not extract token sequence")
                         .split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    
    // Part one solution
    let p1_result = run_program(12, 2, &initial_program);    
    println!("Final value: {}", p1_result);


    // Part two solution
    'outer: for i in 0..=99 {
        for j in 0..=99 {
            let result = run_program(i, j, &initial_program);
            if result == 19690720 {
                println!("100 x noun + verb: {}", 100 * i + j);
                break 'outer;
            }
        }
    }
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
   BufReader::new(File::open(filename)?).lines().collect()
}


fn int_code_interpreter(v: &mut[i64]) -> &mut[i64] {
    let mut ip = 0;
    loop {
        let (a, b) = (v[v[ip + 1] as usize], v[v[ip + 2] as usize]);
        match FromPrimitive::from_i64(v[ip]) {
            Some(OpCode::Add)      => v[v[ip + 3] as usize] = a + b,            
            Some(OpCode::Multiply) => v[v[ip + 3] as usize] = a * b,
            Some(OpCode::Complete) => break,
            None                   => break,
        };
        ip += OP_JUMP_DISTANCE as usize;
    }
    v
}

fn run_program(noun: i64, verb: i64, program: &Vec<i64>) -> i64 {
    let mut memory = program.clone();
    memory[1] = noun;
    memory[2] = verb;
    int_code_interpreter(&mut memory);
    memory[0]
}


#[test]
fn test_int_code_iterpreter() {
    assert_eq!(int_code_interpreter(& mut vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(int_code_interpreter(& mut vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(int_code_interpreter(& mut vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(int_code_interpreter(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}