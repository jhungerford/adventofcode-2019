use day24::{first_repeat, Grid};

fn main() {
    let mut layout = Grid::load("input.txt");

    println!("Part 1: {}", first_repeat(&mut layout))
}
