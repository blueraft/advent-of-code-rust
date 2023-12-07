use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, PartialOrd)]
enum Rank {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
struct RankState {
    rank: Rank,
    hand: Vec<Card>,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(()),
        }
    }
}

impl PartialOrd for RankState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank != other.rank {
            Some(self.rank.cmp(&other.rank))
        } else {
            let mut self_cards = self.hand.iter();
            let mut other_cards = other.hand.iter();
            while let Some(c) = self_cards.next() {
                if let Some(o) = other_cards.next() {
                    if c != o {
                        return Some(c.cmp(o));
                    }
                }
            }
            Some(self.hand.cmp(&other.hand))
        }
    }
}

impl TryFrom<HashMap<Card, u32>> for Rank {
    type Error = ();

    fn try_from(hand: HashMap<Card, u32>) -> Result<Self, Self::Error> {
        let max = hand.values().max();
        match hand.values().max() {
            Some(&5) => Ok(Rank::FiveOfKind),
            Some(&4) => Ok(Rank::FourOfKind),
            Some(&3) if hand.len() == 2 => Ok(Rank::FullHouse),
            Some(&3) => Ok(Rank::ThreeOfKind),
            Some(&2) if hand.len() == 3 => Ok(Rank::TwoPair),
            Some(&2) => Ok(Rank::OnePair),
            Some(&1) => Ok(Rank::HighCard),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut values: Vec<(RankState, u32)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut split_line = line.split_whitespace();
            let hand: Vec<Card> = split_line
                .next()
                .unwrap()
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect();

            let rank: Rank = hand
                .clone()
                .into_iter()
                .fold(HashMap::new(), |mut map, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                })
                .try_into()
                .unwrap();
            let rank_state = RankState { rank, hand };
            let bid: u32 = split_line.next().unwrap().parse().unwrap();
            (rank_state, bid)
        })
        .collect();

    values.sort_by_key(|v| v.0.clone());

    let mut rank_value = (values.len() + 1) as u32;
    let sum = values
        .iter()
        .map(|v| {
            rank_value -= 1;
            v.1 * rank_value
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut values: Vec<(RankState, u32)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut split_line = line.split_whitespace();
            let hand: Vec<Card> = split_line
                .next()
                .unwrap()
                .chars()
                .map(|c| {
                    let card: Card = c.try_into().unwrap();
                    match card {
                        Card::J => Card::Joker,
                        (other) => other,
                    }
                })
                .collect();

            let mut hand_map = hand.clone().into_iter().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });
            let highest_card_excluding_joker = hand_map
                .iter()
                .filter(|(card, _)| *card != &Card::Joker)
                .max_by_key(|(_, &value)| value)
                .map(|(card, _)| card.clone());

            if let Some(card) = highest_card_excluding_joker {
                if let Some(num_jokers) = hand_map.remove(&Card::Joker) {
                    *hand_map.entry(card).or_insert(1) += num_jokers;
                }
            }

            let rank_state = RankState {
                rank: hand_map.try_into().unwrap(),
                hand,
            };
            let bid: u32 = split_line.next().unwrap().parse().unwrap();
            (rank_state, bid)
        })
        .collect();

    values.sort_by_key(|v| v.0.clone());

    let mut rank_value = (values.len() + 1) as u32;
    let sum = values
        .iter()
        .map(|v| {
            rank_value -= 1;
            let val = v.1 * rank_value;
            println!("{rank_value:?} - {:?} - {val:?}", v.1);
            val
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
