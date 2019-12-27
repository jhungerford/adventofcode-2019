use std::fmt;
use computer::*;
use std::fmt::{Formatter, Error};
use std::collections::HashMap;

mod computer;

#[derive(Eq, PartialEq)]
enum Tile {
    Empty, Wall, Block, Paddle, Ball,
}

impl Tile {
    fn parse(value: i64) -> Tile {
        use Tile::*;

        match value {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            n => panic!("Invalid tile: {}", n),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use Tile::*;

        match self {
            Empty => write!(f, " "),
            Wall => write!(f, "."),
            Block => write!(f, "#"),
            Paddle => write!(f, "-"),
            Ball => write!(f, "o"),
        }
    }
}

struct Game {
    input: GameInputState,
    tiles: HashMap<Position, Tile>
}

struct GameInputState {
    next_state: NextGameInputState,
    x: Option<i64>,
    y: Option<i64>,
}

enum NextGameInputState {
    X, Y, TileId
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // Figure out how large the printed rectangle should be.  y is positive in the down direction.
        let (left, right, top, bottom) = self.tiles.keys()
            .fold((0, 0, 0, 0), |(bound_left, bound_right, bound_top, bound_bottom), panel| (
                if panel.x < bound_left { panel.x } else { bound_left },
                if panel.x > bound_right { panel.x } else { bound_right },
                if panel.y < bound_top { panel.y } else { bound_top },
                if panel.y > bound_bottom { panel.y } else { bound_bottom },
            ));

        for y in top ..= bottom {
            for x in left ..= right {
                let tile = self.tiles.get(&Position {x, y}).unwrap_or(&Tile::Empty);

                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Game {
    fn new() -> Game {
        Game {
            input: GameInputState {
                next_state: NextGameInputState::X,
                x: None,
                y: None,
            },
            tiles: HashMap::new(),
        }
    }

    fn num_blocks(&self) -> usize {
        self.tiles.values()
            .filter(|&tile| *tile == Tile::Block)
            .count()
    }
}

impl ProgramIO for Game {
    fn input(&mut self) -> i64 {
        unimplemented!()
    }

    fn output(&mut self, value: i64) {
        use NextGameInputState::*;

        // Every 3 outputs are x, y, and tile id
        match self.input.next_state {
            X => {
                self.input.x = Some(value);
                self.input.next_state = Y;
            },
            Y => {
                self.input.y = Some(value);
                self.input.next_state = TileId;
            },
            TileId => {
                let position = Position {
                    x: self.input.x.unwrap(),
                    y: self.input.y.unwrap(),
                };

                self.tiles.insert(position, Tile::parse(value));
                self.input.x = None;
                self.input.y = None;
                self.input.next_state = X;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Part 1: how many block tiles are on the screen when the game exits?
    let mut computer = Computer::from_file("input.txt")?;
    let mut game = Game::new();

    computer.run(&mut game);
    println!("Part 1: {}\n{}", game.num_blocks(), game);

    Ok(())
}
