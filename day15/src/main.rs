use day15::{explore_map, oxygen, oxygen_distance, Position, print_map, shortest_path};

fn main() {
    let map = explore_map();
    let oxygen_dist = oxygen_distance(&map);

    print_map(&map, vec![Position::default()]);

    println!("Part 1: {}", shortest_path(&oxygen_dist));
    println!("Part 2: {}", oxygen(&oxygen_dist));
}
