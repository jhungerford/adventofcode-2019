use day16::{fft, load_input};

fn main() {
    let input = load_input("input.txt");

    println!("Part 1: {}", fft(&input, 100));
}
