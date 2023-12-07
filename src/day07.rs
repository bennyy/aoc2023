use std::collections::HashMap;

use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day07, i64);
default_new_ctor!(Day07);

#[derive(Copy, Debug, Clone, Eq, Hash, PartialEq)]
enum Card {
    Joker = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: i64,
}

impl Day07 {
    fn parse_data(input_str: String, joker: bool) -> Vec<Hand> {
        let mut hands: Vec<Hand> = Vec::new();

        let meaning_of_j = if joker { Card::Joker } else { Card::Jack };

        for line in input_str.split('\n') {
            let mut s = line.split_ascii_whitespace();
            let cards: Vec<Card> = s
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    'T' => Card::Ten,
                    'J' => meaning_of_j,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!("Unknown Card"),
                })
                .collect();

            let bid = s.next().unwrap().parse::<i64>().unwrap();

            hands.push(Hand { cards, bid });
        }
        hands
    }

    fn count_cards(hand: &Hand) -> HashMap<Card, i64> {
        let mut count_cards: HashMap<Card, i64> = HashMap::new();
        for card in hand.cards.iter() {
            if !count_cards.contains_key(card) {
                count_cards.insert(*card, 0);
            }
            *count_cards.get_mut(card).unwrap() += 1;
        }
        count_cards
    }

    fn sort_hands(hands: &[Hand]) -> Vec<Hand> {
        let mut hands = hands.to_owned();
        hands.sort_by(|a, b| {
            for (a, b) in a.cards.iter().zip(&b.cards) {
                let left = *a as i64;
                let right = *b as i64;
                if left == right {
                    continue;
                }
                return left.cmp(&right);
            }

            std::cmp::Ordering::Equal
        });

        hands
    }

    fn get_hand_type(hand: &Hand) -> HandType {
        let no_of_cards = Day07::count_cards(hand);
        let no_of_jokers = *no_of_cards.get(&Card::Joker).unwrap_or(&0);

        // If all of them is Jokers
        if no_of_jokers == 5 {
            return HandType::FiveOfAKind;
        }

        // The ones with most of, will "receive" the Joker
        let max_unique_cards = no_of_cards
            .iter()
            .filter(|(card, _x)| **card != Card::Joker)
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap()
            .1
            + no_of_jokers;

        // Remove the Joker from the Card count, since it's already added to the Card it's most of.
        let different_no_of_cards = if no_of_jokers > 0 {
            no_of_cards.values().len() - 1_usize
        } else {
            no_of_cards.values().len()
        };

        if different_no_of_cards == 1 {
            return HandType::FiveOfAKind;
        } else if different_no_of_cards == 2 {
            if max_unique_cards == 4 {
                return HandType::FourOfAKind;
            } else if max_unique_cards == 3 {
                return HandType::FullHouse;
            }
        } else if different_no_of_cards == 3 {
            if max_unique_cards == 3 {
                return HandType::ThreeOfAKind;
            } else if max_unique_cards == 2 {
                return HandType::TwoPair;
            }
        } else if different_no_of_cards == 4 && max_unique_cards == 2 {
            return HandType::OnePair;
        } else if different_no_of_cards == 5 && max_unique_cards == 1 {
            return HandType::HighCard;
        }

        panic!("Can't handle this hand: {:?}", hand);
    }

    fn run(input_str: String, joker: bool) -> i64 {
        let hands = Day07::parse_data(input_str, joker);

        let mut five_of_a_kind: Vec<Hand> = Vec::new();
        let mut four_of_a_kind: Vec<Hand> = Vec::new();
        let mut full_house: Vec<Hand> = Vec::new();
        let mut three_of_a_kind: Vec<Hand> = Vec::new();
        let mut two_pair: Vec<Hand> = Vec::new();
        let mut one_pair: Vec<Hand> = Vec::new();
        let mut high_card: Vec<Hand> = Vec::new();

        for hand in hands.iter() {
            match Day07::get_hand_type(hand) {
                HandType::HighCard => high_card.push(hand.clone()),
                HandType::OnePair => one_pair.push(hand.clone()),
                HandType::TwoPair => two_pair.push(hand.clone()),
                HandType::ThreeOfAKind => three_of_a_kind.push(hand.clone()),
                HandType::FullHouse => full_house.push(hand.clone()),
                HandType::FourOfAKind => four_of_a_kind.push(hand.clone()),
                HandType::FiveOfAKind => five_of_a_kind.push(hand.clone()),
            }
        }

        let hands_vec: Vec<Vec<Hand>> = vec![
            high_card,
            one_pair,
            two_pair,
            three_of_a_kind,
            full_house,
            four_of_a_kind,
            five_of_a_kind,
        ];

        let mut result = 0;
        let mut rank = 1;
        for hands in hands_vec {
            for hand in Day07::sort_hands(&hands) {
                result += hand.bid * rank;
                rank += 1;
            }
        }
        result
    }
}

impl AdventOfCode for Day07 {
    fn day_str(&self) -> String {
        "day07".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        self.puzzle1_result = Day07::run(input_str, false);
    }

    fn run_puzzle2(&mut self, input_str: String) {
        self.puzzle2_result = Day07::run(input_str, true);
    }

    fn get_puzzle1_result(&self) -> Option<String> {
        Some(self.puzzle1_result.to_string())
    }

    fn get_puzzle2_result(&self) -> Option<String> {
        Some(self.puzzle2_result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{puzzle1_test, puzzle2_test};

    puzzle1_test!(Day07, "6440", "251029473");
    puzzle2_test!(Day07, "5905", "251003917");
}
