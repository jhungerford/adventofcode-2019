use computer::*;
use std::collections::HashMap;

mod computer;

// 0 = black, 1 = white
#[derive(Copy, Clone)]
enum PanelColor {
    Black,
    White,
}

impl PanelColor {
    fn from(value: i64) -> PanelColor {
        match value {
            0 => PanelColor::Black,
            1 => PanelColor::White,
            n => panic!("Invalid panel color: {}", n),
        }
    }

    fn value(self) -> i64 {
        match self {
            PanelColor::Black => 0,
            PanelColor::White => 1,
        }
    }
}

// 0 = left, 1 = right
#[derive(Copy, Clone)]
enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn from(value: i64) -> TurnDirection {
        match value {
            0 => TurnDirection::Left,
            1 => TurnDirection::Right,
            n => panic!("Invalid turn direction: {}", n),
        }
    }
}

#[derive(Copy, Clone)]
enum RobotDirection { Up, Down, Left, Right }

impl RobotDirection {
    fn turn(self, direction: TurnDirection) -> RobotDirection {
        match (self, direction) {
            (RobotDirection::Up, TurnDirection::Left) => RobotDirection::Left,
            (RobotDirection::Up, TurnDirection::Right) => RobotDirection::Right,
            (RobotDirection::Down, TurnDirection::Left) => RobotDirection::Right,
            (RobotDirection::Down, TurnDirection::Right) => RobotDirection::Left,
            (RobotDirection::Left, TurnDirection::Left) => RobotDirection::Down,
            (RobotDirection::Left, TurnDirection::Right) => RobotDirection::Up,
            (RobotDirection::Right, TurnDirection::Left) => RobotDirection::Up,
            (RobotDirection::Right, TurnDirection::Right) => RobotDirection::Down,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn forward(self, direction: &RobotDirection) -> Position {
        match direction {
            RobotDirection::Up => Position {x: self.x, y: self.y - 1},
            RobotDirection::Down => Position {x: self.x, y: self.y + 1},
            RobotDirection::Left => Position {x: self.x - 1, y: self.y},
            RobotDirection::Right => Position {x: self.x + 1, y: self.y},
        }
    }
}

enum NextRobotInput {
    Paint, Turn,
}

struct RobotState {
    position: Position,
    direction: RobotDirection,
    panels: HashMap<Position, PanelColor>,
    next_input: NextRobotInput
}

impl RobotState {
    fn new() -> RobotState {
        RobotState {
            position: Position {x: 0, y: 0},
            direction: RobotDirection::Up,
            panels: HashMap::new(),
            next_input: NextRobotInput::Paint,
        }
    }

    /// Paints the square that the robot is currently on.
    fn paint(&mut self, color: i64) {
        self.panels.insert(self.position.clone(), PanelColor::from(color));
        self.next_input = NextRobotInput::Turn;
    }

    /// Turns the given direction, then moves forward exactly one panel
    fn turn(&mut self, direction: i64) {
        self.direction = self.direction.turn(TurnDirection::from(direction));
        self.position = self.position.forward(&self.direction);
        self.next_input = NextRobotInput::Paint;
    }

    /// Returns the number of squares that this robot painted at least once.
    fn num_squares_painted(&self) -> usize {
        self.panels.len()
    }
}


impl ProgramIO for RobotState {
    /// Input that reads the color of the panel under the robot.
    fn input(&mut self) -> i64 {
        self.panels.get(&self.position).unwrap_or(&PanelColor::Black).value()
    }

    /// Output that moves the robot.  Toggles between two states when called - the first call paints
    /// the square under the robot, the second call turns the robot left or right.
    fn output(&mut self, value: i64) {
        match self.next_input {
            NextRobotInput::Paint => self.paint(value),
            NextRobotInput::Turn => self.turn(value),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut computer = Computer::from_file("input.txt")?;
    let mut robot = RobotState::new();

    computer.run(&mut robot);

    println!("Part 1: {}", robot.num_squares_painted());

    Ok(())
}
