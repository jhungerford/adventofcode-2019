// @: entrance, .: passage, #: wall, lowercase letters: keys, uppercase letters: doors.
// keys open doors with matching letters.
// Part 1: starting at the entrance, how many steps are in the shortest path that collects all keys?

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use bit_set::BitSet;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Square {
    Entrance,
    Open,
    Wall,
    Key(char),
    Door(char),
    Intersection,
    DeadEnd,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '@' => Square::Entrance,
            '.' => Square::Open,
            '#' => Square::Wall,
            _ if c.is_lowercase() => Square::Key(c),
            _ if c.is_uppercase() => Square::Door(c),
            _ => panic!("Invalid square: '{}'", c),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up, Down, Left, Right,
}

impl Direction {
    fn values() -> Vec<Direction> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    /// Creates a new position at the given row and column.
    pub fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Position::new(self.row - 1, self.col),
            Direction::Down => Position::new(self.row + 1, self.col),
            Direction::Left => Position::new(self.row, self.col - 1),
            Direction::Right => Position::new(self.row, self.col + 1),
        }
    }
}


#[derive(Debug)]
struct RawEdge {
    start: Position,
    end: Position,
    start_square: Square,
    end_square: Square,
    length: usize,
}

#[derive(Debug)]
struct RawGraph {
    edges: HashMap<Position, Vec<RawEdge>>,
}

impl From<&Map> for RawGraph {
    fn from(map: &Map) -> Self {
        /// Position in the map that will form a graph node.
        #[derive(Debug)]
        struct PointOfInterest {
            position: Position,
            direction: Direction,
            square: Square,
        }

        // Scan the map for points of interest that will form the edges of the graph.
        let points_of_interest = (1..(map.squares.len() - 1))
            .flat_map(|row| (1..(map.squares[row].len() - 1))
                .flat_map(move |col| {
                    let position = Position::new(row, col);
                    let non_wall_neighbors = Direction::values().into_iter()
                        .filter(|&direction| map.get(position + direction) != Square::Wall)
                        .collect::<Vec<Direction>>();

                    let maybe_square = match map.get(position) {
                        Square::Open if non_wall_neighbors.len() == 1 => Some(Square::DeadEnd),
                        Square::Open if non_wall_neighbors.len() > 2 => Some(Square::Intersection),
                        s @ Square::Entrance | s @ Square::Key(_) | s @ Square::Door(_) => Some(s),
                        _ => None,
                    };

                    maybe_square.map(|square| non_wall_neighbors.into_iter()
                        .map(|direction| PointOfInterest { position: position.clone(), direction, square })
                        .collect::<Vec<PointOfInterest>>()
                    ).unwrap_or(Vec::new())
                }))
            .collect::<Vec<PointOfInterest>>();

        // Explore outwards from each point of interest to build the edges of the graph.
        let poi_positions = points_of_interest.iter()
            .map(|poi| poi.position)
            .collect::<HashSet<Position>>();

        let edges = points_of_interest.iter().map(|poi| {
            let mut dir = poi.direction;
            let mut pos = poi.position + dir;
            let mut length = 1;

            while !poi_positions.contains(&pos) {
                dir = Direction::values().into_iter()
                    .filter(|&direction| direction != dir.reverse() && map.get(pos + direction) != Square::Wall)
                    .next().unwrap();
                pos = pos + dir;
                length += 1;
            }

            RawEdge {
                start: poi.position,
                end: pos,
                start_square: poi.square,
                end_square: map.get(pos),
                length
            }
        }).collect::<Vec<RawEdge>>();

        RawGraph {
            edges: edges.into_iter().map(|edge| (edge.start, edge)).into_group_map()
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Keys {
    picked_up: BitSet,
}

impl Debug for Keys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.picked_up.iter().map(|num| (num + 'a' as usize) as u8 as char).join(", "))
    }
}

impl Keys {
    /// Creates a new Keys where none of the keys have been picked up.
    fn new() -> Self {
        Keys {
            picked_up: BitSet::new()
        }
    }

    /// Returns whether the given key has been picked up.
    fn has(&self, key: char) -> bool {
        self.picked_up.contains(key as usize - 'a' as usize)
    }

    /// Returns a new Keys that contains all of these keys, plus the given key.
    fn pick_up(&self, key: char) -> Self {
        let mut new_keys = self.clone();
        new_keys.picked_up.insert(key as usize - 'a' as usize);
        new_keys
    }

    /// Returned whether the given number of keys have been picked up.
    fn all_picked_up(&self, num_keys: usize) -> bool {
        self.picked_up.len() == num_keys
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    squares: Vec<Vec<Square>>,
    entrances: Vec<Position>,
    num_keys: usize,
}

impl Map {
    /// Loads a map from the given file.
    pub fn load(filename: &str) -> Map {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut squares = Vec::new();
        let mut entrances= Vec::new();
        let mut num_keys = 0;

        for (row, line) in f.lines().enumerate() {
            let mut row_squares = Vec::new();

            for (col, c) in line.unwrap().chars().enumerate() {
                let square: Square = Square::from(c);

                match &square {
                    Square::Entrance => entrances.push(Position::new(row, col)),
                    Square::Key(_) => num_keys += 1,
                    _ => {},
                }

                row_squares.push(square);
            }

            squares.push(row_squares);
        }

        Map { squares, entrances, num_keys }
    }

    /// Returns the square at the given position.
    pub fn get(&self, position: Position) -> Square {
        self.squares[position.row][position.col]
    }

    /// Returns the shortest number of steps that collects all of the keys.
    pub fn all_keys_steps(&self) -> usize {
        // Convert the map into a raw graph.  Nodes include intersections, dead ends, etc.
        let raw_graph = RawGraph::from(self);

        #[derive(Debug, Eq, PartialEq, Hash)]
        struct Visited {
            position: Position,
            keys: Keys,
        }

        #[derive(Debug, Eq, PartialEq)]
        struct ToVisit {
            position: Position,
            length: usize,
            keys: Keys
        }

        impl Ord for ToVisit {
            fn cmp(&self, other: &Self) -> Ordering {
                // Doors that we don't have the key for yet come later.

                other.length.cmp(&self.length)
            }
        }

        impl PartialOrd for ToVisit {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();

        for entrance in &self.entrances {
            visited.insert(Visited {
                position: *entrance,
                keys: Keys::new(),
            });

            to_visit.push(ToVisit {
                position: *entrance,
                length: 0,
                keys: Keys::new(),
            });
        }


        while let Some(node) = to_visit.pop() {
            println!("Visiting {:?}", node);
            println!("  to_visit {:?}", to_visit);

            if node.keys.all_picked_up(self.num_keys) {
                return node.length;
            }

            for edge in &raw_graph.edges[&node.position] {

                // Pick up keys along the way.
                let new_keys = if let Square::Key(key) = edge.end_square {
                    node.keys.pick_up(key)
                } else {
                    node.keys.clone()
                };

                // Don't go through doors that we don't have the keys to.
                if let Square::Door(door) = edge.end_square {
                    if !new_keys.has(door.to_ascii_lowercase()) {
                        continue;
                    }
                }

                // Skip positions that we've already visited with this set of keys.
                if !visited.insert(Visited {
                    position: edge.end,
                    keys: new_keys.clone(),
                }) {
                    continue;
                }

                // Visit the edge.
                to_visit.push(ToVisit {
                    position: edge.end,
                    length: node.length + edge.length,
                    keys: new_keys.clone(),
                });
            }
        }

        panic!("No path to collect all keys.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_keys_steps_sample() {
        let map = Map::load("sample.txt");
        assert_eq!(8, map.all_keys_steps());
    }

    #[test]
    fn all_keys_steps_sample_part2() {
        let map = Map::load("sample_part2.txt");
        assert_eq!(8, map.all_keys_steps());
    }

    #[test]
    fn all_keys_steps_sample2() {
        let map = Map::load("sample2.txt");
        assert_eq!(86, map.all_keys_steps());
    }

    #[test]
    fn all_keys_steps_sample3() {
        let map = Map::load("sample3.txt");
        assert_eq!(132, map.all_keys_steps());
    }

    #[test]
    fn all_keys_steps_sample4() {
        let map = Map::load("sample4.txt");
        assert_eq!(136, map.all_keys_steps());
    }

    #[test]
    fn all_keys_steps_sample5() {
        let map = Map::load("sample5.txt");
        assert_eq!(81, map.all_keys_steps());
    }
}