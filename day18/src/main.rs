use day18::Map;

fn main() {
    let map = Map::load("input.txt");

    // 4268 is too high.
    println!("Part 1: {}", map.all_keys_steps());
}
