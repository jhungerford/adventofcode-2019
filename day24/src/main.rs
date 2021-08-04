use day24::{Grid, RecursiveGrid};

fn main() {
    let part1 = Grid::load("input.txt");
    println!("Part 1: {}", part1.first_repeat());

    let mut part2 = RecursiveGrid::load("input.txt");
    for _ in 0..200 {
        part2.tick();
    }
    println!("Part 2: {}", part2.num_bugs());
}
