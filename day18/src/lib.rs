// @: entrance, .: passage, #: wall, lowercase letters: keys, uppercase letters: doors.
// keys open doors with matching letters.
// Part 1: starting at the entrance, how many steps are in the shortest path that collects all keys?

use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufRead};

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

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    squares: Vec<Vec<Square>>,
    entrance: Position,
    num_keys: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

impl Map {
    /// Loads a map from the given file.
    pub fn load(filename: &str) -> Map {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut squares = Vec::new();
        let mut entrance= Position::new(0, 0);
        let mut num_keys = 0;

        for (row, line) in f.lines().enumerate() {
            let mut row_squares = Vec::new();

            for (col, c) in line.unwrap().chars().enumerate() {
                let square: Square = Square::from(c);
                row_squares.push(square);

                match square {
                    Square::Entrance => entrance = Position::new(row, col),
                    Square::Key(_) => num_keys += 1,
                    _ => {},
                }
            }

            squares.push(row_squares);
        }

        Map { squares, entrance, num_keys }
    }

    /// Returns the shortest number of steps that collects all of the keys.
    pub fn all_keys_steps(&self) -> usize {
        let mut visited: HashSet<State> = HashSet::new();
        let mut to_visit = VecDeque::new();

        to_visit.push_back(State {
            position: self.entrance.clone(),
            steps: 0,
            keys: Vec::new(),
            doors: Vec::new(),
        });

        while let Some(pos) = to_visit.pop_front() {
            if pos.keys.len() == self.num_keys {
                return pos.steps;
            }

            for neighbor in self.visitable_neighbors(&pos) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor.clone());
                    to_visit.push_back(neighbor);
                }
            }
        }

        panic!("No path to collect all keys.")
    }

    /// Returns the square at the given position.
    fn get(&self, pos: &Position) -> Square {
        self.squares[pos.row][pos.col]
    }

    /// Returns a list of visitable squares that neighbor the given state.
    fn visitable_neighbors(&self, state: &State) -> Vec<State> {
        let neighboring_positions = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
            .filter_map(|(row_delta, col_delta)| {
                let int_row = state.position.row as i32 + row_delta;
                let int_col = state.position.col as i32 + col_delta;

                if int_row < 0 || int_col < 0 || int_row as usize >= self.squares.len() || int_col as usize >= self.squares[state.position.row].len() {
                    None
                } else {
                    Some(Position {
                        row: int_row as usize,
                        col: int_col as usize,
                    })
                }
            });

        neighboring_positions.filter_map(|neighbor| {
            let mut new_keys = state.keys.clone();
            let mut new_doors = state.doors.clone();

            match self.get(&neighbor) {
                Square::Wall => return None,
                Square::Key(k) if !state.keys.contains(&k) => {
                    new_keys.push(k);
                    new_keys.sort();
                },
                Square::Door(d) => {
                    let key = d.to_lowercase().next().unwrap();
                    if state.keys.contains(&key) {
                        new_doors.push(d);
                        new_doors.sort();
                    } else {
                        return None;
                    }
                }
                _ => {},
            }

            Some(State {
                position: neighbor,
                steps: state.steps + 1,
                keys: new_keys,
                doors: new_doors,
            })
        }).collect()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    position: Position,
    steps: usize,
    keys: Vec<char>,
    doors: Vec<char>,
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
}