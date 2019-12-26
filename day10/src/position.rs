use std::fmt;
use std::iter::Iterator;
use std::ops::Add;
use std::f64::consts::PI;

/// Position on the map - x is from the left edge, and y is from the top.  (0,0) is the top left corner.
#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// PositionStep is the state of the `position.to(other)` iterator.
#[derive(Eq, PartialEq, Debug)]
pub struct PositionStep {
    position: Position,
    end: Position,
    x: i32,
    y: i32,
}

impl Add<&PositionStep> for Position {
    type Output = Self;

    fn add(self, step: &PositionStep) -> Position {
        Position {
            x: (self.x as i32 + step.x) as usize,
            y: (self.y as i32 + step.y) as usize,
        }
    }
}

impl Iterator for PositionStep {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.end {
            return None
        }

        let next_position = self.position + self;
        self.position = next_position;
        Some(next_position)
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {x, y}
    }

    /// Returns an iterator of points that can be visited from this position to the other position.
    pub fn to<'a>(&'a self, other: &'a Position) -> PositionStep {
        let direction_x = other.x as i32 - self.x as i32;
        let direction_y = other.y as i32 - self.y as i32;

        let step = match (direction_x, direction_y) {
            (x, y) if x == 0 && y == 0 => 1,
            (x, y) if x == 0 => y.abs(),
            (x, y) if y == 0 => x.abs(),
            (x, y) => gcd(x, y)
        };

        PositionStep {
            position: self.clone(),
            end: other.clone(),
            x: direction_x / step,
            y: direction_y / step,
        }
    }

    /// Returns the angle from this position to the other position, in degrees.  Up is 0.
    pub fn angle(&self, other: &Position) -> f64 {
        let x = other.x as f64 - self.x as f64;
        let y = self.y as f64 - other.y as f64;

        // atan2 returns the angle from (0, 0) to (x, y) in radians.
        // The right half is [0 - 180], but the left half is [0, -180] (0 is up in both)
        // We want [0, 360) degrees with up at 0, so transform the output.
        let angle = (x.atan2(y)) * 180 as f64 / PI;
        if angle < 0 as f64 { // Less than 0 makes the output range [0, 360)
            360 as f64 + angle
        } else {
            angle
        }
    }

    /// Returns the straight-line distance between this position and the other position.
    pub fn distance(&self, other: &Position) -> f64 {
        // a^2 + b^2 = c^2.  Square distance would work for sorting distance too,
        // but there's few enough asteroids in this problem that the sqrt is fast enough.
        let x = self.x as f64 - other.x as f64;
        let y = self.y as f64 - other.y as f64;

        (x.powi(2) + y.powi(2)).sqrt()
    }
}

/// Returns the greatest common denominator between the two numbers, which must both be non-zero.
/// Uses euclid's algorithm, which is slow but easy to implement.
fn gcd(x: i32, y: i32) -> i32 {
    assert!(x != 0 && y != 0);

    let mut x = x.abs();
    let mut y = y.abs();
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 1), 1);

        assert_eq!(gcd(3, 8), 1);

        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(-2, -4), 2);
        assert_eq!(gcd(2, -4), 2);
        assert_eq!(gcd(-2, 4), 2);

        assert_eq!(gcd(1234, 456789), 1);

        assert_eq!(gcd(24, 120), 24);
        assert_eq!(gcd(24, 90), 6);
    }

    #[test]
    fn test_position_step_add() {
        let position = Position::new(2, 2);
        let position_step = PositionStep {
            position: position.clone(),
            end: position.clone(),
            x: 1,
            y: 2,
        };

        assert_eq!(position + &position_step, Position::new(3, 4));
    }

    #[test]
    fn test_to_iterator() {
        // Down 1, right 1 repeatedly.
        let mut visited: Vec<Position> = Position::new(0, 0).to(&Position::new(4, 4)).collect();
        assert_eq!(visited, vec![
            Position::new(1, 1),
            Position::new(2, 2),
            Position::new(3, 3),
            Position::new(4, 4),
        ]);

        // Start and end on the same square - iterator doesn't have the start, so it should be empty
        visited = Position::new(2, 2).to(&Position::new(2, 2)).collect();
        assert_eq!(visited, Vec::new());

        visited = Position::new(4, 2).to(&Position::new(0, 0)).collect();
        assert_eq!(visited, vec![
            Position::new(2, 1),
            Position::new(0, 0),
        ]);

        visited = Position::new(4, 2).to(&Position::new(0, 1)).collect();
        assert_eq!(visited, vec![
            Position::new(0, 1),
        ]);

        visited = Position::new(2, 2).to(&Position::new(0, 2)).collect();
        assert_eq!(visited, vec![
            Position::new(1, 2),
            Position::new(0, 2),
        ])
    }

    #[test]
    fn test_angle() {
        let center = Position::new(2, 2);

        assert_eq!(0 as f64, center.angle(&Position::new(2, 0)));
        assert_eq!(45 as f64, center.angle(&Position::new(3, 1)));
        assert_eq!(90 as f64, center.angle(&Position::new(3, 2)));
        assert_eq!(135 as f64, center.angle(&Position::new(3, 3)));
        assert_eq!(180 as f64, center.angle(&Position::new(2, 3)));
        assert_eq!(225 as f64, center.angle(&Position::new(1, 3)));
        assert_eq!(270 as f64, center.angle(&Position::new(1, 2)));
        assert_eq!(315 as f64, center.angle(&Position::new(1, 1)));
    }
}