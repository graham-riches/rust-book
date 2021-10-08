fn main() {
    let p1 = (171309..=643603).filter(|x| part_one(*x)).count();
    println!("Part one solution: {}", p1);

    let p2 = (171309..=643603).filter(|x| part_two(*x)).count();
    println!("Part two solution: {}", p2);
}

fn part_one(password: u32) -> bool {
    let digits = digit_to_vec(password);
    is_sorted(&digits, |x, y| x <= y) && contains_repeat_sequence(&digits)
}

fn part_two(password: u32) -> bool {
    let digits = digit_to_vec(password);
    is_sorted(&digits, |x, y| x <= y) && contains_repeat_sequence_without_groups(&digits)
}

fn digit_to_vec(d: u32) -> Vec<u32> {
    d.to_string()
     .chars()
     .map(|x| x.to_digit(10).unwrap())
     .collect::<Vec<u32>>()
}

/// Checks if a given slice ever contains two equal elements sequentially
/// 
/// # Examples
/// ```
/// assert_eq!(contains_repeat_sequence(&vec![1, 2, 2, 3, 4]), true);
/// assert_eq!(contains_repeat_sequence(&vec![1, 2, 3, 4, 5]), false);
/// ```
fn contains_repeat_sequence<T>(data: &[T]) -> bool
where 
    T: Ord,
{
    (0..data.len() - 1).any(|i| data[i] == data[i + 1])
}

#[test]
fn test_contains_repeat_sequence(){
    assert_eq!(contains_repeat_sequence(&vec![1, 2, 2, 3, 4]), true);
    assert_eq!(contains_repeat_sequence(&vec![1, 2, 3, 4, 5]), false);
}

/// Checks if a slice is sorted according to a given binary predicate function
/// 
/// # Examples
/// ```
/// assert_eq!(is_sorted(&vec![1, 2, 3, 4, 5], |x, y| x <= y), true);
/// assert_eq!(is_sorted(&vec![5, 4, 3, 2, 1], |x, y| x >= y), true);
/// ```
fn is_sorted<T, F: Fn(&T, &T) -> bool>(data: &[T], f: F) -> bool
where
    T: Ord,
{    
    (0..data.len() - 1).all(|i| f(&data[i], &data[i + 1]))
}

#[test]
fn test_is_sorted() {
    assert_eq!(is_sorted(&vec![1, 2, 3, 4, 5], |x, y| x <= y), true);
    assert_eq!(is_sorted(&vec![1, 2, 3, 4, 0], |x, y| x <= y), false);
    assert_eq!(is_sorted(&vec![1, 1, 1, 1, 1], |x, y| x <= y), true);
    assert_eq!(is_sorted(&vec![1, 2, 3, 4, 5], |x, y| x >= y), false);
    assert_eq!(is_sorted(&vec![5, 4, 3, 2, 1], |x, y| x >= y), true);
    assert_eq!(is_sorted(&vec![1, 1, 1, 1, 1], |x, y| x >= y), true);
}

/// Checks if a sequence contains two repeating elements that are not part of a larger
/// group of repeating elements
fn contains_repeat_sequence_without_groups(data: &[u32]) -> bool {
    match data.len() {
        0 => false,
        1 => false,
        _ => {
            let diff = (0..data.len() - 1).map(|i| data[i] as i32 - data[i + 1] as i32).collect::<Vec<i32>>();
            match diff.len() {
                1 => diff[0] == 0,
                2 => diff.iter().filter(|x| **x != 0).count() == 1,
                _ => {                    
                    if (diff[0] == 0 && diff[1] != 0) || (diff[diff.len() - 1] == 0 && diff[diff.len() - 2] != 0){
                        true
                    } else {
                        (1..diff.len() - 1).any(|i| (diff[i] == 0) && (diff[i - 1] != 0) && (diff[i + 1]) != 0)
                    }                    
                }
            }
        }
    }
}


#[test]
fn test_contains_repeat_sequence_without_groups() {
    assert_eq!(contains_repeat_sequence_without_groups(&vec![1, 2, 3, 4, 5]), false);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![1, 2, 2, 3, 4]), true);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![1, 2, 2, 2, 3]), false);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![1, 1, 2, 3, 4]), true);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![1, 2, 3, 4, 4]), true);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![1, 1, 1]), false);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![1, 1, 0]), true);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![0, 1, 1]), true);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![0, 1]), false);
    assert_eq!(contains_repeat_sequence_without_groups(&vec![1, 1]), true);
}
