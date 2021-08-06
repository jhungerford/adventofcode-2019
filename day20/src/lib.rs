use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up, Down, Left, Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
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

#[derive(Debug, Copy, Clone)]
struct BoundedPosition {
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
}

impl BoundedPosition {
    fn new<T>(row: usize, col: usize, maze: &Vec<Vec<T>>) -> Self {
        BoundedPosition {
            row, col,
            rows: maze.len(),
            cols: maze[row].len(),
        }
    }

    fn at(row: usize, col: usize, rows: usize, cols: usize) -> Self {
        BoundedPosition { row, col, rows, cols }
    }

    /// Adds or subtracts the given number of rows from this position, returning a new
    /// BoundedPosition if it's in bounds.
    fn plus_rows(&self, rows: isize) -> Option<BoundedPosition> {
        let new_row = self.row as isize + rows;
        if new_row > 0 && new_row < self.rows as isize {
            Some(BoundedPosition::at(new_row as usize, self.col, self.rows, self.cols))
        } else {
            None
        }
    }

    /// Adds or subtracts the given number of columns from this position, returning a new
    /// BoundedPosition if it's in bounds.
    fn plus_cols(&self, cols: isize) -> Option<BoundedPosition> {
        let new_col = self.col as isize + cols;
        if new_col > 0 && new_col < self.cols as isize {
            Some(BoundedPosition::at(self.row, new_col as usize, self.rows, self.cols))
        } else {
            None
        }
    }
}

impl Add<Direction> for BoundedPosition {
    type Output = Option<BoundedPosition>;

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => self.plus_rows(-1),
            Direction::Down => self.plus_rows(1),
            Direction::Left => self.plus_cols(-1),
            Direction::Right => self.plus_cols(1),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Square {
    Empty,
    Wall,
    Open,
    Portal,
}

#[derive(Debug, Clone)]
pub struct Portal {
    name: String,
    pos: Position,
    dir: Direction,
}

#[derive(Debug)]
pub struct Maze {
    squares: Vec<Vec<Square>>,
    name_portals: HashMap<String, Vec<Portal>>,
    pos_portal: HashMap<Position, Portal>,
}

impl Maze {
    /// Loads a maze from the given file.
    pub fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let lines = f.lines()
            .map(|line| line.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let longest_line = lines.iter().map(|line| line.len()).max().unwrap();

        /// Returns whether the square at the given position is a portal letter.
        fn is_letter(maybe_pos: Option<BoundedPosition>, lines: &Vec<Vec<char>>) -> bool {
            if let Some(pos) = maybe_pos {
                let value = lines[pos.row][pos.col];
                return value >= 'A' && value <= 'Z';
            }

            false
        }

        /// Parses a portal at the given open position, returning None if the position isn't a portal.
        fn parse_portal(pos: Position, lines: &Vec<Vec<char>>) -> Option<Portal> {

            // Portals are only adjacent to one open square, which determines their direction.
            let bounded_pos = BoundedPosition::new(pos.row, pos.col, lines);
            if is_letter(bounded_pos.plus_rows(-1), lines) {
                Some(Portal {
                    name: format!("{}{}", lines[pos.row - 2][pos.col], lines[pos.row - 1][pos.col]),
                    pos,
                    dir: Direction::Down,
                })
            } else if is_letter(bounded_pos.plus_rows(1), lines) {
                Some(Portal {
                    name: format!("{}{}", lines[pos.row + 1][pos.col], lines[pos.row + 2][pos.col]),
                    pos,
                    dir: Direction::Up,
                })
            } else if is_letter(bounded_pos.plus_cols(-1), lines) {
                Some(Portal {
                    name: format!("{}{}", lines[pos.row][pos.col - 2], lines[pos.row][pos.col - 1]),
                    pos,
                    dir: Direction::Right,
                })
            } else if is_letter(bounded_pos.plus_cols(1), lines) {
                Some(Portal {
                    name: format!("{}{}", lines[pos.row][pos.col + 1], lines[pos.row][pos.col + 2]),
                    pos,
                    dir: Direction::Left,
                })
            } else {
                None
            }
        }

        // Parse the lines into a maze.
        let mut squares = Vec::new();
        let mut name_portals = HashMap::new();
        let mut pos_portal = HashMap::new();
        for (row_num, row) in lines.iter().enumerate() {
            let mut row_squares = Vec::new();
            for (col_num, c) in row.iter().enumerate() {
                let square = match c {
                    ' ' => Square::Empty,
                    '#' => Square::Wall,
                    '.' => {
                        if let Some(portal) = parse_portal(Position::new(row_num, col_num), &lines) {
                            name_portals.entry(portal.name.clone()).or_insert(Vec::new()).push(portal.clone());
                            pos_portal.insert(portal.pos, portal.clone());

                            Square::Portal
                        } else {
                            Square::Open
                        }
                    },
                    _ => Square::Empty,
                };

                row_squares.push(square);
            }

            for _ in row.len()..longest_line {
                row_squares.push(Square::Empty);
            }

            squares.push(row_squares);
        }

        Maze { squares, name_portals, pos_portal }
    }

    /// Returns the shortest path from the entrance of this maze to the exit.
    pub fn shortest_path(&self) -> usize {
        struct ToVisit {
            position: Position,
            direction: Direction,
            steps: usize,
        }

        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();

        let start_portal = self.name_portals["AA"].iter().next().unwrap();
        to_visit.push_back(ToVisit {
            position: start_portal.pos,
            direction: start_portal.dir,
            steps: 0,
        });

        while let Some(node) = to_visit.pop_front() {
            visited.insert(node.position);

            // Take a step in the node's direction.
            let new_pos = node.position + node.direction;
            let new_square = self.square_at(&new_pos);

            // If the step goes through a portal, end up at its exit.
            if new_square == Square::Portal {
                let portal = self.portal_at(&new_pos);

                // If we've reached the exit portal, we've found the path.
                if portal.name == "ZZ" {
                    return node.steps + 1;
                }

                // Otherwise go through the portal.
                let exit = self.portal_exit(&portal);
                to_visit.push_back(ToVisit {
                    position: exit.pos,
                    direction: exit.dir,
                    steps: node.steps + 2,
                });

            } else {
                // Step puts us in an open square - explore its new neighbors.
                for dir in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                    let neighbor_pos = new_pos + dir;
                    let neighbor_square = self.square_at(&neighbor_pos);
                    if (neighbor_square == Square::Open || neighbor_square == Square::Portal) && !visited.contains(&neighbor_pos) {
                        to_visit.push_back(ToVisit {
                            position: new_pos,
                            direction: dir,
                            steps: node.steps + 1,
                        });
                    }
                }
            }
        }

        panic!("No path to the exit.")
    }

    /// Returns the shortest path from the entrance of this maze to the exit on the outer level,
    /// where inner portals lead to an inner level of the maze.
    pub fn shortest_path_recursive(&self) -> usize {
        #[derive(Debug, Hash, Eq, PartialEq)]
        struct Visited {
            position: Position,
            level: usize,
        }

        struct ToVisit {
            position: Position,
            direction: Direction,
            steps: usize,
            level: usize,
        }

        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();

        let start_portal = self.name_portals["AA"].iter().next().unwrap();
        to_visit.push_back(ToVisit {
            position: start_portal.pos,
            direction: start_portal.dir,
            steps: 0,
            level: 0,
        });

        while let Some(node) = to_visit.pop_front() {
            visited.insert(Visited {
                position: node.position,
                level: node.level,
            });

            // Take a step in the node's direction.
            let new_pos = node.position + node.direction;
            let new_square = self.square_at(&new_pos);

            // If the step goes through a portal, end up at its exit.
            if new_square == Square::Portal {
                let portal = self.portal_at(&new_pos);

                // If we've reached the exit portal on the top level, we've found the path.
                if portal.name == "ZZ" && node.level == 0 {
                    return node.steps + 1;
                }

                // If we're at the top level and this is an exterior portal, it's a wall.
                if node.level == 0 && self.is_exterior_portal(portal) {
                    continue;
                }

                // If we're not at the top level, AA and ZZ are walls.
                if node.level > 0 && (portal.name == "AA" || portal.name == "ZZ") {
                    continue;
                }

                // Otherwise go through the portal.
                let exit = self.portal_exit(&portal);
                let exit_level = if self.is_exterior_portal(portal) {
                    node.level - 1
                } else {
                    node.level + 1
                };

                to_visit.push_back(ToVisit {
                    position: exit.pos,
                    direction: exit.dir,
                    steps: node.steps + 2,
                    level: exit_level,
                });
            } else {
                // Step puts us in an open square - explore its new neighbors.
                for dir in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                    let neighbor_pos = new_pos + dir;
                    let neighbor_square = self.square_at(&neighbor_pos);
                    let neighbor_visited = Visited {
                        position: neighbor_pos,
                        level: node.level,
                    };
                    if (neighbor_square == Square::Open || neighbor_square == Square::Portal) && !visited.contains(&neighbor_visited) {
                        to_visit.push_back(ToVisit {
                            position: new_pos,
                            direction: dir,
                            steps: node.steps + 1,
                            level: node.level,
                        });
                    }
                }
            }
        }

        panic!("No path through the maze")
    }

    /// Returns the square at the given position.
    fn square_at(&self, pos: &Position) -> Square {
        self.squares[pos.row][pos.col]
    }

    /// Returns the portal at the given position.
    fn portal_at(&self, pos: &Position) -> &Portal {
        &self.pos_portal[pos]
    }

    /// Returns the exit for the given portal.
    fn portal_exit(&self, portal: &Portal) -> &Portal {
        self.name_portals.get(&portal.name).unwrap().iter()
            .find(|p| p.pos != portal.pos)
            .unwrap()
    }

    fn is_exterior_portal(&self, portal: &Portal) -> bool {
        portal.pos.row == 2
            || portal.pos.row == self.squares.len() - 3
            || portal.pos.col == 2
            || portal.pos.col == self.squares[portal.pos.row].len() - 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_path_sample() {
        let maze = Maze::load("sample.txt");
        assert_eq!(23, maze.shortest_path());
    }

    #[test]
    fn shortest_path_sample2() {
        let maze = Maze::load("sample2.txt");
        assert_eq!(58, maze.shortest_path());
    }

    #[test]
    fn shortest_path_recursive_sample() {
        let maze = Maze::load("sample.txt");
        assert_eq!(26, maze.shortest_path_recursive());
    }

    #[test]
    fn shortest_path_recursive_sample3() {
        let maze = Maze::load("sample3.txt");
        assert_eq!(396, maze.shortest_path_recursive());
    }
}