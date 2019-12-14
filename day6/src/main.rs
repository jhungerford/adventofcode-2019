use std::collections::HashSet;
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

/// Given a map of orbits and an object, returns a Vec that maps the object to the center of the system.  
/// Object isn't included in the returned list.
fn orbit<'a>(outer_to_inner: &HashMap<&'a str, &'a str>, object: &str) -> Vec<&'a str> {
    let mut orbit = Vec::new();
    let mut at = outer_to_inner.get(object);
    
    while at != None {
        orbit.push(*at.unwrap());
        at = outer_to_inner.get(at.unwrap());
    }

    orbit.to_owned()

}

/// Returns the total number of orbits present in the given map.
fn total_orbits(outer_to_inner: &HashMap<&str, &str>) -> usize {
    let mut total_orbits = 0;
    
    for object in outer_to_inner.keys() {
        total_orbits += orbit(outer_to_inner, object).len();
    }

    total_orbits
}

/// Returns the fewest number of transfers to get from the object 'from' is orbiting to the object 'to' is orbiting.
fn fewest_transfers(outer_to_inner: &HashMap<&str, &str>, from: &str, to: &str) -> usize {
    // Convert both full orbits to sets, figure out the difference.  Overlapping objects aren't part of the shortest path
    // between objects, so the shortest path is the symmetric difference between the sets of objects in each orbit.
    let from_orbit: HashSet<&str> = orbit(outer_to_inner, from).into_iter().collect();
    let to_orbit: HashSet<&str> = orbit(outer_to_inner, to).into_iter().collect();

    from_orbit.symmetric_difference(&to_orbit).count()
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

    println!("Part 1: {}", total_orbits(&outer_to_inner));
    println!("Part 2: {}", fewest_transfers(&outer_to_inner, "YOU", "SAN"));

    Ok(())
}
