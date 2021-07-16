// Intcode program will control the repair droid.

// Remote control executes steps:
// Accepts a movement command via an input instruction
// Sends movement command to repair droid
// Waits for repair droid to finish movement
// Reports on the status of the repair droid via an output instruction.

// Movement commands: North (1), South (2), West (3), East (4).  Other commands are invalid.

// Droid can reply with status codes:
// 0: wall - position has not changed
// 1: moved one step in the requested direction.
// 2: moved one step in the requested direction, new position is the location of the oxygen system.

use crate::computer::Computer;
use std::collections::{VecDeque, HashSet, HashMap};
use std::ops::Add;

// What is the fewest number of movement commands required to move the repair droid from
// its starting position to the location fo the oxygen system?
pub mod computer;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North, South, West, East,
}

impl Direction {
    fn from_code(code: i32) -> Direction {
        match code {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("Invalid direction code."),
        }
    }

    fn to_code(&self) -> i32 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    /// Returns all directions.
    fn values() -> Vec<Self> {
        vec![Direction::North, Direction::South, Direction::East, Direction::West]
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Status {
    Wall, Open, Oxygen,
}

impl Status {
    fn from_code(code: i32) -> Status {
        match code {
            0 => Status::Wall,
            1 => Status::Open,
            2 => Status::Oxygen,
            _ => panic!("Invalid status code"),
        }
    }

    fn to_code(&self) -> i32 {
        match self {
            Status::Wall => 0,
            Status::Open => 1,
            Status::Oxygen => 2,
        }
    }
}

/// Position in space.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    /// Constructs a new Position at the given point.
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new(0, 0)
    }
}

impl Add<&Direction> for &Position {
    type Output = Position;

    fn add(self, rhs: &Direction) -> Self::Output {
        match *rhs {
            Direction::North => Position::new(self.x, self.y - 1),
            Direction::South => Position::new(self.x, self.y + 1),
            Direction::West => Position::new(self.x - 1, self.y),
            Direction::East => Position::new(self.x + 1, self.y),
        }
    }
}

/// Moves the droid in the given direction, returning its status.
fn move_droid(computer: &mut Computer, direction: &Direction) -> Status {
    computer.input(direction.to_code());
    Status::from_code(computer.run().unwrap())
}

/// Returns the fewest number of movement commands required to move the repair droid from its
/// starting position to the location of the oxygen system.
pub fn shortest_path_bfs() -> usize {
    #[derive(Debug)]
    struct ToVisit {
        position: Position,
        path: Vec<Direction>,
    }

    let mut computer = Computer::load("input.txt").unwrap();
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();

    visited.insert(Position::default());
    to_visit.push_back(ToVisit {
        position: Position::default(),
        path: Vec::new(),
    });

    while let Some(node) = to_visit.pop_front() {

        // Starting at the origin, follow the node's path.
        for dir in &node.path {
            if move_droid(&mut computer, dir) != Status::Open {
                panic!("Followed path into a non-open square.");
            }
        }

        // Explore the node's neighbors, marking open squares as ones to explore.
        for neighbor_dir in Direction::values() {
            let neighbor = &node.position + &neighbor_dir;
            if !visited.insert(neighbor.clone()) {
                continue;
            }

            match move_droid(&mut computer, &neighbor_dir) {
                Status::Wall => {},
                Status::Open => {
                    if move_droid(&mut computer, &neighbor_dir.reverse()) != Status::Open {
                        panic!("Backed up into a non-open square.");
                    }

                    let mut neighbor_path = Vec::new();
                    for step in &node.path {
                        neighbor_path.push(step.clone());
                    }
                    neighbor_path.push(neighbor_dir.clone());

                    to_visit.push_back(ToVisit {
                        position: neighbor,
                        path: neighbor_path,
                    })
                },
                Status::Oxygen => {
                    return node.path.len() + 1;
                },
            }
        }

        // Return to the origin.
        for dir in node.path.iter().rev() {
            if move_droid(&mut computer, &dir.reverse()) != Status::Open {
                panic!("Returned to origin through a non-open square.");
            }
        }
    }

    panic!("No path to oxygen found.")
}

/// Fully explores the map, returning a map of position to status.
pub fn explore_map() -> HashMap<Position, Status> {
    let mut computer = Computer::load("input.txt").unwrap();

    // Depth-first traversal of the map.
    let mut map = HashMap::new();
    let mut position = Position::default();
    let mut to_visit = Vec::new();

    map.insert(position.clone(), Status::Open);
    for dir in Direction::values() {
        to_visit.push(dir);
    }

    while let Some(dir) = to_visit.pop() {
        let neighbor_pos = &position + &dir;
        let existing_square = map.get(&neighbor_pos);

        match existing_square {
            Some(&Status::Open) | Some(&Status::Oxygen) => {
                // Backtrack through an open square.
                position = &position + &dir;
                if move_droid(&mut computer, &dir) != *existing_square.unwrap() {
                    panic!("Found a different square while backtracking.");
                }
            },
            Some(&Status::Wall) => {},
            None => {
                // Move in the new direction, recording what we find and a trail to return.
                let neighbor_square = move_droid(&mut computer, &dir);
                map.insert(neighbor_pos.clone(), neighbor_square.clone());

                if neighbor_square == Status::Open || neighbor_square == Status::Oxygen {
                    position = &position + &dir;
                    to_visit.push(dir.reverse());

                    for around_dir in Direction::values() {
                        let around_pos = &neighbor_pos + &around_dir;
                        if !map.contains_key(&around_pos) {
                            to_visit.push(around_dir);
                        }
                    }
                }
            }
        }
    }

    map
}

/// Returns the shortest number of steps it takes to get to the oxygen system.
pub fn shortest_path(oxygen_dist: &HashMap<Position, usize>) -> usize {
    oxygen_dist[&Position::default()]
}

/// Returns the number of minutes that it takes to fill the map with oxygen.  Oxygen starts
/// at the oxygen system, and takes 1 minute to spread to adjacent open locations.
pub fn oxygen(oxygen_dist: &HashMap<Position, usize>) -> usize {
    *oxygen_dist.values().max().unwrap()
}

/// Computes the shortest path from oxygen to each open square in the map.
pub fn oxygen_distance(map: &HashMap<Position, Status>) -> HashMap<Position, usize> {
    // DFS through open nodes from the oxygen system until all of the open squares are explored.
    let num_open = map.values()
        .filter(|&square| *square == Status::Open)
        .count();

    let oxygen_pos = map.iter()
        .filter_map(|(pos, square)| if *square == Status::Oxygen { Some(pos.clone()) } else { None })
        .next()
        .unwrap();

    #[derive(Debug)]
    struct ToVisit {
        position: Position,
        dist: usize,
    }

    let mut dist = HashMap::new();
    let mut to_visit = VecDeque::new();

    to_visit.push_back(ToVisit {
        position: oxygen_pos,
        dist: 0
    });

    while let Some(node) = to_visit.pop_front() {

        dist.insert(node.position.clone(), node.dist);

        for dir in Direction::values() {
            let neighbor_pos = &node.position + &dir;
            if map[&neighbor_pos] == Status::Open && !dist.contains_key(&neighbor_pos) {
                to_visit.push_back(ToVisit {
                    position: neighbor_pos,
                    dist: node.dist + 1,
                })
            }
        }
    }

    dist
}

/// Prints the map.
pub fn print_map(map: &HashMap<Position, Status>, highlight: Vec<Position>) {
    let (min_x, max_x, min_y, max_y) = bounds(map);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Position::new(x, y);
            if highlight.contains(&pos) {
                print!("@");
            } else {
                print!("{}", match map.get(&pos) {
                    Some(Status::Oxygen) => "O",
                    Some(Status::Wall) => "#",
                    Some(Status::Open) => ".",
                    None => " ",
                });
            }
        }

        println!();
    }
}

/// Returns the min x, max x, min y, max y bounds of the map.
fn bounds(map: &HashMap<Position, Status>) -> (i32, i32, i32, i32) {
    map.keys().fold((0, 0, 0, 0), |bounds, pos| (
        bounds.0.min(pos.x),
        bounds.1.max(pos.x),
        bounds.2.min(pos.y),
        bounds.3.max(pos.y),
    ))
}