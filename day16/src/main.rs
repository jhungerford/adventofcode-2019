use day16::{fft, load_input, fft_real};

fn main() {
    let input = load_input("input.txt");

    println!("Part 1: {}", fft(&input, 100));
    println!("Part 2: {}", fft_real(&input, 100));
}
