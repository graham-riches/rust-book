use std::io::{stdin, stdout, Write};

fn main() {
    println!("Enter comma delimited floating point numbers: ");
    let mut s = String::new();
    stdout().flush();
    stdin().read_line(&mut s).expect("Could not read line");
    let sum: f32 = s.split_whitespace()
        .collect::<String>()
        .split(',')
        .map(|x| x.parse::<f32>().unwrap())
        .sum();
    println!("{}", sum);
}
