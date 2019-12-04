use std::collections::HashSet;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

#[derive(Copy, Clone, Debug)]
enum Direction {
    UP, DOWN, LEFT, RIGHT
}

fn parse_direction(s: &str) -> Direction {
    match s.as_ref() {
        "U" => Direction::UP,
        "D" => Direction::DOWN,
        "L" => Direction::LEFT,
        "R" => Direction::RIGHT,
        _ => panic!("Invalid direction: {}", s),
    }
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    direction: Direction,
    length: i32,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

const ORIGIN: Position = Position{x: 0, y: 0};

fn manhatten_distance(p1: &Position, p2: &Position) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn wire_locations(segments: Vec<Segment>) -> HashSet<Position> {
    let mut locations = HashSet::new();

    // Move before inserting the position so the origin isn't included.
    let mut position = ORIGIN;
    for s in segments {
        for _ in 1..s.length + 1 {
            position = match s.direction {
                Direction::UP => Position{x: position.x, y: position.y - 1},
                Direction::DOWN => Position{x: position.x, y: position.y + 1},
                Direction::LEFT => Position{x: position.x - 1, y: position.y},
                Direction::RIGHT => Position{x: position.x + 1, y: position.y},
            };

            locations.insert(position);
        }
    }

    locations.to_owned()
}

fn closest_intersection(s1: Vec<Segment>, s2: Vec<Segment>) -> Option<i32> {
    let positions1 = wire_locations(s1);
    let positions2 = wire_locations(s2);

    positions1.intersection(&positions2)
        .map(|p| manhatten_distance(&ORIGIN, p))
        .min()
}

fn parse_segments(line: &str) -> Vec<Segment> {
    // line is a comma-separated list of directions and distances.  U10 means that the wire goes up 10 squares.
    let parsed = line.split(",")
        .map(|s| Segment{
            direction: parse_direction(&s[0..1]), 
            length: str::parse::<i32>(&s.trim()[1..]).unwrap(),
        })
        .collect::<Vec<Segment>>();

    parsed.to_owned()
}


fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut line1 = String::new();
    let mut line2 = String::new();

    reader.read_line(&mut line1)?;
    reader.read_line(&mut line2)?;

    let segments1 = parse_segments(&line1);
    let segments2 = parse_segments(&line2);

    println!("Part 1: {}", closest_intersection(segments1, segments2).unwrap());

    Ok(())
}
