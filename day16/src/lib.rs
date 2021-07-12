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
use std::collections::{HashSet, HashMap};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

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
pub fn fft(input: &str, phases: usize) -> usize {
    let mut buffer = parse(input);

    for _ in 0..phases {
        buffer = fft_phase(buffer);
    }

    format_result(&buffer, 8, 0)
}

/// Decodes the real signal from the given input by repeating the input 10,000 times,
/// running fft for the given number of phases, and returning the 8-digit message at the offset
/// given by the first seven digits of the input.
pub fn fft_real(input: &str, phases: usize) -> usize {
    let offset: usize = input[0..7].parse().unwrap();
    let mut buffer = parse(&input.trim().repeat(10000));
    let len = buffer.len();

    // Since we only care about the 8 digits after the offset, work backwards
    // to figure out which indexes we need to update.
    let mut ns = HashSet::new();
    let mut to_expand = Vec::new();
    (offset..offset + 8).for_each(|expand| to_expand.push(expand));

    while let Some(expand) = to_expand.pop() {
        if !ns.contains(&expand) {
            indexes(expand, buffer.len()).for_each(|(i, _)| {
                if ns.insert(i) {
                    to_expand.push(i);
                }
            })
        }
    }

    let mut ns = ns.into_iter().collect::<Vec<usize>>();
    ns.par_sort();

    // Run FFT.
    for phase in 0..phases {
        println!("Phase {}", phase);

        buffer = fft_phase_indexes(buffer, &ns, len);
    }

    format_result(&buffer, 8, offset)
}

/// Parses digits in the given input into a buffer.
fn parse(input: &str) -> HashMap<usize, u32> {
    let trimmed = input.trim();
    let mut buf = HashMap::with_capacity(trimmed.len());

    for (i, c) in trimmed.chars().enumerate() {
        buf.insert(i, c as u32 - '0' as u32);
    }

    buf
}

/// Takes digits from the buffer starting at the given offset, and formats them into a number.
fn format_result(buffer: &HashMap<usize, u32>, digits: usize, offset: usize) -> usize {
    (0..digits)
        .map(|i| buffer[&(i + offset)] * u32::pow(10, (digits-1-i) as u32))
        .sum::<u32>() as usize
}

/// Calculates the result of running one phase of FFT.
fn fft_phase(input: HashMap<usize, u32>) -> HashMap<usize, u32> {
    let len = input.len();
    let indexes: Vec<usize> = (0..len).collect();
    fft_phase_indexes(input, &indexes, len)
}

/// Calculates the result of running one phase of FFT on the given indexes.
fn fft_phase_indexes(input: HashMap<usize, u32>, ns: &Vec<usize>, len: usize) -> HashMap<usize, u32> {
    let result = ns.into_par_iter().map(|n| {
        let digit = indexes(n+1, len)
            .map(|(i, signum)| input[&i] as i32 * signum)
            .sum::<i32>().abs() as u32 % 10;

        (*n, digit)
    }).collect::<HashMap<usize, u32>>();

    result
}

/// Returns an iterator of indexes and signums that should be summed to compute the value n.
fn indexes(n: usize, len: usize) -> Indexes {
    Indexes {n, len, next_index: n - 1, signum: 1, group_index: 0}
}

struct Indexes {
    n: usize,
    len: usize,
    next_index: usize,
    signum: i32,
    group_index: usize,
}

impl Iterator for Indexes {
    /// Item is the next index, and the sign for that index.
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.len {
            return None;
        }

        // Pattern is 0, 1, 0, -1, where each element in the pattern is repeated n times.
        // First 0 is skipped.
        // n=1: 1, 0, -1, 0, 1, 0, -1, ... - indexes is (0, 1), (2, -1), (4, 1), (6, -1)
        // n=2: 0, 1, 1, 0, 0, -1, -1, 0, 0, ... - indexes is (1, 1), (2, 1), (5, -1), (6, -1)
        // n=3: 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, ...
        //      indexes is (2, 1), (3, 1), (4, 1), (8, -1), (9, -1), (10, -1)

        // index starts at n-1, increments n times, flips signum + advances n+1
        let result = (self.next_index, self.signum);

        self.group_index += 1;
        if self.group_index == self.n {
            self.next_index += self.n + 1;
            self.signum *= -1;
            self.group_index = 0;
        } else {
            self.next_index += 1;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexes() {
        assert_eq!(vec![(0, 1), (2, -1), (4, 1), (6, -1), (8, 1)], indexes(1, 10).collect::<Vec<(usize, i32)>>());
        assert_eq!(vec![(1, 1), (2, 1), (5, -1), (6, -1), (9, 1)], indexes(2, 10).collect::<Vec<(usize, i32)>>());
        assert_eq!(vec![(2, 1), (3, 1), (4, 1), (8, -1), (9, -1)], indexes(3, 10).collect::<Vec<(usize, i32)>>());
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

    #[test]
    fn fft_real_signal() {
        assert_eq!(84462026, fft_real("03036732577212944063491565474664", 100));
        assert_eq!(78725270, fft_real("02935109699940807407585447034323", 100));
        assert_eq!(53553731, fft_real("03081770884921959731165446850517", 100));
    }
}