use day20::Maze;

fn main() {
    let maze = Maze::load("input.txt");

    println!("Part 1: {}", maze.shortest_path());
    println!("Part 2: {}", maze.shortest_path_recursive());
}
