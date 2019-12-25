use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::path::Path;

use position::*;

mod position;

#[derive(Eq, PartialEq, Debug)]
enum Contents { Empty, Asteroid }

impl Contents {
    fn from_char(c: char) -> Contents {
        use Contents::*;

        match c {
            '.' => Empty,
            '#' => Asteroid,
            n => panic!("{} is not a valid contents.", n),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Map {
    width: usize,
    height: usize,
    contents: Vec<Vec<Contents>>,
    asteroids: Vec<Position>,
}

impl Map {
    /// Parses the given lines into a map.
    fn parse(lines: &Vec<&str>) -> Map {
        assert!(!lines.is_empty(), "Map cannot be empty.");

        let width = lines[0].len();
        let height = lines.len();

        let contents: Vec<Vec<Contents>> = lines.into_iter()
            .map(|line| line.chars().map(Contents::from_char).collect())
            .collect();

        let asteroids = (0..width).flat_map(|x| (0..height)
            .map(move |y| Position::new(x, y)))
            .filter(|position| contents[position.y][position.x] == Contents::Asteroid)
            .collect();

        Map {width, height, contents, asteroids}
    }

    fn get(&self, position: &Position) -> &Contents {
        &self.contents[position.y][position.x]
    }

    /// Returns the most asteroids that are visible from another asteroid on this map.
    fn max_visible(&self) -> usize {
        self.asteroids.iter()
            .map(|pos| self.total_visible(&pos))
            .max()
            .unwrap_or(0)
    }

    /// Returns the total number of asteroids that are visible from the given position.
    fn total_visible(&self, position: &Position) -> usize {
        let visible: HashSet<Position> = self.asteroids.iter()
            .flat_map(|extent| self.visible_asteroid(position, &extent))
            .collect();

        visible.len()
    }

    /// Returns the closest visible asteroid from the given position to the extent.
    /// Asteroids in positions that are between the given position and the extent block asteroids
    /// that are further away.
    fn visible_asteroid(&self, position: &Position, extent: &Position) -> Option<Position> {
        position.to(extent).find(|pos| self.get(&pos) == &Contents::Asteroid)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Contents::*;

    #[test]
    fn test_parse_map() {
        let lines = vec!["..#", "#.#"];
        let expected = Map {
            width: 3,
            height: 2,
            contents: vec![
                vec![Empty, Empty, Asteroid],
                vec![Asteroid, Empty, Asteroid],
            ],
            asteroids: vec![
                Position::new(0, 1),
                Position::new(2, 0),
                Position::new(2, 1),
            ],
        };

        assert_eq!(Map::parse(&lines), expected);
    }

    #[test]
    fn test_max_visible() {
        let map = sample_map();

        assert_eq!(map.max_visible(), 8);
    }

    #[test]
    fn test_total_visible() {
        let map = sample_map();

        assert_eq!(map.total_visible(&Position::new(1, 0)), 7);
        assert_eq!(map.total_visible(&Position::new(0, 2)), 6);
        assert_eq!(map.total_visible(&Position::new(3, 4)), 8);
        assert_eq!(map.total_visible(&Position::new(4, 4)), 7);
        assert_eq!(map.total_visible(&Position::new(4, 2)), 5);
    }

    #[test]
    fn test_get() {
        let map = sample_map();

        assert_eq!(map.get(&Position::new(0, 0)), &Contents::Empty);
        assert_eq!(map.get(&Position::new(1, 0)), &Contents::Asteroid);
        assert_eq!(map.get(&Position::new(2, 2)), &Contents::Asteroid);
    }

    fn sample_map() -> Map {
        let lines = vec![
            ".#..#", // .7..7
            ".....", // .....
            "#####", // 67775
            "....#", // ....7
            "...##", // ...87
        ];

        Map::parse(&lines)
    }
}

/// Converts the given file into a vector of lines.
fn file_to_lines(name: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(name).expect("File does not exist.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Error parsing line."))
        .collect()
}

fn main() {
    let lines = file_to_lines("input.txt");
    let str_lines = lines.iter().map(|s| s as &str).collect();
    let map = Map::parse(&str_lines);

    println!("Part 1: {}", map.max_visible());
}
