use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::iter::Sum;
use std::ops::{Add, AddAssign};
use std::path::Path;

use regex::Regex;

/// Vector is an x, y, and z coordinate in 3-space.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    /// Returns the energy for this vector.  Energy in this space is the sum of the absolute
    /// values of the components of this vector.
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sum for Vector {
    fn sum<I: Iterator<Item=Vector>>(iter: I) -> Self {
        iter.fold(ZERO, |sum, v| sum + v)
    }
}

const ZERO: Vector = Vector { x: 0, y: 0, z: 0 };

/// A moon is a solar object with a position and velocity.
#[derive(Eq, PartialEq, Debug)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    /// Parses a moon from the given string like `<x=4, y=1, z=1>`.  Panics if the line is invalid.
    /// Also accepts moons with velocities, like `pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>`.
    fn parse(s: &str) -> Moon {
        let pattern = Regex::new(r"^(?:pos=)?<x=\s*(-?\d+), y=\s*(-?\d+), z=\s*(-?\d+)>(?:, vel=<x=\s*(-?\d+), y=\s*(-?\d+), z=\s*(-?\d+)>)?$").unwrap();
        let captures = pattern.captures(s).unwrap();

        Moon {
            position: Vector {
                x: captures[1].parse::<i32>().unwrap(),
                y: captures[2].parse::<i32>().unwrap(),
                z: captures[3].parse::<i32>().unwrap(),
            },
            velocity: Vector {
                x: captures.get(4).map_or(0, |x| x.as_str().parse::<i32>().unwrap()),
                y: captures.get(5).map_or(0, |y| y.as_str().parse::<i32>().unwrap()),
                z: captures.get(6).map_or(0, |z| z.as_str().parse::<i32>().unwrap()),
            },
        }
    }

    /// Returns the pull on this moon towards the other moon.
    fn gravity(&self, other: &Moon) -> Vector {
        /// Returns -1, 0, or 1 to pull the given value towards the other value.
        #[inline]
        fn pull(value: i32, to: i32) -> i32 {
            if value > to {
                -1
            } else if value < to {
                1
            } else {
                0
            }
        }

        Vector {
            x: pull(self.position.x, other.position.x),
            y: pull(self.position.y, other.position.y),
            z: pull(self.position.z, other.position.z),
        }
    }

    /// A moon's energy is the product of it's velocity and position.
    fn energy(&self) -> i32 {
        self.velocity.energy() * self.position.energy()
    }
}

/// System is a collection of moons that can interact with one another.
struct System {
    moons: Vec<Moon>
}

impl System {
    /// Steps this system forward by one time unit.  Moons affect each other's velocity by gravity,
    /// then change positions based on their velocity.
    fn step(&mut self) {
        // Gravity.  Each moon's position changes by the pull of the other moons.  On each axis,
        // adds or subtracts 1 to the moon's velocity on each axis to pull moons together.
        let new_velocities: Vec<Vector> = self.moons.iter().map(|moon| {
            moon.velocity + self.moons.iter()
                .filter(|&other_moon| other_moon != moon)
                .map(|other_moon| moon.gravity(other_moon))
                .sum()
        }).collect();

        let new_positions: Vec<Vector> = self.moons.iter().enumerate().map(|(i, moon)| {
            moon.position + new_velocities[i]
        }).collect();

        self.moons = (0..self.moons.len()).map(|i| Moon {
            position: new_positions[i],
            velocity: new_velocities[i],
        }).collect();
    }

    /// Returns the total energy in this system.  Energy is the sum of the potential and kinetic
    /// energy of all of the moons, where potential energy is the sum of the absolute values
    /// of a moon's position and kinetic is velocity.
    fn energy(&self) -> i32 {
        self.moons.iter().map(Moon::energy).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_vector() {
        assert_eq!(Vector { x: 1, y: 2, z: 3 }, ZERO + Vector { x: 1, y: 2, z: 3 });
        assert_eq!(Vector {x: 2, y: 4, z: 6}, Vector { x: 1, y: 2, z: 3 } + Vector { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn add_assign_vector() {
        let mut v = Vector {x: 1, y: 2, z: 3};

        v += ZERO;
        assert_eq!(Vector {x: 1, y: 2, z: 3}, v);

        v += Vector {x: 2, y: 2, z: 2};
        assert_eq!(Vector {x: 3, y: 4, z: 5}, v);
    }

    #[test]
    fn sum_vector() {
        let sum = vec![
            Moon::parse("<x=-1, y=0, z=2>").position,
            Moon::parse("<x=2, y=-10, z=-7>").position,
            Moon::parse("<x=4, y=-8, z=8>").position,
            Moon::parse("<x=3, y=5, z=-1>").position,
        ].into_iter().sum();

        assert_eq!(Vector {x: 8, y: -13, z: 2}, sum);
    }

    #[test]
    fn parse_moon() {
        let expected = Moon {
            position: Vector { x: 4, y: 1, z: 1 },
            velocity: ZERO,
        };

        assert_eq!(expected, Moon::parse("<x=4, y=1, z=1>"));
    }

    #[test]
    fn parse_moon_with_velocity() {
        let expected = Moon {
            position: Vector {x: -1, y: 0, z: 2},
            velocity: Vector {x: 3, y: -1, z: -1},
        };

        assert_eq!(Moon::parse("pos=<x=-1, y=  0, z= 2>, vel=<x= 3, y=-1, z=-1>"), expected);
    }

    #[test]
    fn moon_gravity() {
        let moon1 = &Moon::parse("<x=4, y=-1, z=1>");
        let moon2 = &Moon::parse("<x=2, y=10, z=1>");

        assert_eq!(moon1.gravity(moon2), Vector { x: -1, y: 1, z: 0});
        assert_eq!(moon2.gravity(moon1), Vector { x: 1, y: -1, z: 0});
    }

    #[test]
    fn system_step() {
        let mut system = System {moons: vec![
            Moon::parse("<x=-1, y=0, z=2>"),
            Moon::parse("<x=2, y=-10, z=-7>"),
            Moon::parse("<x=4, y=-8, z=8>"),
            Moon::parse("<x=3, y=5, z=-1>"),
        ]};

        system.step();

        assert_eq!(system.moons, vec![
            Moon::parse("pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>"),
            Moon::parse("pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>"),
            Moon::parse("pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>"),
            Moon::parse("pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>"),
        ]);
    }

    #[test]
    fn system_total_energy() {
        let mut system = System {moons: vec![
            Moon::parse("<x=-1, y=0, z=2>"),
            Moon::parse("<x=2, y=-10, z=-7>"),
            Moon::parse("<x=4, y=-8, z=8>"),
            Moon::parse("<x=3, y=5, z=-1>"),
        ]};
        assert_eq!(system.energy(), 0);

        system.step();
        assert_eq!(system.energy(), 229);
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

    let mut system = System {
        moons: lines.into_iter().map(|line| Moon::parse(&line)).collect()
    };

    // Part 1: after 1000 steps, what's the total energy in the system?
    for _ in 0..1000 {
        system.step()
    }

    println!("Part 1: {}", system.energy());
}
