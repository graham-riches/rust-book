fn main() {
    another_function();
    function_taking_i32(5);
    let y = returns_five();
    println!("Y is: {}", y);
}

fn another_function() {
    println!("Another function");
}

fn function_taking_i32(x: i32) {
    println!("Value of  is {}", x);
}

fn returns_five() -> u32 {
    5
}