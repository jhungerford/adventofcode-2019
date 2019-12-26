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

    fn max_visible_position(&self) -> Position {
        self.asteroids.iter()
            .max_by(|&pos_a, &pos_b| self.total_visible(pos_a).cmp(&self.total_visible(pos_b)))
            .unwrap()
            .clone()
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

    /// Returns asteroids in the order that they should be vaporized.  The laser starts at 0 degrees
    /// from the given position, and vaporizes one asteroid that it can see.  It rotates clockwise,
    /// vaporizing asteroids until they're all gone.  If one asteroid occludes another, the laser
    /// must make a full rotation before vaporizing the next asteroid in line.
    fn vaporize_order(&self, from: &Position) -> Vec<&Position> {
        #[derive(Debug)]
        struct AngleDistance<'a> {
            position: &'a Position,
            angle: f64,
            distance: f64,
        }

        #[derive(Debug)]
        struct AngleOrder<'a> {
            position: &'a Position,
            angle: f64,
            order: usize,
        }

        #[derive(Debug)]
        struct AngleCount {
            angle: f64,
            count: usize,
        }

        // Convert asteroids (excluding the station) -> (angle, distance).
        // For all of the asteroids at an angle, convert distance into the order the asteroids
        // are vaporized.  From (angle, order), sort by order then angle to get the final result.
        let mut angle_distances: Vec<AngleDistance> = self.asteroids.iter()
            .filter(|&asteroid| asteroid != from)
            .map(|asteroid| AngleDistance {
                position: asteroid,
                angle: from.angle(&asteroid),
                distance: from.distance(&asteroid)
            }).collect();

        angle_distances.sort_by(|a, b| {
            a.angle.partial_cmp(&b.angle).unwrap()
                .then(a.distance.partial_cmp(&b.distance).unwrap())
        });

        // State is the previous angle and order (number of asteroids seen at that angle so far)
        let mut angle_orders: Vec<AngleOrder> = angle_distances.iter()
            .scan(AngleCount { angle: -1 as f64, count: 0 }, |angle_count, angle_distance| {
                if angle_distance.angle == angle_count.angle {
                    angle_count.count += 1;
                } else {
                    *angle_count = AngleCount {
                        angle: angle_distance.angle,
                        count: 0,
                    };
                }

                Some(AngleOrder{
                    position: angle_distance.position,
                    angle: angle_distance.angle,
                    order: angle_count.count,
                })
            }).collect();

        angle_orders.sort_by(|a, b| {
            a.order.cmp(&b.order).then(a.angle.partial_cmp(&b.angle).unwrap())
        });

        angle_orders.into_iter().map(|angle_order| angle_order.position).collect()
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

    #[test]
    fn test_vaporize_order() {
        let map = vaporize_map();

        let actual = map.vaporize_order(&Position::new(8, 3));
        assert_eq!(vec![
            &Position::new(8, 1),
            &Position::new(9, 0),
            &Position::new(9, 1),
            &Position::new(10, 0),
            &Position::new(9, 2),
            &Position::new(11, 1),
            &Position::new(12, 1),
            &Position::new(11, 2),
        ], &actual[0..8]);
    }

    #[test]
    fn test_vaporize_order_big_map() {
        let lines = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];

        let map = Map::parse(&lines);

        let station = map.max_visible_position();
        let vaporize_order = map.vaporize_order(&station);
        let two_hundredth = vaporize_order[199];

        assert_eq!(&Position::new(8, 2), two_hundredth);
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

    fn vaporize_map() -> Map {
        // Station at (8, 3).
        let lines = vec![
            ".#....#####...#..",
            "##...##.#####..##",
            "##...#...#.#####.",
            "..#.....#...###..",
            "..#.#.....#....##",
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

    let station = map.max_visible_position();
    let vaporize_order = map.vaporize_order(&station);
    let two_hundredth = vaporize_order[199];

    println!("Part 2: {}", two_hundredth.x * 100 + two_hundredth.y);
}
