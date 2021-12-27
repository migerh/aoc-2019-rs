use crate::utils::ParseError;
use mod_exp::mod_exp;
use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum ShuffleStrategy {
    Deal(isize),
    Invert,
    Cut(isize),
}

impl FromStr for ShuffleStrategy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_CUT: Regex = Regex::new(r"^cut (?P<cut>.*)$").unwrap();
            static ref RE_DEAL: Regex = Regex::new(r"^deal with increment (?P<inc>.*)$").unwrap();
            static ref RE_INVERT: Regex = Regex::new(r"^deal into new stack$").unwrap();
        }

        if let Some(c) = RE_CUT
            .captures(s)
            .and_then(|cap| cap.name("cut").map(|v| v.as_str().parse::<isize>())?.ok())
        {
            return Ok(Self::Cut(c));
        }

        if let Some(c) = RE_DEAL
            .captures(s)
            .and_then(|cap| cap.name("inc").map(|v| v.as_str().parse::<isize>())?.ok())
        {
            return Ok(Self::Deal(c));
        }

        if let Some(_) = RE_INVERT.captures(s) {
            return Ok(Self::Invert);
        }

        Err(ParseError::new(&format!("Could not parse input \"{}\"", s)))
    }
}

#[aoc_generator(day22)]
fn input_generator(input: &str) -> Result<Vec<ShuffleStrategy>, ParseError> {
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| ShuffleStrategy::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn shuffle(cards: Vec<u32>, strategies: &Vec<ShuffleStrategy>) -> Vec<u32> {
    let mut cards = cards.into_iter().collect::<VecDeque<_>>();
    for s in strategies {
        cards = match s {
            ShuffleStrategy::Invert => cards.into_iter().rev().collect::<VecDeque<_>>(),
            ShuffleStrategy::Cut(c) => {
                let c = *c;
                let range = if c < 0 { c..0 } else { 0..c };

                for _ in range {
                    if c < 0 {
                        let c = cards.pop_back().unwrap();
                        cards.push_front(c);
                    } else {
                        let c = cards.pop_front().unwrap();
                        cards.push_back(c);
                    }
                }
                cards
            }
            ShuffleStrategy::Deal(d) => {
                let mut new_cards = cards.clone();

                let mut pos = 0;
                let d = *d as usize;
                for c in &cards {
                    new_cards[pos] = *c;
                    pos = (pos + d) % cards.len();
                }

                new_cards
            }
        };
    }

    cards.into_iter().collect::<Vec<_>>()
}

#[aoc(day22, part1)]
fn problem1(strategies: &Vec<ShuffleStrategy>) -> Result<usize, ParseError> {
    let mut deck = vec![0; 10007];

    for i in 0..deck.len() {
        deck[i] = i as u32;
    }

    let shuffled_deck = shuffle(deck, strategies);
    shuffled_deck
        .iter()
        .position(|v| *v == 2019)
        .ok_or(ParseError::new(
            "Could not determine position of element '2019'",
        ))
}

#[aoc(day22, part2)]
fn problem2(shuffle: &Vec<ShuffleStrategy>) -> i128 {
    let len = 119_315_717_514_047;
    let rep = 101_741_582_076_661;

    let (a, b) = shuffle.iter().rev().fold((1, 0), |(a, b), s| {
        let (a_new, b_new) = match s {
            ShuffleStrategy::Invert => (-a, -b - 1),
            ShuffleStrategy::Cut(n) => (a, b + *n as i128),
            ShuffleStrategy::Deal(d) => {
                let d = *d as i128;
                let n = mod_exp(d, len - 2, len);
                (a * n, b * n)
            }
        };
        (a_new % len, b_new % len)
    });

    let result = 2020 * mod_exp(a, rep, len) + b * ((mod_exp(a, rep, len) - 1) * mod_exp(a - 1, len - 2, len) % len);
    result % len
}

#[cfg(test)]
mod test {
    use super::*;

    const DECK: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    const EXAMPLES: [&str; 4] = [
        "deal with increment 7
deal into new stack
deal into new stack",
        "cut 6
deal with increment 7
deal into new stack",
        "deal with increment 7
deal with increment 9
cut -2",
        "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1",
    ];

    const RESULTS: [[u32; 10]; 4] = [
        [0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
        [3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
        [6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
        [9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
    ];

    fn deck() -> Vec<u32> {
        DECK.iter().cloned().collect::<Vec<_>>()
    }

    #[test]
    fn part1_verify_example_1() -> Result<(), ParseError> {
        let input = input_generator(EXAMPLES[0])?;
        Ok(assert_eq!(shuffle(deck(), &input), RESULTS[0]))
    }

    #[test]
    fn part1_verify_example_2() -> Result<(), ParseError> {
        let input = input_generator(EXAMPLES[1])?;
        Ok(assert_eq!(shuffle(deck(), &input), RESULTS[1]))
    }

    #[test]
    fn part1_verify_example_3() -> Result<(), ParseError> {
        let input = input_generator(EXAMPLES[2])?;
        Ok(assert_eq!(shuffle(deck(), &input), RESULTS[2]))
    }

    #[test]
    fn part1_verify_example_4() -> Result<(), ParseError> {
        let input = input_generator(EXAMPLES[3])?;
        Ok(assert_eq!(shuffle(deck(), &input), RESULTS[3]))
    }
}
