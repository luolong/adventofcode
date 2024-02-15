mod common;

use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::str::FromStr;

use anyhow::{Context, Error, Result};
use itertools::Itertools;

const DEFAULT_FILENAME: &str = "day7.txt";

const CARDS: &str = "23456789TJQKA";

#[derive(Debug, Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq)]
struct Card(u8);

impl Card {
    fn is_joker(&self) -> bool {
        self.0 == 11
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = CARDS
            .chars()
            .nth((self.0 as usize) - 2)
            .ok_or(std::fmt::Error)?;

        write!(f, "{label}")
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let i = CARDS
            .find(s)
            .with_context(|| format!("Parsing a card from \"{s}\""))?;
        Ok(Card((i + 2) as u8))
    }
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        let i = CARDS
            .find(c)
            .with_context(|| format!("Parsing a card from '{c}'"))?;
        Ok(Card((i + 2) as u8))
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn with_jokers(&self, joker_count: u32) -> Result<HandType> {
        match (joker_count, self) {
            (0, _) => Ok(*self),

            // single joker in hand
            (1, Self::HighCard) => Ok(HandType::OnePair),
            (1, Self::OnePair) => Ok(HandType::ThreeOfAKind),
            (1, Self::TwoPair) => Ok(HandType::FullHouse),
            (1, Self::ThreeOfAKind) => Ok(HandType::FourOfAKind),
            (1, Self::FourOfAKind) => Ok(HandType::FiveOfAKind),

            // 2 jokers (that means one pair is made of jokers)
            (2, Self::OnePair) => Ok(HandType::ThreeOfAKind),
            (2, Self::TwoPair) => Ok(HandType::FourOfAKind),
            (2, Self::FullHouse) => Ok(HandType::FiveOfAKind),

            // 3 jokers (that means one triple is jokers)
            (3, Self::ThreeOfAKind) => Ok(HandType::FourOfAKind),
            (3, Self::FullHouse) => Ok(HandType::FiveOfAKind),

            // 4 jokers
            (4, Self::FourOfAKind) => Ok(HandType::FiveOfAKind),

            // 5 jokers
            (5, Self::FiveOfAKind) => Ok(*self),

            // Anything else is impossible
            _ => Err(Error::msg("Too many jokers")),
        }
    }
}

impl Display for HandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            HandType::HighCard => "High card",
            HandType::OnePair => "One pair",
            HandType::TwoPair => "Two pair",
            HandType::ThreeOfAKind => "Three of a kind",
            HandType::FullHouse => "Full house",
            HandType::FourOfAKind => "Four of a kind",
            HandType::FiveOfAKind => "Five of a kind",
        };
        write!(f, "{name}")
    }
}

impl From<&[Card; 5]> for HandType {
    fn from(cards: &[Card; 5]) -> Self {
        let mut cards = &cards[..];
        let mut score = 0;
        while let Some((a, tail)) = cards.split_first() {
            cards = tail;
            let matches = tail.iter().filter(|&b| a == b).count();
            if matches > 0 {
                score += matches + 1;
            }
        }

        match score {
            0 => HandType::HighCard,
            2 => HandType::OnePair,
            4 => HandType::TwoPair,
            5 => HandType::ThreeOfAKind,
            7 => HandType::FullHouse,
            9 => HandType::FourOfAKind,
            14 => HandType::FiveOfAKind,
            _ => panic!("Impossible score: {score}"),
        }
    }
}

impl FromIterator<Card> for [Card; 5] {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut cards = [Default::default(); 5];
        for (i, card) in iter.into_iter().enumerate() {
            cards[i] = card;
        }
        cards
    }
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
struct Hand {
    t: HandType,
    cards: [Card; 5],
    values: [u32; 5],
}

impl Hand {
    fn with_jokers(&self) -> Result<Hand> {
        let mut values = self.values.clone();
        let cards = self.cards;

        let mut joker_count = 0u32;
        for i in 0..5 {
            if self.cards[i].is_joker() {
                values[i] = 0;
                joker_count += 1;
            }
        }

        let t = self.t.with_jokers(joker_count)?;
        let hand = Hand { t, cards, values };
        if cfg!(debug_assertions) {
            if joker_count > 0 {
                eprintln!("  With {joker_count} jokers, {self} becomes {hand}")
            }
        }
        Ok(hand)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Hand {
            t,
            cards,
            values: _,
        } = self;
        write!(f, "{t}: ")?;
        for card in cards {
            write!(f, "{card} ")?;
        }
        Ok(())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.t
            .cmp(&other.t)
            .then_with(|| self.values.cmp(&other.values))
    }
}

impl From<[Card; 5]> for Hand {
    fn from(cards: [Card; 5]) -> Self {
        let t = HandType::from(&cards);
        let mut values = [0; 5];
        cards.iter().enumerate().for_each(|(i, c)| {
            values[i] = c.0 as u32;
        });
        Hand { t, cards, values }
    }
}

fn main() -> Result<()> {
    let reader = common::get_reader(DEFAULT_FILENAME)?;

    let mut hands: Vec<(Hand, u32)> = Vec::new();
    let mut joker_hands: Vec<(Hand, u32)> = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        let (hand, bid) = line
            .split_once(" ")
            .with_context(|| "Parsing hands and bids from: {line}")?;

        let bid: u32 = bid.parse().context("Parsing a bid")?;
        let cards: [Card; 5] = hand.chars().map(|c| c.try_into()).take(5).try_collect()?;
        let hand = Hand::from(cards);

        hand.with_jokers()
            .map(|it| joker_hands.push((it, bid)))
            .context("Parsing hand with jokers")?;

        hands.push((hand, bid));
    }

    hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    if cfg!(debug_assertions) {
        eprintln!("Part1 (after sort)");
        hands
            .iter()
            .for_each(|(hand, bid)| eprintln!("{hand}, bif {bid}"));
        eprintln!();
    }

    let part1: u32 = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| bid * ((rank as u32) + 1))
        .sum();

    joker_hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    if cfg!(debug_assertions) {
        eprintln!("Part2 (after sort)");
        joker_hands
            .iter()
            .for_each(|(hand, bid)| eprintln!("{hand}, bif {bid}"));
        eprintln!();
    }
    let part2: u32 = joker_hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| bid * ((rank as u32) + 1))
        .sum();

    println!("Day 7, Part 1: {part1}");
    println!("Day 7, Part 2: {part2}");

    Ok(())
}
