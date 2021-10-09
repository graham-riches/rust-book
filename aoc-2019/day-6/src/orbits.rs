use std::collections::{HashMap, HashSet};
use string_utils::string_utils;

#[derive(Debug)]
pub struct OrbitMap {
    map: HashMap<String, Node>
}

impl OrbitMap {
    /// Creates a new OrbitMap from vector of strings
    ///
    /// # Arguments
    /// * 'lines' - Vector of strings that contains the orbit mappings
    pub fn from_lines(lines: &Vec<String>) -> OrbitMap {
        let orbits = lines.iter()
            .map(|x| string_utils::split_into_tuple(&x, ')').unwrap())
            .collect::<Vec<(&str, &str)>>();

        let mut map: HashMap<String, Node> = HashMap::new();

        // Add all parent nodes and populate their child vectors
        for &(parent, child) in &orbits {
            match map.get_mut(parent) {
                Some(p) => { p.children.push(child.to_string()); }
                None => {
                    let mut new_parent = Node::new(parent);
                    new_parent.children.push(child.to_string());
                    map.insert(parent.to_string(), new_parent);
                }
            }
        }

        // Add any missing child nodes and populate all parents
        for &(parent, child) in &orbits {
            match map.get(child) {
                Some(_c) => (),
                None => {
                    let new_child = Node::new(child);
                    map.insert(child.to_string(), new_child);
                }
            };
            let c = map.get_mut(child).unwrap();
            c.parent = parent.to_string();
        }
        // Return the constructed map
        OrbitMap { map }
    }

    /// Calculates the number of total orbits that are nested starting at a particular parent
    pub fn get_child_orbits(&self, starting_planet: &str, initial_count: i64) -> i64 {
        let s = match self.map.get(starting_planet) {
            Some(p) => p.children.iter().map(|x| self.get_child_orbits(x, initial_count + 1)).sum(),
            None => 0,
        };
        s + initial_count
    }

    /// Gets the parent orbits for a planet and returns a HashSet containing the unique set
    /// of parent orbits
    ///
    /// # Arguments
    /// * 'planet' - String key for the starting planet
    /// * 'stop_planet' - String key for the final planet to stop the search at
    /// * 'set' - Mutable hashset that stores the set of planets that were accessed en-route to the current location
    fn get_orbit_parents(&self, planet: &str, stop_planet: &str, set: &mut HashSet<String>) -> () {
        match self.map.get(planet) {
            Some(p) => {
                if p.parent == stop_planet {
                    return;
                } else{
                    set.insert(p.parent.clone());
                    self.get_orbit_parents(&p.parent, stop_planet, set);
                }
            },
            None => ()
        }
    }
    /// Calculates the total orbital distance between two planets given a common intersection point
    ///
    /// # Arguments
    /// * 'first' - String key of the starting planet
    /// * 'second' - String key of the ending planet
    /// * 'common'- String key of a known planet that first and second are guaranteed to both orbit
    fn calculate_orbital_distance(&self, first: &str, second: &str, common: &str) -> i64 {
        let mut first_set: HashSet<String> = HashSet::new();
        let mut second_set: HashSet<String> = HashSet::new();
        self.get_orbit_parents(first, common, &mut first_set);
        self.get_orbit_parents(second, common, &mut second_set);
        (first_set.len() + second_set.len()) as i64
    }

    /// Calculates the number of orbital transfers required to jump between two planets
    ///
    /// # Arguments
    /// * 'first' - String key of the starting planet
    /// * 'second' - String key of the ending planet
    /// * 'common'- String key of a known planet that first and second are guaranteed to both orbit
    pub fn calculate_orbital_transfers(&self, first: &str, second: &str, common: &str) -> i64 {
        let mut first_set: HashSet<String> = HashSet::new();
        let mut second_set: HashSet<String> = HashSet::new();
        self.get_orbit_parents(first, common, &mut first_set);
        self.get_orbit_parents(second, common, &mut second_set);
        let intersections = first_set.intersection(&second_set).collect::<Vec<&String>>();
        intersections.iter().map(|x| self.calculate_orbital_distance(first, second, x)).min().unwrap()
    }

}

#[derive(Debug)]
struct Node {
    name: String,
    parent: String,
    children: Vec<String>
}

impl Node {
    fn new(name: &str) -> Node {
        Node {
            name: name.to_string(),
            parent: "".to_string(),
            children: vec![]
        }
    }
}
