// @: entrance, .: passage, #: wall, lowercase letters: keys, uppercase letters: doors.
// keys open doors with matching letters.
// Part 1: starting at the entrance, how many steps are in the shortest path that collects all keys?

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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

#[derive(Clone, Eq, PartialEq, Hash)]
struct Keys {
    picked_up: BitSet,
}

impl Debug for Keys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.picked_up.iter().map(|num| (num + 'a' as usize) as u8 as char).join(", "))
    }
}

impl Keys {
    /// Creates a new Keys where none of the keys have been picked up.
    fn new() -> Self {
        Keys {
            picked_up: BitSet::new()
        }
    }

    /// Returns whether this has all of the keys for the given doors.
    fn has_keys(&self, doors: &Keys) -> bool {
        self.picked_up.is_superset(&doors.picked_up)
    }

    /// Returns a new Keys that contains all of these keys, plus the given key.
    fn pick_up(&self, square: Square) -> Self {
        let mut new_keys = self.clone();

        if let Some(key) = match square {
            Square::Door(d) => {
                Some(d.to_ascii_lowercase())
            }
            Square::Key(k) => {
                Some(k)
            }
            _ => None,
        } {
            new_keys.picked_up.insert(key as usize - 'a' as usize);
        }

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

#[derive(Debug)]
struct KeyEdge {
    end: Position,
    doors: Keys,
    length: usize,
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
    pub fn get(&self, position: &Position) -> Square {
        self.squares[position.row][position.col]
    }

    /// Returns the shortest number of steps that collects all of the keys.
    pub fn all_keys_steps(&self) -> usize {
        // Only the entrances and keys matter - compute the edges between them.
        let key_graph = self.key_graph();

        // State captures the position of robots and collected keys as we explore the graph.
        #[derive(Debug, Hash, Eq, PartialEq, Clone)]
        struct State {
            robots: Vec<Position>,
            keys: Keys,
        }

        #[derive(Debug, Eq, PartialEq, Clone)]
        struct ToVisit {
            state: State,
            length: usize
        }

        impl Ord for ToVisit {
            fn cmp(&self, other: &Self) -> Ordering {
                other.length.cmp(&self.length)
            }
        }

        impl PartialOrd for ToVisit {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // Use Dijkstra's algorithm to compute the shortest path that collects all of the keys.
        let mut to_visit = BinaryHeap::new();
        let mut dist = HashMap::new();

        let initial_state = State {
            robots: self.entrances.clone(),
            keys: Keys::new(),
        };

        to_visit.push(ToVisit {
            state: initial_state.clone(),
            length: 0,
        });

        dist.insert(initial_state, 0);

        while let Some(node) = to_visit.pop() {
            let node_length = dist[&node.state];

            // Bail if we've already found a shorter route to this state.
            if node.length > node_length {
                continue;
            }

            // If we've picked up all the keys, we've found the shortest path through the map.
            if node.state.keys.all_picked_up(self.num_keys) {
                return node_length;
            }


            // Explore each of the robots neighbors that we can reach.
            for (i, robot) in node.state.robots.iter().enumerate() {
                for neighbor_edge in &key_graph[robot] {
                    // Only go through doors that we have keys for.
                    if !node.state.keys.has_keys(&neighbor_edge.doors) {
                        continue;
                    }

                    let mut neighbor_robots = node.state.robots.clone();
                    neighbor_robots[i] = neighbor_edge.end.clone();

                    let neighbor_state = State {
                        robots: neighbor_robots,
                        keys: node.state.keys.pick_up(self.get(&neighbor_edge.end))
                    };

                    let neighbor_length = node_length + neighbor_edge.length;
                    if neighbor_length < *dist.get(&neighbor_state).unwrap_or(&usize::MAX) {
                        dist.insert(neighbor_state.clone(), neighbor_length);

                        to_visit.push(ToVisit {
                            state: neighbor_state,
                            length: neighbor_length,
                        })
                    }
                }
            }
        }

        panic!("No path to collect all keys.")
    }

    /// Returns a graph view of this map.  The map contains edges from each of the entrances
    /// and keys to reachable edges and keys.
    fn key_graph(&self) -> HashMap<Position, Vec<KeyEdge>> {
        let mut graph: HashMap<Position, Vec<KeyEdge>> = HashMap::new();

        let mut to_visit = VecDeque::new();
        let mut keys = Vec::new();

        struct ToVisit {
            start: Position,
            position: Position,
            doors: Keys,
            length: usize,
            visited: HashSet<Position>,
        }

        // Start at each of the keys / entrances.
        for row in 0..self.squares.len() {
            for col in 0..self.squares[row].len() {
                let pos = Position::new(row, col);
                let square = self.get(&pos);
                match square {
                    Square::Entrance | Square::Key(_) => {
                        if square != Square::Entrance {
                            keys.push(pos);
                        }

                        to_visit.push_back(ToVisit {
                            start: pos,
                            position: pos,
                            doors: Keys::new(),
                            length: 0,
                            visited: vec![pos].into_iter().collect(),
                        })
                    },
                    _ => {}
                };
            }
        }

        // Explore until we've found all of the paths between keys.
        while let Some(node) = to_visit.pop_front() {
            for direction in Direction::values() {
                let neighbor = node.position + direction;
                if node.visited.contains(&neighbor) {
                    continue;
                }

                let neighbor_square = self.get(&neighbor);
                let mut neighbor_visited = node.visited.clone();
                neighbor_visited.insert(neighbor);

                match neighbor_square {
                    Square::Key(_) => {
                        graph.entry(node.start).or_default().push(KeyEdge {
                            end: neighbor,
                            doors: node.doors.clone(),
                            length: node.length + 1,
                        });
                    },

                    Square::Door(_) => {
                        to_visit.push_back(ToVisit {
                            start: node.start,
                            position: neighbor,
                            doors: node.doors.pick_up(neighbor_square),
                            length: node.length + 1,
                            visited: neighbor_visited,
                        });
                    },

                    Square::Open | Square::Entrance => {
                        to_visit.push_back(ToVisit {
                            start: node.start,
                            position: neighbor,
                            doors: node.doors.clone(),
                            length: node.length + 1,
                            visited: neighbor_visited,
                        });
                    },
                    Square::Wall => {},
                }
            }
        }

        // Make sure there's an entry for any keys that only connect to the entrance.
        for key in keys {
            graph.entry(key).or_default();
        }

        graph
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

    #[test]
    fn part2_all_keys_steps_sample() {
        let map = Map::load("part2_sample.txt");
        assert_eq!(8, map.all_keys_steps());
    }

    #[test]
    fn part2_all_keys_steps_sample2() {
        let map = Map::load("part2_sample2.txt");
        assert_eq!(24, map.all_keys_steps());
    }

    #[test]
    fn part2_all_keys_steps_sample3() {
        let map = Map::load("part2_sample3.txt");
        assert_eq!(32, map.all_keys_steps());
    }

    #[test]
    fn part2_all_keys_steps_sample4() {
        let map = Map::load("part2_sample4.txt");
        assert_eq!(72, map.all_keys_steps());
    }
}