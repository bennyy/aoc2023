use std::collections::{HashMap, HashSet};

use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day04, i32);
default_new_ctor!(Day04);

#[derive(Debug, Clone)]
struct Card {
    card_id: i32,
    winning_numbers: HashSet<i32>,
    my_numbers: HashSet<i32>,
}

impl Day04 {
    fn parse_data(input_str: String) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();

        for card_str in input_str.split('\n') {
            let mut str = card_str.split(':');

            let id: i32 = str
                .next()
                .unwrap()
                .replace("Card", "")
                .replace(' ', "")
                .parse::<i32>()
                .unwrap();
            let mut numbers = str.next().unwrap().split('|');
            let winning_numbers: HashSet<i32> = numbers
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect();
            let my_numbers: HashSet<i32> = numbers
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect();

            cards.push(Card {
                card_id: id,
                winning_numbers,
                my_numbers,
            });
        }

        cards
    }
}

impl AdventOfCode for Day04 {
    fn day_str(&self) -> String {
        "day04".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let cards = Day04::parse_data(input_str);
        let mut result = 0;
        for card in cards {
            let total_winning_numbers =
                card.my_numbers.intersection(&card.winning_numbers).count() as u32;

            result += match total_winning_numbers.cmp(&1) {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 1,
                std::cmp::Ordering::Greater => 2_i32.pow(total_winning_numbers - 1),
            };
        }
        self.puzzle1_result = result;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let cards = Day04::parse_data(input_str);

        let mut total_scratchcards: HashMap<i32, i32> = HashMap::new();

        for card in &cards {
            // We have one original scratchcard of each.
            total_scratchcards.insert(card.card_id, 1);
        }

        for card in &cards {
            let this_card_id = card.card_id;
            let scratchcards = *total_scratchcards.get(&card.card_id).unwrap();
            let total_winning_numbers =
                card.my_numbers.intersection(&card.winning_numbers).count() as i32;

            for card_id in (this_card_id + 1)..(this_card_id + total_winning_numbers + 1) {
                // Add the scratchcard copies
                *total_scratchcards.get_mut(&card_id).unwrap() += scratchcards;
            }
        }

        self.puzzle2_result = total_scratchcards.values().sum();
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

    puzzle1_test!(Day04, "13", "22488");
    puzzle2_test!(Day04, "30", "7013204");
}
