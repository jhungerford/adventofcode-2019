use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::iter::Sum;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;
use num::integer::lcm;

/// Vector is an x, y, and z coordinate in 3-space.
#[derive(Eq, PartialEq, Clone)]
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
        *self = Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector {
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

impl Sum for Vector {
    fn sum<I: Iterator<Item=Vector>>(iter: I) -> Self {
        iter.fold(Vector::default(), |sum, v| sum + v)
    }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct ParseErr {
    message: String,
}

/// A moon is a solar object with a position and velocity.
#[derive(Eq, PartialEq)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    /// Adjusts this moon's velocity to pull it towards the other moon.
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

    /// Updates this moon's position based on its velocity.
    fn update_position(&mut self) {
        self.position += self.velocity.clone();
    }

    /// A moon's energy is the product of it's velocity and position.
    fn energy(&self) -> i32 {
        self.velocity.energy() * self.position.energy()
    }
}

impl FromStr for Moon {
    type Err = ParseErr;

    /// Parses a moon from the given string like `<x=4, y=1, z=1>`.  Panics if the line is invalid.
    /// Also accepts moons with velocities, like `pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^(?:pos=)?<x=\s*(-?\d+), y=\s*(-?\d+), z=\s*(-?\d+)>(?:, vel=<x=\s*(-?\d+), y=\s*(-?\d+), z=\s*(-?\d+)>)?$").unwrap();
        let captures = pattern.captures(s).unwrap();

        Ok(Moon {
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
        })
    }
}

impl Debug for Moon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos={:?}, vel={:?}", self.position, self.velocity)
    }
}

/// System is a collection of moons that can interact with one another.
struct System {
    moons: Vec<Moon>
}

impl System {
    /// Loads a system from the given file.
    fn load(name: &str) -> System {
        let file = File::open(name).expect("File does not exist.");
        let buf = BufReader::new(file);

        let moons: Vec<Moon> = buf.lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        System { moons }
    }

    /// Steps this system forward by one time unit.  Moons affect each other's velocity by gravity,
    /// then change positions based on their velocity.
    fn step(&mut self) {
        // Gravity.  Each moon's position changes by the pull of the other moons.  On each axis,
        // adds or subtracts 1 to the moon's velocity on each axis to pull moons together.
        let mut new_velocities: Vec<Vector> = self.moons.iter()
            .map(|moon| moon.velocity.clone())
            .collect();

        for pair in (0..self.moons.len()).permutations(2) {
            let moon = &self.moons[pair[0]];
            let other = &self.moons[pair[1]];

            new_velocities[pair[0]] += moon.gravity(other);
        }

        for (i, moon) in self.moons.iter_mut().enumerate() {
            moon.velocity = new_velocities[i].clone();
        }

        // Position.  Each moon moves its position based on its velocity.
        for moon in &mut self.moons {
            moon.update_position();
        }
    }

    /// Returns the total energy in this system.  Energy is the sum of the potential and kinetic
    /// energy of all of the moons, where potential energy is the sum of the absolute values
    /// of a moon's position and kinetic is velocity.
    fn energy(&self) -> i32 {
        self.moons.iter().map(Moon::energy).sum()
    }

    /// Returns the number of steps that this system takes before all of the moons' positions
    /// and velocities match a previous point in time.
    fn repeat(&self) -> usize {
        // Axes only affect themselves, so LCM of each axis is the steps until the system repeats.
        let x_repeat = self.axis_repeat(|moon| moon.position.x);
        let y_repeat = self.axis_repeat(|moon| moon.position.y);
        let z_repeat = self.axis_repeat(|moon| moon.position.z);

        lcm(x_repeat, lcm(y_repeat, z_repeat))
    }

    /// Returns the number of cycles before the given axis repeats.
    fn axis_repeat(&self, axis: fn(&Moon) -> i32) -> usize {
        let initial_positions: Vec<i32> = self.moons.iter().map(axis).collect();
        let initial_velocities = vec![0; initial_positions.len()];

        let mut positions = initial_positions.clone();
        let mut velocities = initial_velocities.clone();

        let mut rounds = 0;
        loop {
            for pair in (0..positions.len()).permutations(2) {
                let pos = positions[pair[0]];
                let other_pos = positions[pair[1]];

                velocities[pair[0]] += if pos > other_pos {
                    -1
                } else if pos < other_pos {
                    1
                } else {
                    0
                };
            }

            positions.iter_mut().zip(&velocities).for_each(|(p, v)| *p += v);
            rounds += 1;

            if positions == initial_positions && velocities == initial_velocities {
                return rounds;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_vector() {
        assert_eq!(Vector { x: 1, y: 2, z: 3 }, Vector::default() + Vector { x: 1, y: 2, z: 3 });
        assert_eq!(Vector {x: 2, y: 4, z: 6}, Vector { x: 1, y: 2, z: 3 } + Vector { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn add_assign_vector() {
        let mut v = Vector {x: 1, y: 2, z: 3};

        v += Vector::default();
        assert_eq!(Vector {x: 1, y: 2, z: 3}, v);

        v += Vector {x: 2, y: 2, z: 2};
        assert_eq!(Vector {x: 3, y: 4, z: 5}, v);
    }

    #[test]
    fn sum_vector() {
        let sum = vec![
            "<x=-1, y=0, z=2>".parse::<Moon>().unwrap().position,
            "<x=2, y=-10, z=-7>".parse::<Moon>().unwrap().position,
            "<x=4, y=-8, z=8>".parse::<Moon>().unwrap().position,
            "<x=3, y=5, z=-1>".parse::<Moon>().unwrap().position,
        ].into_iter().sum();

        assert_eq!(Vector {x: 8, y: -13, z: 2}, sum);
    }

    #[test]
    fn parse_moon() {
        let expected = Moon {
            position: Vector { x: 4, y: 1, z: 1 },
            velocity: Vector::default(),
        };

        assert_eq!(expected, "<x=4, y=1, z=1>".parse().unwrap());
    }

    #[test]
    fn parse_moon_with_velocity() {
        let expected = Moon {
            position: Vector {x: -1, y: 0, z: 2},
            velocity: Vector {x: 3, y: -1, z: -1},
        };

        assert_eq!("pos=<x=-1, y=  0, z= 2>, vel=<x= 3, y=-1, z=-1>".parse::<Moon>().unwrap(), expected);
    }

    #[test]
    fn moon_gravity() {
        let moon1: Moon = "<x=4, y=-1, z=1>".parse().unwrap();
        let moon2: Moon = "<x=2, y=10, z=1>".parse().unwrap();

        assert_eq!(moon1.gravity(&moon2), Vector { x: -1, y: 1, z: 0});
        assert_eq!(moon2.gravity(&moon1), Vector { x: 1, y: -1, z: 0});
    }

    #[test]
    fn system_step() {
        let mut system = System {moons: vec![
            "<x=-1, y=0, z=2>".parse().unwrap(),
            "<x=2, y=-10, z=-7>".parse().unwrap(),
            "<x=4, y=-8, z=8>".parse().unwrap(),
            "<x=3, y=5, z=-1>".parse().unwrap(),
        ]};

        system.step();

        assert_eq!(system.moons, vec![
            "pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>".parse().unwrap(),
            "pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>".parse().unwrap(),
            "pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>".parse().unwrap(),
            "pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>".parse().unwrap(),
        ]);
    }

    #[test]
    fn system_total_energy() {
        let mut system = System {moons: vec![
            "<x=-1, y=0, z=2>".parse().unwrap(),
            "<x=2, y=-10, z=-7>".parse().unwrap(),
            "<x=4, y=-8, z=8>".parse().unwrap(),
            "<x=3, y=5, z=-1>".parse().unwrap(),
        ]};
        assert_eq!(system.energy(), 0);

        system.step();
        assert_eq!(system.energy(), 229);
    }

    #[test]
    fn system_repeat_sample1() {
        let system = System {moons: vec![
            "<x=-1, y=0, z=2>".parse().unwrap(),
            "<x=2, y=-10, z=-7>".parse().unwrap(),
            "<x=4, y=-8, z=8>".parse().unwrap(),
            "<x=3, y=5, z=-1>".parse().unwrap(),
        ]};

        assert_eq!(system.repeat(), 2772);
    }

    #[test]
    fn system_repeat_sample2() {
        let system = System {moons: vec![
            "<x=-8, y=-10, z=0>".parse().unwrap(),
            "<x=5, y=5, z=10>".parse().unwrap(),
            "<x=2, y=-7, z=3>".parse().unwrap(),
            "<x=9, y=-8, z=-3>".parse().unwrap(),
        ]};

        assert_eq!(system.repeat(), 4686774924);
    }
}

fn main() {
    let mut system = System::load("input.txt");

    // Part 1: after 1000 steps, what's the total energy in the system?
    for _ in 0..1000 {
        system.step();
    }

    println!("Part 1: {}", system.energy());

    // Part 2: how long does it take for the universe to reach a previous point in time?
    let system = System::load("input.txt");
    println!("Part 2: {}", system.repeat());
}
