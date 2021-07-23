use std::fmt::{Display, Formatter};

use crate::computer::Computer;

pub mod computer;

#[derive(Debug, Eq, PartialEq)]
enum Square {
    UpRobot,
    DownRobot,
    LeftRobot,
    RightRobot,
    Space,
    Scaffold,
}

impl From<i64> for Square {
    fn from(value: i64) -> Self {
        match value as u8 as char {
            '^' => Square::UpRobot,
            'v' => Square::DownRobot,
            '<' => Square::LeftRobot,
            '>' => Square::RightRobot,
            '.' => Square::Space,
            '#' => Square::Scaffold,
            _ => panic!("Invalid square: '{}' - might be a control character like newline.", value),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Square::UpRobot => "^",
            Square::DownRobot => "v",
            Square::LeftRobot => "<",
            Square::RightRobot => ">",
            Square::Space => ".",
            Square::Scaffold => "#",
        };

        write!(f, "{}", value)
    }
}

struct Position {
    row: usize,
    col: usize,
}

impl Position {
    /// Creates a new position at the given row and column.
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

/// Boots the program, returning the sum of the alignment parameters for the scaffold intersections.
pub fn calibration(computer: &mut Computer) -> usize {
    // Running the program for the first time prints the map.
    computer.run();

    // Load the output into a map.
    let mut map: Vec<Vec<Square>> = Vec::new();
    let mut line = Vec::new();

    for &value in &computer.output {
        if value as u8 as char == '\n' {
            if !line.is_empty() {
                map.push(line);
            }
            line = Vec::new();
        } else {
            line.push(Square::from(value));
        }
    }

    // Scaffolds with four scaffold neighbors are intersections - count them up.
    // Intersections don't show up on the map edges.
    let mut intersections = Vec::new();
    for row in 1..(map.len() - 1) {
        for col in 1..(map[row].len() - 1) {
            let pos = Position::new(row, col);

            if is_intersection(&map, &pos) {
                intersections.push(Position::new(row, col));
            }
        }
    }

    intersections.iter().map(|pos| pos.row * pos.col).sum()
}

/// Returns whether the square at the given row and column is an intersection.
/// Intersections never show up on the edge of the map, so row and col shouldn't be edges.
fn is_intersection(map: &Vec<Vec<Square>>, pos: &Position) -> bool {
    if map[pos.row][pos.col] != Square::Scaffold {
        return false;
    }

    for (row_offset, col_offset) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighbor_row = (pos.row as isize + row_offset) as usize;
        let neighbor_col = (pos.col as isize + col_offset) as usize;

        if map[neighbor_row][neighbor_col] != Square::Scaffold {
            return false;
        }
    }

    true
}