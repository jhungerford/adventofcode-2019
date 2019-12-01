use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    
    let mut total_fuel = 0;
    for line in f.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        total_fuel += fuel(mass);
    }

    println!("Part 1: {}", total_fuel);

    Ok(())
}
