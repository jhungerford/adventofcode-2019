// Cards: 0..10006 (ordered - 0 on top, 10006 on bottom.)
// Shuffling techniques:
// - Deal into new stack - inverts cards into a new deck.
// - Cut N - takes the top N cards from the top of the deck, and moves them to the bottom of the deck retaining their order.
//           N can be negative, which cuts the bottom N cards to the top of the deck.
// - Increment N - places each card every N slots.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use mod_exp::mod_exp;

#[derive(Debug, Copy, Clone)]
pub enum Shuffle {
    Reverse,
    Cut(isize),
    Increment(usize),
}

#[derive(Debug)]
pub struct ParseErr {
    message: String,
}

impl FromStr for Shuffle {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // deal into new stack
        if s == "deal into new stack" {
            return Ok(Shuffle::Reverse);
        }

        // cut 1004
        if s.starts_with("cut ") {
            let amount = s[4..].parse::<isize>().unwrap();
            return Ok(Shuffle::Cut(amount));
        }

        // deal with increment 64
        if s.starts_with("deal with increment ") {
            let amount = s[20..].parse::<usize>().unwrap();
            return Ok(Shuffle::Increment(amount));
        }

        Err(ParseErr{ message: format!("Invalid shuffle '{}'", s) })
    }
}

/// Loads a list of shuffles out of the given file.
pub fn load_shuffles(filename: &str) -> Vec<Shuffle> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

/// Shuffle that composes the operations using modular arithmetic.
/// See https://codeforces.com/blog/entry/72593
#[derive(Debug, Copy, Clone)]
pub struct ModShuffle {
    a: i128,
    b: i128,
    m: i128,
}

impl Add for ModShuffle {
    type Output = ModShuffle;

    fn add(self, rhs: Self) -> Self::Output {
        let m = rhs.m;
        if self.m != m {
            panic!("Can't add ModShuffles with different moduli")
        }

        ModShuffle {
            a: modulo(self.a * rhs.a, m),
            b: modulo(self.b * rhs.a + rhs.b, m),
            m
        }
    }
}

impl Sum for ModShuffle {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap()
    }
}

impl ModShuffle {
    fn new(a: i128, b: i128, m: i128) -> ModShuffle {
        ModShuffle { a, b, m }
    }

    /// Creates a new ModShuffle that will perform all of the shuffles once on the given number of cards.
    pub fn from(shuffles: &Vec<Shuffle>, num_cards: i128) -> Self {
        shuffles.iter().map(|shuffle| match shuffle {
            Shuffle::Reverse => ModShuffle::new(-1, -1, num_cards),
            Shuffle::Cut(n) => ModShuffle::new(1, -*n as i128, num_cards),
            Shuffle::Increment(n) => ModShuffle::new(*n as i128, 0, num_cards),
        }).sum()
    }

    /// Returns the position of the given card after running the shuffle once.
    pub fn position(&self, card: usize) -> usize {
        modulo(self.a * card as i128 + self.b, self.m) as usize

    }

    /// Returns the number on the card that ends up in the given position after repeating
    /// the shuffle for the given number of rounds.
    pub fn card(&self, position: usize, rounds: i128) -> usize {

        // Applying the shuffles n times forms a geometic series,
        // which simplifies to a^n * x + b * (1 - a^n) / (1 - a) mod m.
        // Inverting the expression, we get f^-n(x) = x - B / A

        let exp_a = mod_exp(self.a, rounds, self.m);
        let exp_b = modulo(modulo(self.b * (1 - exp_a), self.m) * inverse(modulo(1 - self.a, self.m), self.m), self.m);

        let result = (position as i128 - exp_b) * inverse(exp_a, self.m);

        modulo(result, self.m) as usize
    }

    /// Returns a full deck after running this shuffle once.
    pub fn deck(&self) -> Vec<usize> {
        let mut cards: Vec<usize> = vec![0; self.m as usize];

        for card in 0..self.m as usize {
            cards[self.position(card)] = card;
        }

        cards
    }
}

/// Returns n mod m.  `n % m` computes the _remainder_, where modulo(n, m) computes n - ⌊ n / m ⌋.
/// Modulo will always be positive, where the remainder may be negative.
fn modulo(n: i128, m: i128) -> i128 {
    n.rem_euclid(m)
}

fn inverse(a: i128, m: i128) -> i128 {
    mod_exp(a, m-2, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal() {
        let expected = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let shuffles = vec![Shuffle::Reverse];

        let mod_shuffle = ModShuffle::from(&shuffles, expected.len() as i128);
        assert_eq!(expected, mod_shuffle.deck());

        let cards = (0..expected.len()).map(|i| mod_shuffle.card(i, 1)).collect::<Vec<usize>>();
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_cut() {
        let expected = vec![3, 4, 5, 6, 7, 8, 9, 10, 0, 1, 2];
        let shuffles = vec![Shuffle::Cut(3)];

        let mod_shuffle = ModShuffle::from(&shuffles, expected.len() as i128);
        assert_eq!(expected, mod_shuffle.deck());

        let cards = (0..expected.len()).map(|i| mod_shuffle.card(i, 1)).collect::<Vec<usize>>();
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_negative_cut() {
        let expected = vec![7, 8, 9, 10, 0, 1, 2, 3, 4, 5, 6];
        let shuffles = vec![Shuffle::Cut(-4)];

        let mod_shuffle = ModShuffle::from(&shuffles, expected.len() as i128);
        assert_eq!(expected, mod_shuffle.deck());

        let cards = (0..expected.len()).map(|i| mod_shuffle.card(i, 1)).collect::<Vec<usize>>();
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_increment() {
        let expected = vec![0, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7];

        let shuffles = vec![Shuffle::Increment(3)];

        let mod_shuffle = ModShuffle::from(&shuffles, expected.len() as i128);
        assert_eq!(expected, mod_shuffle.deck());

        let cards = (0..expected.len()).map(|i| mod_shuffle.card(i, 1)).collect::<Vec<usize>>();
        assert_eq!(cards, expected);
    }

    #[test]
    fn sample1() {
        let expected = vec![0, 8, 5, 2, 10, 7, 4, 1, 9, 6, 3];

        let shuffles = vec![
            "deal with increment 7",
            "deal into new stack",
            "deal into new stack",
        ].into_iter().map(|line| line.parse().unwrap()).collect::<Vec<Shuffle>>();

        let mod_shuffle = ModShuffle::from(&shuffles, expected.len() as i128);
        assert_eq!(expected, mod_shuffle.deck());

        let cards = (0..expected.len()).map(|i| mod_shuffle.card(i, 1)).collect::<Vec<usize>>();
        assert_eq!(cards, expected);
    }

    #[test]
    fn sample2() {
        let expected = vec![9, 1, 4, 7, 10, 2, 5, 8, 0, 3, 6];

        let shuffles = vec![
            "cut 6",
            "deal with increment 7",
            "deal into new stack",
        ].into_iter().map(|line| line.parse().unwrap()).collect::<Vec<Shuffle>>();

        let mod_shuffle = ModShuffle::from(&shuffles, expected.len() as i128);
        assert_eq!(expected, mod_shuffle.deck());

        let cards = (0..expected.len()).map(|i| mod_shuffle.card(i, 1)).collect::<Vec<usize>>();
        assert_eq!(cards, expected);
    }

    #[test]
    fn sample3() {
        let expected = vec![8, 4, 0, 7, 3, 10, 6, 2, 9, 5, 1];

        let shuffles = vec![
            "deal with increment 7",
            "deal with increment 9",
            "cut -2",
        ].into_iter().map(|line| line.parse().unwrap()).collect::<Vec<Shuffle>>();

        let mod_shuffle = ModShuffle::from(&shuffles, expected.len() as i128);
        assert_eq!(expected, mod_shuffle.deck());

        let cards = (0..expected.len()).map(|i| mod_shuffle.card(i, 1)).collect::<Vec<usize>>();
        assert_eq!(cards, expected);
    }

    #[test]
    fn sample4() {
        let expected = vec![1, 8, 4, 0, 7, 3, 10, 6, 2, 9, 5];

        let shuffles = vec![
            "deal into new stack",
            "cut -2",
            "deal with increment 7",
            "cut 8",
            "cut -4",
            "deal with increment 7",
            "cut 3",
            "deal with increment 9",
            "deal with increment 3",
            "cut -1",
        ].into_iter().map(|line| line.parse().unwrap()).collect::<Vec<Shuffle>>();

        let mod_shuffle = ModShuffle::from(&shuffles, expected.len() as i128);
        assert_eq!(expected, mod_shuffle.deck());

        let cards = (0..expected.len()).map(|i| mod_shuffle.card(i, 1)).collect::<Vec<usize>>();
        assert_eq!(cards, expected);
    }
}
