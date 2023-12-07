use std::{cmp::Ordering, collections::BTreeMap};

use nom::{
    bytes::complete::{self, take_till, take_while1},
    character::complete::space1,
    combinator::{map, map_res},
    sequence::preceded,
    IResult,
};
use tracing::trace;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Joker = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    JokerWeak = 1,
    None = 0,
}

impl Card {
    pub fn from_string(input: &str) -> Self {
        match input {
            "A" => Self::Ace,
            "K" => Self::King,
            "Q" => Self::Queen,
            "J" => Self::Joker,
            "T" => Self::Ten,
            "9" => Self::Nine,
            "8" => Self::Eight,
            "7" => Self::Seven,
            "6" => Self::Six,
            "5" => Self::Five,
            "4" => Self::Four,
            "3" => Self::Three,
            "2" => Self::Two,
            &_ => Self::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Card; 5],
    pub bid: u64,
    pub cheat: bool,
}

impl Hand {
    pub fn cheat(&self) -> [Card; 5] {
        if !self.cards.contains(&Card::Joker) {
            return self.cards;
        }

        let mut m: BTreeMap<Card, usize> = BTreeMap::new();

        for c in self.cards {
            let count = match m.get(&c) {
                Some(c) => c + 1,
                None => 1,
            };

            m.insert(c, count);
        }

        let mut transform = Card::None;
        let mut max_size = 0;
        for (card, size) in m.clone() {
            if card == Card::Joker {
                continue;
            }

            if size < max_size {
                continue;
            }

            if size == max_size && card < transform {
                continue;
            }

            transform = card;
            max_size = size;
        }
        trace!(?transform);

        let mut cards = self.cards;
        for (i, c) in cards.into_iter().enumerate() {
            if c == Card::Joker {
                cards[i] = transform;
            }
        }
        trace!(?cards);

        cards
    }

    pub fn cards_by_cardinality(&self, cheat: bool) -> Vec<usize> {
        let mut m: BTreeMap<Card, usize> = BTreeMap::new();

        let mut cards = self.cards;
        if cheat {
            cards = self.cheat();
        }

        for c in cards {
            let count = match m.get(&c) {
                Some(c) => c + 1,
                None => 1,
            };

            m.insert(c, count);
        }

        m.values().copied().collect::<Vec<usize>>()
    }
}

impl Ord for Hand {
    #[tracing::instrument]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut a = self.cards_by_cardinality(self.cheat);
        a.sort();
        a.reverse();

        let mut o = other.cards_by_cardinality(self.cheat);
        o.sort();
        o.reverse();

        for (a_size, b_size) in a.iter().zip(o) {
            trace!(a_size, b_size);

            let size_comp = a_size.cmp(&b_size);
            if size_comp != Ordering::Equal {
                return size_comp;
            }
        }

        let mut cards = self.cards;
        let mut other_cards = other.cards;
        if self.cheat {
            cards = cards.map(|c| {
                if c == Card::Joker {
                    return Card::JokerWeak;
                }

                c
            });
            trace!(?cards);

            other_cards = other_cards.map(|c| {
                if c == Card::Joker {
                    return Card::JokerWeak;
                }

                c
            });
        }

        let cmps = cards
            .iter()
            .zip(other_cards)
            .map(|(a, b)| a.cmp(&b))
            .collect::<Vec<Ordering>>();

        for c in cmps {
            if c != Ordering::Equal {
                return c;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn from_str(input: &str) -> Result<u64, std::num::ParseIntError> {
    input.parse::<u64>()
}

pub fn is_digit(input: char) -> bool {
    input.is_ascii_digit()
}

pub fn parse_digit(input: &str) -> IResult<&str, u64> {
    let (input, _) = take_till(|c: char| c.is_ascii_digit())(input)?;
    map_res(take_while1(is_digit), from_str)(input)
}

pub fn hand(input: &str, cheat: bool) -> IResult<&str, Hand> {
    let mut input = input;
    let mut cards = [Card::None; 5];
    #[allow(clippy::needless_range_loop)]
    for i in 0..cards.len() {
        let (inp, card) = map(complete::take(1usize), |c: &str| Card::from_string(c))(input)?;
        cards[i] = card;
        input = inp;
    }

    let (input, bid) = preceded(space1, parse_digit)(input)?;

    Ok((input, Hand { cards, bid, cheat }))
}

#[tracing::instrument(skip_all)]
pub fn part_one(inp: Vec<String>) -> u64 {
    let mut hands = inp
        .iter()
        .map(|i| {
            let (_, h) = hand(i, false).unwrap();
            h
        })
        .collect::<Vec<Hand>>();

    hands.sort();
    for hand in hands.iter() {
        let cards = hand.cards;
        trace!(?cards);
    }

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum::<u64>()
}

#[tracing::instrument(skip_all)]
pub fn part_two(inp: Vec<String>) -> u64 {
    let mut hands = inp
        .iter()
        .map(|i| {
            let (_, h) = hand(i, true).unwrap();
            h
        })
        .collect::<Vec<Hand>>();

    hands.sort();
    for hand in hands.iter() {
        let cards = hand.cards;
        trace!(?cards);
    }

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test_log::test]
    pub fn test_hand_cheat() {
        let (_, b) = hand("T55J5 684", false).unwrap();
        let res = b.cheat();

        assert_eq!(
            res,
            [Card::Ten, Card::Five, Card::Five, Card::Five, Card::Five]
        );

        let (_, b) = hand("T55JT 684", false).unwrap();
        let res = b.cheat();

        assert_eq!(
            res,
            [Card::Ten, Card::Five, Card::Five, Card::Ten, Card::Ten]
        );

        let (_, b) = hand("T5JJT 684", false).unwrap();
        let res = b.cheat();

        assert_eq!(
            res,
            [Card::Ten, Card::Five, Card::Ten, Card::Ten, Card::Ten]
        );
    }

    #[test_log::test]
    pub fn test_hand_partial_cmp() {
        let (_, a) = hand("32T3K 765", false).unwrap();
        let (_, b) = hand("T55J5 684", false).unwrap();

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);

        let (_, a) = hand("KK677 28 ", false).unwrap();
        let (_, b) = hand("KTJJT 220", false).unwrap();

        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert_eq!(b.cmp(&a), Ordering::Less);

        let (_, a) = hand("33332 28 ", false).unwrap();
        let (_, b) = hand("2AAAA 220", false).unwrap();

        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert_eq!(b.cmp(&a), Ordering::Less);

        let (_, a) = hand("77888 28 ", false).unwrap();
        let (_, b) = hand("77788 220", false).unwrap();

        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert_eq!(b.cmp(&a), Ordering::Less);

        let (_, a) = hand("77788 220", false).unwrap();
        let (_, b) = hand("77888 28 ", false).unwrap();

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    pub fn test_hand() {
        assert_eq!(
            hand("32T3K 765", false),
            Ok((
                "",
                Hand {
                    cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                    bid: 765,
                    cheat: false,
                }
            )),
        );
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28 ".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 6440);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28 ".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 5905);
    }
}
