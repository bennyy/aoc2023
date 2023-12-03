use std::{collections::HashSet, usize::MAX};

use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day03, i32);
default_new_ctor!(Day03);

const EMPTY: usize = MAX;
const SYMBOL: usize = MAX - 1;
const GEAR: usize = MAX - 2;

#[derive(Debug, Clone)]
struct EngineSchematic {
    part_numbers: Vec<i32>,
    schematic_map: Vec<usize>,
    width: usize,
}

impl Day03 {
    fn parse_data(input_str: String) -> EngineSchematic {
        let schematic: Vec<&str> = input_str.split('\n').collect();
        let mut part_numbers: Vec<i32> = Vec::new();
        let mut schematic_map: Vec<usize> = Vec::new();
        let width = schematic.first().unwrap().len();

        for schematic_str in schematic.iter() {
            // Parse all part-numbers from this row
            let mut tmp_part_numbers: Vec<i32> = schematic_str
                .split(|c: char| !c.is_numeric())
                .map(|x| x.chars().filter(|c| c.is_ascii_digit()).collect())
                .filter(|x: &String| !x.is_empty())
                .map(|x: String| x.parse::<i32>().unwrap_or_default())
                .collect();

            let mut new_number = true;
            let mut index_increase = 0;
            for c in schematic_str.chars() {
                if c.is_ascii_alphanumeric() {
                    schematic_map.push(part_numbers.len() + index_increase);
                    new_number = false;
                } else {
                    if !new_number {
                        new_number = true;
                        index_increase += 1;
                    }
                    if c == '.' {
                        schematic_map.push(EMPTY);
                    } else if c == '*' {
                        schematic_map.push(GEAR);
                    } else if !c.is_alphabetic() {
                        schematic_map.push(SYMBOL);
                    } else {
                        panic!("Should not happen")
                    }
                }
            }

            // If the number is the end of the string, we haven't increased the index. Do this for the assertion.
            if !new_number {
                index_increase += 1;
            }
            assert_eq!(index_increase, tmp_part_numbers.len());

            part_numbers.append(&mut tmp_part_numbers);
        }

        EngineSchematic {
            part_numbers,
            schematic_map,
            width,
        }
    }

    fn find_nearby_parts(index: usize, width: usize, schematic_map: &[usize]) -> HashSet<usize> {
        let directions: Vec<(i32, i32)> = vec![
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ];

        let mut found_part_numbers_indexes_tmp: HashSet<usize> = HashSet::new();

        for (dx, dy) in directions {
            let x = (index % width) as i32 + dx;
            let y = (index / width) as i32 + dy;
            if x < 0 || y < 0 || x >= width as i32 || y >= width as i32 {
                continue;
            }

            let i = (x as usize) + width * (y as usize);
            let part_number_index = *schematic_map.get(i).unwrap();
            if part_number_index != SYMBOL
                && part_number_index != GEAR
                && part_number_index != EMPTY
            {
                found_part_numbers_indexes_tmp.insert(part_number_index);
            }
        }

        found_part_numbers_indexes_tmp
    }
}

impl AdventOfCode for Day03 {
    fn day_str(&self) -> String {
        "day03".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let engine_schematic = Day03::parse_data(input_str);
        let width = engine_schematic.width;
        let schematic_map = engine_schematic.schematic_map;
        let part_numbers = engine_schematic.part_numbers;

        // Get a vector of all indexes where a 'symbol' exists.
        let symbol_indexes: Vec<_> = schematic_map
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == SYMBOL || **x == GEAR)
            .collect();

        // Iterate over all 'symbols' and check if it has some adjecent part numbers.
        // It will save the index of that part number.
        let mut found_part_numbers_indexes: HashSet<usize> = HashSet::new();
        for (index, _) in symbol_indexes {
            let tmp = Day03::find_nearby_parts(index, width, &schematic_map);
            found_part_numbers_indexes.extend(&tmp);
        }
        // Based on the indexes, find the actual part number and sum it.
        self.puzzle1_result = part_numbers
            .iter()
            .enumerate()
            .filter(|(index, _)| found_part_numbers_indexes.contains(index))
            .map(|(_, part_number)| *part_number)
            .sum();
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let engine_schematic = Day03::parse_data(input_str);
        let width = engine_schematic.width;
        let schematic_map = engine_schematic.schematic_map;
        let part_numbers = engine_schematic.part_numbers;

        // Get a vector of all indexes where a 'symbol' exists.
        let gear_indexes: Vec<_> = schematic_map
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == GEAR)
            .collect();

        // Iterate over all 'gears' and check if it has some adjecent part numbers.
        // It will save the index of that part number.
        let mut gear_ratios: Vec<i32> = Vec::new();
        for (index, _) in gear_indexes {
            let found_part_numbers_indexes_tmp: HashSet<usize> =
                Day03::find_nearby_parts(index, width, &schematic_map);

            // "A gear is any * symbol that is adjacent to exactly two part numbers."
            if found_part_numbers_indexes_tmp.len() == 2 {
                let gear_ratio: i32 = part_numbers
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| found_part_numbers_indexes_tmp.contains(index))
                    .map(|(_i, part_number)| *part_number)
                    .product();
                gear_ratios.push(gear_ratio);
            }
        }

        self.puzzle2_result = gear_ratios.iter().sum();
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

    puzzle1_test!(Day03, "4361", "532331");
    puzzle2_test!(Day03, "467835", "82301120");
}
