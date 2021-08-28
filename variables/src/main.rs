
use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100000;

    let z = 5;
    let z = z + 1;
    let z = z + 2;

    println!("The value of z is: {}", z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    println!("{} - {}", a[0], b[0]);
    
    // try to access invalid array memory
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number");
    
    let element = a[index];
    println!("a[{}] = {}", index, element);
    
}
