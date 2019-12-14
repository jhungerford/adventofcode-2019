use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

/// Parses the given orbit line - A)B will return (A, B), and means that B orbits A.
fn parse_line(line: &str) -> (&str, &str) {
    let split: Vec<&str> = line.splitn(2, ")").collect();
    (split[0], split[1])
}

/// Given a list of lines like A)B, parses them into a map of outer -> inner orbits (B -> A)
fn parse_orbits<'a>(lines: &Vec<&'a str>) -> HashMap<&'a str, &'a str> {
    let mut orbits = HashMap::new();

    for line in lines {
        let (inner, outer) = parse_line(line);
        orbits.insert(outer, inner);
    }

    orbits.to_owned()
}

fn total_orbits(outer_to_inner: HashMap<&str, &str>) -> u32 {
    let mut total_orbits = 0;
    
    for outer in outer_to_inner.keys() {
        let mut object = outer_to_inner.get(outer);
        while object != None {
            object = outer_to_inner.get(object.unwrap());
            total_orbits += 1;
        }
    }

    total_orbits
}

#[cfg(test)]
mod test;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let lines: Vec<String> = f.lines()
        .map(|line| line.unwrap())
        .collect();

    let str_lines: Vec<&str> = lines.iter()
        .map(|s| s.as_ref())
        .collect();

    let outer_to_inner = parse_orbits(&str_lines);

    println!("Part 1: {}", total_orbits(outer_to_inner));

    Ok(())
}
