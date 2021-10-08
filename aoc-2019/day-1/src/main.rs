use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("input.txt").expect("Could not read lines from file");
    let mass_sum: i64 = lines.iter()
                             .map(|x| calculate_fuel_to_launch(x.parse::<i64>().unwrap()))
                             .sum();
    
    println!("Part 1: Total mass: {}", mass_sum);

    let fuel_mass_sum: i64 = lines.iter()
                                  .map(|x| calculate_with_extra_fuel(x.parse::<i64>().unwrap()))
                                  .sum();
    println!("Part 2: Total mass: {}", fuel_mass_sum );
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn calculate_fuel_to_launch(mass: i64) -> i64 {
    mass / 3 - 2
}

fn calculate_with_extra_fuel(mass: i64) -> i64 {
    let mut fuel_mass = calculate_fuel_to_launch(mass);
    let mut extra_fuel_mass = calculate_fuel_to_launch(fuel_mass);
    while extra_fuel_mass > 0 {
        fuel_mass += extra_fuel_mass;
        extra_fuel_mass = calculate_fuel_to_launch(extra_fuel_mass);
    }
    fuel_mass
}


#[test]
fn test_calculate_fuel_to_launch() {
    assert_eq!(calculate_fuel_to_launch(12), 2);
    assert_eq!(calculate_fuel_to_launch(14), 2);
    assert_eq!(calculate_fuel_to_launch(1969), 654);
    assert_eq!(calculate_fuel_to_launch(100756), 33583);
}


#[test]
fn test_get_total_fuel() {
    assert_eq!(calculate_with_extra_fuel(14), 2);
    assert_eq!(calculate_with_extra_fuel(1969), 966);
    assert_eq!(calculate_with_extra_fuel(100756), 50346);
}