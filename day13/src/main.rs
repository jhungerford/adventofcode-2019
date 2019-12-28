extern crate pancurses;

use std::{fmt, thread, time};
use std::collections::HashMap;
use std::fmt::{Error, Formatter};

use pancurses::{cbreak, endwin, initscr, noecho, Window};

use computer::*;

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

    fn char(&self) -> char {
        use Tile::*;

        match self {
            Empty => ' ',
            Wall => '.',
            Block => '#',
            Paddle => '-',
            Ball => 'o',
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.char())
    }
}

struct Game<'a> {
    input: GameInputState,
    tiles: HashMap<Position, Tile>,
    title: &'a str,
    score: i64,
    window: Window,
}

struct GameInputState {
    next_state: NextGameInputState,
    x: Option<i64>,
    y: Option<i64>,
}

enum NextGameInputState {
    X, Y, TileId
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl fmt::Display for Game<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{} - Blocks: {} - Score: {}", self.title, self.num_blocks(), self.score)?;

        // Figure out how large the printed rectangle should be.  y is positive in the down direction.
        let (left, right, top, bottom) = self.bounds();

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

impl Game<'_> {
    fn new(title: &str) -> Game {
        let window = initscr();
        cbreak(); // Disable line buffering - we want arrow keys as soon as they're typed.
        noecho(); // Don't echo input back to the screen.
        window.clear();

        Game {
            input: GameInputState {
                next_state: NextGameInputState::X,
                x: None,
                y: None,
            },
            tiles: HashMap::new(),
            title,
            score: 0,
            window,
        }
    }

    fn num_blocks(&self) -> usize {
        self.tiles.values()
            .filter(|&tile| *tile == Tile::Block)
            .count()
    }

    fn bounds(&self) -> (i64, i64, i64, i64) {
        self.tiles.keys()
            .fold((0, 0, 0, 0), |(bound_left, bound_right, bound_top, bound_bottom), panel| (
                if panel.x < bound_left { panel.x } else { bound_left },
                if panel.x > bound_right { panel.x } else { bound_right },
                if panel.y < bound_top { panel.y } else { bound_top },
                if panel.y > bound_bottom { panel.y } else { bound_bottom },
            ))
    }

    /// Draws the current game state to the window.
    fn render(&self) {
        self.window.mvaddstr(0, 0, format!("{} - Blocks: {} - Score: {}", self.title, self.num_blocks(), self.score));
        self.window.clrtoeol();

        // Figure out how large the printed rectangle should be.  y is positive in the down direction.
        let (left, right, top, bottom) = self.bounds();

        for y in top ..= bottom {
            for x in left ..= right {
                let tile = self.tiles.get(&Position {x, y}).unwrap_or(&Tile::Empty);
                self.window.mvaddch(y as i32 + 1, x as i32, tile.char());
            }
        }

        if bottom >= 20 && right >= 39 {
            thread::sleep(time::Duration::from_millis(5));
        }

        self.window.refresh();
    }
}

impl ProgramIO for Game<'_> {
    fn input(&mut self) -> i64 {
        // Instead of having a human play, keep the paddle under the ball.
        let ball = self.tiles.iter()
            .find_map(|(position, tile)| if tile == &Tile::Ball { Some(position) } else { None })
            .unwrap();

        let paddle = self.tiles.iter()
            .find_map(|(position, tile)| if tile == &Tile::Paddle { Some(position) } else { None })
            .unwrap();

        (ball.x - paddle.x).signum()

    }

    fn output(&mut self, value: i64) {
        use NextGameInputState::*;

        // Every 3 outputs are x, y, and tile id
        // x=-1, y=0 means that the third argument is a score, not a tile.
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

                if position == (Position {x: -1, y: 0}) {
                    self.score = value;
                } else {
                    self.tiles.insert(position, Tile::parse(value));
                }

                self.input.x = None;
                self.input.y = None;
                self.input.next_state = X;
                self.render();
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Part 1: how many block tiles are on the screen when the game exits?
    /*
    let mut computer = Computer::from_file("input.txt")?;
    let mut game = Game::new("Part 1");

    computer.run(&mut game);
    game.window.getch();
    endwin();
    println!("Part 1: {}\n{}", game.num_blocks(), game);
    */

    // Part 2: after inserting a quarter (2 -> memory address 0), what's the score when you win the game?
    let mut computer = Computer::from_file("input.txt")?;
    let mut game = Game::new("Part 2");
    computer.memory.set_addr(0, 2); // Insert quarters.
    computer.run(&mut game);
    game.window.getch();
    endwin();

    if game.num_blocks() == 0 {
        println!("Part 2: {} ", game.score);
    } else {
        println!("Part 2: destroy all of the blocks to get the answer.  Left and right arrows move the paddle.");
    }

    Ok(())
}
