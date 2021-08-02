// Cards: 0..10006 (ordered - 0 on top, 10006 on bottom.)
// Shuffling techniques:
// - Deal into new stack - inverts cards into a new deck.
// - Cut N - takes the top N cards from the top of the deck, and moves them to the bottom of the deck retaining their order.
//           N can be negative, which cuts the bottom N cards to the top of the deck.
// - Increment N - places each card every N slots.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<usize>
}

impl Deck {
    /// Builds a new deck with the given number of cards.
    pub fn new(size: usize) -> Deck {
        Deck {
            cards: (0..size).collect()
        }
    }

    /// Returns the position of the given card.
    pub fn position(&self, card: usize) -> usize {
        self.cards.iter().position(|&c| c == card).unwrap()
    }

    /// Performs the given shuffle on this deck, modifying it.
    pub fn shuffle(&mut self, shuffle: Shuffle) {
        let num_cards = self.cards.len();

        match shuffle {
            // Deal reverses the deck.
            Shuffle::Deal => self.cards.reverse(),
            // Cut amount takes n cards from the top or bottom (if n is negative) of the deck,
            // and moves them to the opposite side.
            Shuffle::Cut(n) => {
                // TODO: extend copys values - is there a faster way to do this?
                let mut new_cards = Vec::with_capacity(self.cards.len());
                if n > 0 {
                    new_cards.extend_from_slice(&self.cards[n as usize..]);
                    new_cards.extend_from_slice(&self.cards[0..n as usize]);
                } else {
                    let split = (num_cards as isize + n) as usize;
                    new_cards.extend_from_slice(&self.cards[split..]);
                    new_cards.extend_from_slice(&self.cards[0..split]);
                }

                self.cards = new_cards;
            },
            // Increment lays out the cards, skipping n cards each time.
            Shuffle::Increment(n) => {
                let mut new_cards = vec![0; num_cards];
                for (index, card) in self.cards.iter().enumerate() {
                    new_cards[(index * n) % num_cards] = *card;
                }

                self.cards = new_cards;
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Shuffle {
    Deal,
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
            return Ok(Shuffle::Deal);
        }

        // deal with increment 64
        if s.starts_with("deal with increment ") {
            let amount = s[20..].parse::<usize>().unwrap();
            return Ok(Shuffle::Increment(amount));
        }

        // cut 1004
        if s.starts_with("cut ") {
            let amount = s[4..].parse::<isize>().unwrap();
            return Ok(Shuffle::Cut(amount));
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Deal);

        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], deck.cards);
    }

    #[test]
    fn test_cut() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Cut(3));

        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], deck.cards);
    }

    #[test]
    fn test_negative_cut() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Cut(-4));

        assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], deck.cards);
    }

    #[test]
    fn test_increment() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Increment(3));

        assert_eq!(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3], deck.cards);
    }

    #[test]
    fn sample1() {
        let mut deck = Deck::new(10);
        for line in vec![
            "deal with increment 7",
            "deal into new stack",
            "deal into new stack",
        ] {
            deck.shuffle(line.parse().unwrap());
        }

        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], deck.cards);
    }

    #[test]
    fn sample2() {
        let mut deck = Deck::new(10);
        for line in vec![
            "cut 6",
            "deal with increment 7",
            "deal into new stack",
        ] {
            deck.shuffle(line.parse().unwrap());
        }

        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], deck.cards);
    }

    #[test]
    fn sample3() {
        let mut deck = Deck::new(10);
        for line in vec![
            "deal with increment 7",
            "deal with increment 9",
            "cut -2",
        ] {
            deck.shuffle(line.parse().unwrap());
        }

        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], deck.cards);
    }

    #[test]
    fn sample4() {
        let mut deck = Deck::new(10);
        for line in vec![
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
        ] {
            deck.shuffle(line.parse().unwrap());
        }

        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], deck.cards);
    }
}
