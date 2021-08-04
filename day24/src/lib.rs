// Bugs (#), empty spaces (.)
// Bug dies unless there's exactly one bug adjacent
// Empty space becomes invested if exactly one or two bugs are adjacent
// Otherwise bug or empty space remains the same
// Tiles outside the grid are empty.

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Grid stores bugs as 1's, and empty spaces as 0's.  The grid is a 5x5 square,
/// and the left-to-right bits in the value run from left-to-right, top-to-bottom in the grid.
pub struct Grid {
    value: u32
}

const GRID_MASKS: [u32; 25] = [
    0b01000_10000_00000_00000_00000,
    0b10100_01000_00000_00000_00000,
    0b01010_00100_00000_00000_00000,
    0b00101_00010_00000_00000_00000,
    0b00010_00001_00000_00000_00000,

    0b10000_01000_10000_00000_00000,
    0b01000_10100_01000_00000_00000,
    0b00100_01010_00100_00000_00000,
    0b00010_00101_00010_00000_00000,
    0b00001_00010_00001_00000_00000,

    0b00000_10000_01000_10000_00000,
    0b00000_01000_10100_01000_00000,
    0b00000_00100_01010_00100_00000,
    0b00000_00010_00101_00010_00000,
    0b00000_00001_00010_00001_00000,

    0b00000_00000_10000_01000_10000,
    0b00000_00000_01000_10100_01000,
    0b00000_00000_00100_01010_00100,
    0b00000_00000_00010_00101_00010,
    0b00000_00000_00001_00010_00001,

    0b00000_00000_00000_10000_01000,
    0b00000_00000_00000_01000_10100,
    0b00000_00000_00000_00100_01010,
    0b00000_00000_00000_00010_00101,
    0b00000_00000_00000_00001_00010,
];

impl Grid {
    /// Loads a new grid from the given file.  Bugs are '#', and empty spaces are '.'.
    pub fn load(filename: &str) -> Grid {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut value = 0;

        for line in f.lines() {
            for c in line.unwrap().chars() {
                value <<= 1;
                if c == '#' {
                    value |= 1;
                }
            }
        }

        Grid { value }
    }

    /// Advances this grid by one tick.
    pub fn tick(&mut self) {
        let mut new_value = 0;

        for (i, &mask) in GRID_MASKS.iter().enumerate() {
            new_value <<= 1;

            let square_has_bug = self.value & (1 << (24 - i)) > 0;
            let num_adjacent = (self.value & mask).count_ones();

            if square_has_bug && num_adjacent == 1 {
                // A bug dies (becomes an empty space) unless there is exactly one bug adjacent.
                new_value |= 1;
            } else if !square_has_bug && (num_adjacent == 1 || num_adjacent == 2) {
                // An empty space becomes infested if exactly one or two bugs are adjacent.
                new_value |= 1;
            }
        }

        self.value = new_value;
    }

    /// Returns the biodiversity rating if this grid.  The biodiversity rating of the grid
    /// counts the powers of two for each tile - the top-left tile is worth 1, the next tile
    /// to the right on the top row is worth 2, and so on.
    pub fn biodiversity(&self) -> u32 {
        self.value.reverse_bits() >> 7
    }
}

/// Returns the biodiversity rating for the first grid that appears twice.
pub fn first_repeat(grid: &mut Grid) -> u32 {
    let mut seen = HashSet::new();

    while !seen.contains(&grid.biodiversity()) {
        seen.insert(grid.biodiversity());
        grid.tick();
    }

    grid.biodiversity()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sample() {
        let grid = Grid::load("sample.txt");
        assert_eq!(0b00001_10010_10011_00100_10000, grid.value);
    }

    #[test]
    fn tick_sample() {
        let mut grid = Grid::load("sample.txt");

        grid.tick();
        assert_eq!(0b10010_11110_11101_11011_01100, grid.value);

        grid.tick();
        assert_eq!(0b11111_00001_00001_00010_10111, grid.value);

        grid.tick();
        assert_eq!(0b10000_11110_00011_10110_01101, grid.value);

        grid.tick();
        assert_eq!(0b11110_00001_11001_00000_11000, grid.value);
    }

    #[test]
    fn first_repeat_sample() {
        let mut grid = Grid::load("sample.txt");
        let repeat = first_repeat(&mut grid);
        println!("{:b}", repeat);

        assert_eq!(2129920, repeat);
    }
}