use day18::Map;

fn main() {
    println!("Part 1: {}", Map::load("input.txt").all_keys_steps());
    println!("Part 2: {}", Map::load("input2.txt").all_keys_steps());
}
