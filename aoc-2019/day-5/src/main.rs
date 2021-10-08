mod intcode;

fn main() {
    let program = intcode::load_program_from_file("program.txt").expect("Could not read program");    
    let output = |x| -> () { println!("Output: {}", x)};
    
    println!("Part one solution");
    let input = || {
        1
    };
    let mut p1 = program.clone();
    intcode::run_interpreter(&mut p1, input, output);

    println!("Part two solution");    
    let input = || {        
        5
    };    
    let mut p = program.clone();
    intcode::run_interpreter(&mut p, input, output);
}
