fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s3 = takes_and_gives_back(s1);
    println!("{}", calculate_length(&s3));

    let s4 = String::from("this is a string");
    let index = find_first_word(&s4);
    println!("{}", &s4[0..index]);
    let first_word = find_first_word_as_slice(&s4);
    println!("{}", first_word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(a_string: &String) -> usize {
    a_string.len()
}

fn find_first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    s.len()
}

fn find_first_word_as_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}