use day22::{load_shuffles, ModShuffle};

fn main() {
    let shuffles = load_shuffles("input.txt");
    let part1_mod_shuffles = ModShuffle::from(&shuffles, 10007);

    println!("Part 1: {}", part1_mod_shuffles.position(2019));

    // Part 2: with a deck of 119315717514047 cards,
    // and applying the shuffle process 101741582076661 times in a row,
    // what number is on the card that ends up in position 2020?
    let part2_mod_shuffles = ModShuffle::from(&shuffles, 119315717514047);
    println!("Part 2: {}", part2_mod_shuffles.card(2020, 101741582076661))
}
