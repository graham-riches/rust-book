use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    collections::HashSet,
    hash::{Hash, Hasher},
 };

 #[derive(Copy, Clone)]
struct CoordinateCost {
    x: i64,
    y: i64,
    cost: i64
}

impl Hash for CoordinateCost {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for CoordinateCost {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for CoordinateCost {}

fn main() {
    let lines = lines_from_file("data.txt").expect("Could not read input file");
    let p1 = part_one_solution(&lines);
    println!("Part one solution: {}", p1);

    let p2 = part_two_solution(&lines);
    println!("Part two solution: {}", p2);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn wire_map_to_coordinate_set(path: &Vec<String>) -> HashSet<CoordinateCost> {
    let mut set = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut cost = 0;
    for movement in path {
        let distance = &movement[1..].parse::<i64>().expect("Could not parse instruction as integral value");
        match &movement[0..1] {
            "R" => {
                for _i in 1..=*distance {
                    x += 1;
                    cost += 1;
                    set.insert(CoordinateCost {x, y, cost});
                }},                
            "L" => {
                for _i in 1..=*distance {
                    x -= 1;
                    cost += 1;
                    set.insert(CoordinateCost {x, y, cost});
                }},
            "U" => {
                for _j in 1..=*distance {
                    y += 1;
                    cost += 1;
                    set.insert(CoordinateCost {x, y, cost});
                }},
            "D" => {
                for _j in 1..=*distance {
                    y -= 1;
                    cost += 1;
                    set.insert(CoordinateCost {x, y, cost});
            }},
            _   => (),
        };     
    }
    set
}

fn part_one_solution(lines: &Vec<String>) -> i64 {
    let paths = lines.iter()
                     .map(|x| x.split(",").map(|y| y.to_string()).collect::<Vec<String>>())
                     .collect::<Vec<Vec<String>>>();

    let points = paths.iter()
                      .map(|x| wire_map_to_coordinate_set(x))
                      .collect::<Vec<HashSet<CoordinateCost>>>();

    points[0].intersection(&points[1])             
             .map(|c| c.x.abs() + c.y.abs())
             .fold(std::i64::MAX, std::cmp::min)
}

fn part_two_solution(lines: &Vec<String>) -> i64 {
    let paths = lines.iter()
                     .map(|x| x.split(",").map(|y| y.to_string()).collect::<Vec<String>>())
                     .collect::<Vec<Vec<String>>>();

    let points = paths.iter()
                      .map(|x| wire_map_to_coordinate_set(x))
                      .collect::<Vec<HashSet<CoordinateCost>>>();

    let mut costs = Vec::new();
    for point in &points[0] {
        if points[1].contains(&point) {
            let c2 = match points[1].get(&point) {
                Some(c) => c.cost,
                None => 0,
            };
            costs.push(point.cost + c2);
        }
    }
    costs.iter().fold(std::i64::MAX, |a, &b| a.min(b))
}


#[test]
fn test_part_one_solution() {
    assert_eq!(part_one_solution(&vec![String::from("R8,U5,L5,D3"), 
                                       String::from("U7,R6,D4,L4")]), 6);
    assert_eq!(part_one_solution(&vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), 
                                       String::from("U62,R66,U55,R34,D71,R55,D58,R83")]), 159);
    assert_eq!(part_one_solution(&vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), 
                                       String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")]), 135);
}

#[test]
fn test_part_two_solution() {
    assert_eq!(part_two_solution(&vec![String::from("R8,U5,L5,D3"), 
                                       String::from("U7,R6,D4,L4")]), 30);
    assert_eq!(part_two_solution(&vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), 
                                       String::from("U62,R66,U55,R34,D71,R55,D58,R83")]), 610);
    assert_eq!(part_two_solution(&vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), 
                                       String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")]), 410);
}