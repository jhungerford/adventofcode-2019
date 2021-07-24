use crate::computer::Computer;
use std::fmt::{Display, Formatter};
use std::time::Instant;

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
fn print_beam(computer: &mut Computer) {
    'row: for row in 0..1200 {

        print!("{row:>width$} ", row=row, width=6);

        let mut found_beam = false;
        'col: for col in 0..=row {
            let square = scan(computer, row, col);
            print!("{}", square);

            if square == Point::Pulled {
                found_beam = true;
            }

            if found_beam && square == Point::Stationary {
                break 'col;
            }
        }

        println!()
    }
}

fn beam_start(computer: &mut Computer, rows: i64) {

    for row in 0..rows {
        let mut beam_start = -1;
        let mut beam_end = -2;

        for col in 0..=row {
            let point = scan(computer, row, col);
            if point == Point::Pulled {
                beam_end = col;
                if beam_start == -1 {
                    beam_start = col;
                }
            }
        }

        println!("Row {} - {} to {} - length {}", row, beam_start, beam_end, beam_end - beam_start + 1);
    }
}

/// Finds the first square with the given side length in the tractor beam,
/// and returns 10000 * row + col.
fn find_square(side: i32) -> i32 {
    // Find the lowest row and column that fits in the tractor beam.

    for row_corner in 0.. {
        // Working out the tractor beam bounds by hand:
        // Left:  col = ⌈ 3/4 row ⌉
        // Right: col = ⌈ 12/13 (row - 1) ⌉

        // We're looking for the min (row_corner, column_corner) such that:
        // row = row_corner + side - 1 is on the left
        // col = col_corner + side - 1 is on the right.

        // Lower left corner is on the left side and tells us the column.
        let col_corner = (3.0/4.0 * (row_corner + side - 1) as f64).ceil() as i32;

        // Top right corner is on the right side.
        let right_col = (12.0/13.0 * (row_corner - 1) as f64).ceil() as i32;

        // And if the top and left sides of the square match, we've got the answer.
        if right_col - side + 1 == col_corner {
            return 10000 * row_corner + col_corner;
        }
    }

    panic!("No square found.")
}

fn main() {
    let mut computer = Computer::load("input.txt");

    println!("Part 1: {}", num_pulled(&mut computer));

    // Part 2 - find the top left corner of the 100x100 square in the tractor beam.

    // Ran `cargo run >> output.txt` with this block to produce a bigger sample square.
    // let start = Instant::now();
    // print_beam(&mut computer);
    // println!("Took {} ms", start.elapsed().as_millis());

    println!("Part 2: {}", find_square(100));
}
