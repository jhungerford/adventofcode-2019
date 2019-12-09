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

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
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

fn wire_locations(segments: &Vec<Segment>) -> Vec<Position> {
    let mut locations = Vec::new();

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

            locations.push(position);
        }
    }

    locations.to_owned()
}

fn closest_intersection(s1: &Vec<Segment>, s2: &Vec<Segment>) -> i32 {
    let positions1: HashSet<Position> = wire_locations(s1).into_iter().collect();
    let positions2: HashSet<Position> = wire_locations(s2).into_iter().collect();

    positions1.intersection(&positions2)
        .map(|p| manhatten_distance(&ORIGIN, p))
        .min()
        .unwrap()
}

fn sorted_intersection_distances(positions: &Vec<Position>, intersections: &HashSet<&Position>) -> Vec<i32> {
    // Vec of (distance, position), where the only positions are intersections.
    let mut distance_intersections: Vec<(usize, &Position)> = positions.iter().enumerate()
        .filter(|(_, position)| intersections.contains(position)) // Only care about intersections.
        .map(|(i, position)| (i + 1, position)) // Position distances start at 1, not 0, since the origin isn't included.
        .collect();
    
    // Sort by intersection, then map to just distance.
    distance_intersections.sort_by(|(_, position1), (_, position2)| position1.cmp(position2));

    distance_intersections.iter()
        .map(|(i, _)| *i as i32) // Only care about distances now.
        .collect()
}

fn first_intersection(s1: &Vec<Segment>, s2: &Vec<Segment>) -> i32 {
    let positions1 = wire_locations(s1);
    let positions2 = wire_locations(s2);

    // Figure out where the two wires intersect.
    let positions_set1: HashSet<Position> = positions1.iter().cloned().collect();
    let positions_set2: HashSet<Position> = positions2.iter().cloned().collect();

    let intersections: HashSet<&Position> = positions_set1.intersection(&positions_set2).collect();

    // Grab a list of distances to each intersection.  Both returned lists have intersections at the same indexes.
    let distances1 = sorted_intersection_distances(&positions1, &intersections);
    let distances2 = sorted_intersection_distances(&positions2, &intersections);

    // Zip the distances together.  Since both distance lists have intersection positions at the same index,
    // the first intersection is the one with the fewest combined steps.
    return distances1.iter().zip(distances2)
        .map(|(d1, d2)| d1 + d2)
        .min()
        .unwrap()
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

    println!("Part 1: {}", closest_intersection(&segments1, &segments2));
    println!("Part 2: {}", first_intersection(&segments1, &segments2));

    Ok(())
}
