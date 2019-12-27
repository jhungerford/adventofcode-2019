use computer::*;
use std::collections::HashMap;
use std::fmt;

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

impl fmt::Display for PanelColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PanelColor::Black => write!(f, "."),
            PanelColor::White => write!(f, "#"),
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
    x: i32,
    y: i32,
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
    fn paint(&mut self, color: PanelColor) {
        self.panels.insert(self.position.clone(), color);
    }

    /// Turns the given direction, then moves forward exactly one panel
    fn turn(&mut self, direction: TurnDirection) {
        self.direction = self.direction.turn(direction);
        self.position = self.position.forward(&self.direction);
    }

    /// Returns the color of the panel at the given position.
    fn color(&self, position: &Position) -> &PanelColor {
        self.panels.get(position).unwrap_or(&PanelColor::Black)
    }

    /// Returns the number of squares that this robot painted at least once.
    fn num_squares_painted(&self) -> usize {
        self.panels.len()
    }

    /// Prints a rectangle containing the squares that the robot has painted.
    fn print(&self) {
        // Figure out how large the printed rectangle should be.  y is positive in the down direction.
        let (left, right, top, bottom) = self.panels.keys()
            .fold((0, 0, 0, 0), |(bound_left, bound_right, bound_top, bound_bottom), panel| (
                if panel.x < bound_left { panel.x } else { bound_left },
                if panel.x > bound_right { panel.x } else { bound_right },
                if panel.y < bound_top { panel.y } else { bound_top },
                if panel.y > bound_bottom { panel.y } else { bound_bottom },
            ));

        for y in top ..= bottom {
            for x in left ..= right {
                print!("{}", self.color(&Position {x, y}));
            }
            println!();
        }
    }
}


impl ProgramIO for RobotState {
    /// Input that reads the color of the panel under the robot.
    fn input(&mut self) -> i64 {
        self.color(&self.position).value()
    }

    /// Output that moves the robot.  Toggles between two states when called - the first call paints
    /// the square under the robot, the second call turns the robot left or right.
    fn output(&mut self, value: i64) {
        match self.next_input {
            NextRobotInput::Paint => {
                self.paint(PanelColor::from(value));
                self.next_input = NextRobotInput::Turn;

            },
            NextRobotInput::Turn => {
                self.turn(TurnDirection::from(value));
                self.next_input = NextRobotInput::Paint;
            },
        }
    }
}

fn main() -> std::io::Result<()> {
    // Part 1: how many squares did the robot paint at least once?
    let mut part1_computer = Computer::from_file("input.txt")?;
    let mut part1_robot = RobotState::new();

    part1_computer.run(&mut part1_robot);

    println!("Part 1: {}", part1_robot.num_squares_painted());

    // Part 2: what does the robot print when it starts on a white square?
    let mut part2_computer = Computer::from_file("input.txt")?;
    let mut part2_robot = RobotState::new();

    part2_robot.paint(PanelColor::White);
    part2_computer.run(&mut part2_robot);
    part2_robot.print();

    Ok(())
}
