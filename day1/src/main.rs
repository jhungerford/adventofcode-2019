use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

// Calculates the amount of fuel required for the given mass: mass / 3 - 2
fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

// Calculates the amount of fuel required for the mass and the fuel to lift the mass, and so on.
fn tyranny_fuel(mass: i32) -> i32 {
    let fuel = fuel(mass);
    match fuel <= 0 {
        false => {
            fuel + tyranny_fuel(fuel)
        }
        true => 0
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut total_fuel = 0;
    let mut total_tyranny_fuel = 0;
    for line in f.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        total_fuel += fuel(mass);
        total_tyranny_fuel += tyranny_fuel(mass);
    }

    println!("Part 1: {}", total_fuel);
    println!("Part 2: {}", total_tyranny_fuel);

    Ok(())
}
