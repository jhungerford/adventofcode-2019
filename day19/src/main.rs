use crate::computer::Computer;
use std::fmt::{Display, Formatter};

mod computer;

#[derive(Debug, Eq, PartialEq)]
enum Point {
    Stationary,
    Pulled,
}

impl From<i64> for Point {
    fn from(value: i64) -> Self {
        match value {
            0 => Point::Stationary,
            1 => Point::Pulled,
            _ => panic!("Invalid square {}", value)
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Point::Stationary => ".",
            Point::Pulled => "#",
        };

        write!(f, "{}", c)
    }
}

/// Scans the square at the given row and column, returning what the ship sees.
/// Resets the computer to its original configuration.
fn scan(computer: &mut Computer, row: i64, col: i64) -> Point {
    computer.input(row);
    computer.input(col);

    let output = computer.run().unwrap();
    computer.reset();

    Point::from(output)
}

/// Part 1: Returns the number of points that the tractor beam pulls in a 50x50 square.
fn num_pulled(computer: &mut Computer) -> i32 {
    let mut num_pulled = 0;
    for row in 0..50 {
        for col in 0..50 {
            if scan(computer, row, col) == Point::Pulled {
                num_pulled += 1;
            }
        }
    }

    num_pulled
}

/// Prints the beam in a rows x cols rectange.
fn print_beam(computer: &mut Computer, rows: i64, cols: i64) {
    for row in 0..rows {
        for col in 0..cols {
            scan(computer, row, col);
        }
    }
}

fn main() {
    let mut computer = Computer::load("input.txt");

    println!("Part 1: {}", num_pulled(&mut computer));
}
