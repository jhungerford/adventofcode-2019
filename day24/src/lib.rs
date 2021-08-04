// Bugs (#), empty spaces (.)
// Bug dies unless there's exactly one bug adjacent
// Empty space becomes invested if exactly one or two bugs are adjacent
// Otherwise bug or empty space remains the same
// Tiles outside the grid are empty.

use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::{Debug, Formatter};

/// Grid stores bugs as 1's, and empty spaces as 0's.  The grid is a 5x5 square,
/// and the left-to-right bits in the value run from left-to-right, top-to-bottom in the grid.
pub struct Grid {
    value: u32
}

/// GRID_MASKS contains bitmasks that determine the neighbors for each square in the current level
/// of the grid.  Masks run from left-to-right, top-to-bottom.
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

/// INNER_GRID_MASKS contains bitmasks that determine the neighbors of one level higher than
/// the current recursive grid.
const INNER_GRID_MASKS: [u32; 25] = [
    0b00000_00100_01000_00000_00000,
    0b00000_00100_00000_00000_00000,
    0b00000_00100_00000_00000_00000,
    0b00000_00100_00000_00000_00000,
    0b00000_00100_00010_00000_00000,

    0b00000_00000_01000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00010_00000_00000,

    0b00000_00000_01000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00010_00000_00000,

    0b00000_00000_01000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00010_00000_00000,

    0b00000_00000_01000_00100_00000,
    0b00000_00000_00000_00100_00000,
    0b00000_00000_00000_00100_00000,
    0b00000_00000_00000_00100_00000,
    0b00000_00000_00010_00100_00000,
];

/// OUTER_GRID_MASKS contains bitmasks that determine the neighbors of one level lower than
/// the current recursive grid.
const OUTER_GRID_MASKS: [u32; 25] = [
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,

    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b11111_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,

    0b00000_00000_00000_00000_00000,
    0b10000_10000_10000_10000_10000,
    0b00000_00000_00000_00000_00000,
    0b00001_00001_00001_00001_00001,
    0b00000_00000_00000_00000_00000,

    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_11111,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,

    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
    0b00000_00000_00000_00000_00000,
];

const EMPTY_GRID: Grid = Grid { value: 0 };

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        let mut mask = 1 << 24;
        for _ in 0..5 {
            for _ in 0..5 {
                write!(f, "{}", if (self.value & mask) == 0 { "." } else { "#" })?;
                mask >>= 1;
            }

            writeln!(f)?
        }

        Ok(())
    }
}

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
    fn biodiversity(&self) -> u32 {
        self.value.reverse_bits() >> 7
    }

    /// Returns the biodiversity rating for the first grid that appears twice.
    pub fn first_repeat(mut self) -> u32 {
        let mut seen = HashSet::new();

        while !seen.contains(&self.biodiversity()) {
            seen.insert(self.biodiversity());
            self.tick();
        }

        self.biodiversity()
    }
}

/// RecursiveGrid is a multi-level grid where the middle square of each grid contains a grid.
#[derive(Debug)]
pub struct RecursiveGrid {
    levels: HashMap<isize, Grid>,
    min_level: isize,
    max_level: isize,
}

impl RecursiveGrid {
    /// Loads a RecursiveGrid from the given file.
    pub fn load(filename: &str) -> Self {
        let grid = Grid::load(filename);
        let levels = vec![(0, grid)].into_iter().collect::<HashMap<isize, Grid>>();

        RecursiveGrid {
            levels,
            min_level: 0,
            max_level: 0,
        }
    }

    /// Advances this recursive grid by one tick, modifying it in place.
    pub fn tick(&mut self) {
        let mut new_levels = HashMap::new();
        let mut new_min_level = self.min_level;
        let mut new_max_level = self.max_level;

        // Check one level deeper and higher than our current depth.
        for depth in (self.min_level - 1) ..= (self.max_level + 1) {
            let depth_grid = self.tick_level(depth);
            if depth_grid.value != 0 || self.levels.contains_key(&depth) {
                new_levels.insert(depth, depth_grid);

                if depth < new_min_level {
                    new_min_level = depth;
                }

                if depth > self.max_level {
                    new_max_level = depth;
                }
            }
        }

        self.levels = new_levels;
        self.min_level = new_min_level;
        self.max_level = new_max_level;
    }

    /// Computes one tick for the given level, and returns a new grid containing the value.
    fn tick_level(&self, level: isize) -> Grid {
        let level_grid = self.levels.get(&level).unwrap_or(&EMPTY_GRID);
        let inner_grid = self.levels.get(&(level - 1)).unwrap_or(&EMPTY_GRID);
        let outer_grid = self.levels.get(&(level + 1)).unwrap_or(&EMPTY_GRID);

        let mut new_value = 0;

        for i in 0..GRID_MASKS.len() {
            new_value <<= 1;

            // The middle square is a lower-level grid, so it's always 0.
            if i == 12 {
                continue;
            }

            let square_has_bug = level_grid.value & (1 << (24 - i)) > 0;
            let num_adjacent = (level_grid.value & GRID_MASKS[i]).count_ones()
                + (inner_grid.value & INNER_GRID_MASKS[i]).count_ones()
                + (outer_grid.value & OUTER_GRID_MASKS[i]).count_ones();

            if square_has_bug && num_adjacent == 1 {
                // A bug dies (becomes an empty space) unless there is exactly one bug adjacent.
                new_value |= 1;
            } else if !square_has_bug && (num_adjacent == 1 || num_adjacent == 2) {
                // An empty space becomes infested if exactly one or two bugs are adjacent.
                new_value |= 1;
            }
        }

        Grid { value: new_value }
    }

    /// Returns the number of bugs that are present in this recursive grid.
    pub fn num_bugs(&self) -> u32 {
        self.levels.values().map(|grid| grid.value.count_ones()).sum()
    }
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
        let grid = Grid::load("sample.txt");
        assert_eq!(2129920, grid.first_repeat());
    }

    #[test]
    fn recursive_bugs_sample() {
        let mut grid = RecursiveGrid::load("sample.txt");

        for _ in 0..10 {
            grid.tick();
        }

        assert_eq!(99, grid.num_bugs());
    }
}