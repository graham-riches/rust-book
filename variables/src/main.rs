use std::num::Wrapping;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let mut y = Wrapping(255u8);
    y += Wrapping(1u8);
    println!("The value of y is: {}", y);

    let tup: (i32, f64, u8) = (5000, 6.4, 1);
    let (a, b, c) = tup;
    println!("a {}, b {}, c {}", a, b, c);

    let t: [i32; 5] = [1, 2, 3, 4, 5];
    println!("t[0]: {}", t[0]);
}
