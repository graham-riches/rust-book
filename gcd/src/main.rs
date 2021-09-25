use std::str::FromStr;
use std::env;

fn main() {
    let numbers: Vec<u64> = env::args().skip(1).map(|x| u64::from_str(&x).expect("Error parsing argument")).collect();

    if numbers.len() == 0 {
        eprintln!("Usage: gcd <number 1> ...");
        std::process::exit(1);
    }

    let d = numbers.iter().skip(1).fold(numbers[0], |d, m| gcd(d, *m));
    println!("GCD of {:?} is {}", numbers, d);
}


fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n
    }
    n
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19), 3 * 11);
}
