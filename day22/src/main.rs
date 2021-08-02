use day22::{Deck, load_shuffles};

fn main() {
    let mut deck = Deck::new(10007);

    load_shuffles("input.txt").iter().for_each(|shuffle| {
        deck.shuffle(*shuffle);
    });

    println!("Part 1: {}", deck.position(2019));
}
