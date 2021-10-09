mod int_code;

fn main() {
    let program = int_code::load_program_from_file("program.txt").expect("Could not read program");
    let output = |x| -> () { println!("Output: {}", x)};
    
    println!("Part one solution");
    let input = || {
        1
    };
    let mut p1 = program.clone();
    int_code::run_interpreter(&mut p1, input, output);

    println!("Part two solution");    
    let input = || {        
        5
    };    
    let mut p = program.clone();
    int_code::run_interpreter(&mut p, input, output);
}
