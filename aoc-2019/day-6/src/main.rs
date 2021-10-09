use string_utils::string_utils;
mod orbits;

fn main() {
    // Part 1 Solution:
    let data = string_utils::read_lines_from_file("data.txt").expect("Could not parse file");
    let orbit_map = orbits::OrbitMap::from_lines(&data);
    let count = orbit_map.get_child_orbits("COM", 0);
    println!("Part one: total orbit count: {}", count);

    // Part 2 Solution:
    let result = orbit_map.calculate_orbital_transfers("YOU", "SAN", "COM");
    println!("Part two: total orbital transfers: {}", result);
}
