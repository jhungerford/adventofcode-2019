// Flawed Frequency Transmission (FFT)
// Input: list of numbers - single digits
// Phases: construct new list with the same length as the input list - used as the input for the next phase
// Each element is built by multiplying every value in the input list by a value in a repeating
// pattern and adding up the results.  Only the ones digit is kept.

// Base pattern is 0, 1, 0, -1.  Repeat each value in the pattern a number of times equal to the
// position in the output list - once for the first element, twice for the second, etc.
// Skip the very first value exactly once.

// After 100 phases of FFT, what are the first eight digits in the final output list?

use std::fs::File;
use std::io::{BufReader, BufRead};

/// Loads input from the given file.
pub fn load_input(filename: &str) -> String {
    let f = File::open(filename).unwrap();
    let mut f = BufReader::new(f);

    let mut line = String::new();

    f.read_line(&mut line).unwrap();

    line.to_string()
}

/// Performs a flawed frequency transmission on the input for the given number of phases,
/// and returns the first eight digits of the result.
pub fn fft(input: &str, phases: usize) -> u32 {
    let mut buffer = parse(input);

    for _ in 0..phases {
        buffer = fft_phase(buffer);
    }

    // Format the first 8 digits of the buffer into the result.
    buffer[0..8].iter().enumerate()
        .map(|(i, value)| *value * u32::pow(10, 7-i as u32))
        .sum()
}

/// Parses digits in the given input into a buffer.
fn parse(input: &str) -> Vec<u32> {
    let trimmed = input.trim();
    let mut buf = Vec::with_capacity(trimmed.len());

    for c in trimmed.chars() {
        buf.push(c as u32 - '0' as u32);
    }

    buf
}

/// Calculates the result of running one phase of FFT.
fn fft_phase(input: Vec<u32>) -> Vec<u32> {
    let mut result = Vec::with_capacity(input.len());

    for i in 0..input.len() {
        let digit = input.iter()
            .zip(pattern(i+1))
            .map(|(a, b)| *a as i32 * b)
            .sum::<i32>().abs() as u32 % 10;
        result.push(digit);
    }

    result
}

/// Returns an infinite, repeating pattern that will repeat each value a given number of times.
fn pattern(n: usize) -> Pattern {
    Pattern { n, num: 1 }
}

struct Pattern {
    n: usize,
    num: usize,
}

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = BASE_PATTERN[(self.num / self.n) % BASE_PATTERN.len()];

        self.num += 1;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {
        assert_eq!(vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0], pattern(1).take(10).collect::<Vec<i32>>());
        assert_eq!(vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1], pattern(2).take(10).collect::<Vec<i32>>());
        assert_eq!(vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1], pattern(3).take(10).collect::<Vec<i32>>());
    }

    #[test]
    fn fft_sample() {
        assert_eq!(48226158, fft("12345678", 1));
        assert_eq!(34040438, fft("12345678", 2));
        assert_eq!(03415518, fft("12345678", 3));
        assert_eq!(01029498, fft("12345678", 4));
    }

    #[test]
    fn fft_larger_samples() {
        assert_eq!(24176176, fft("80871224585914546619083218645595", 100));
        assert_eq!(73745418, fft("19617804207202209144916044189917", 100));
        assert_eq!(52432133, fft("69317163492948606335995924319873", 100));
    }
}