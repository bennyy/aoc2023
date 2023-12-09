use std::collections::HashMap;

use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor, math_util::lcm};

default_aoc_struct!(Day08, i64);
default_new_ctor!(Day08);

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
struct Map {
    instructions: String,
    map: HashMap<String, Node>,
}

impl Day08 {
    fn parse_data(input_str: String) -> Map {
        let mut ll = input_str.split("\n\n");
        let instructions = ll.next().unwrap().to_owned();

        let mut map: HashMap<String, Node> = HashMap::new();
        let nodes_str = ll.next().unwrap().split('\n');
        for node in nodes_str {
            let parsable_string = node
                .replace(" = ", ",")
                .replace(['(', ')'], "")
                .replace(", ", ",");
            let mut parsable_string = parsable_string.split(',');
            let this: String = parsable_string.next().unwrap().to_owned();
            let left: String = parsable_string.next().unwrap().to_owned();
            let right: String = parsable_string.next().unwrap().to_owned();

            map.insert(this.to_string(), Node { left, right });
        }
        Map { instructions, map }
    }
}

impl AdventOfCode for Day08 {
    fn day_str(&self) -> String {
        "day08".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let map_s = Day08::parse_data(input_str);

        let mut current_node = "AAA"; // AAA is the starting node
        let mut steps = 0;

        for instruction in map_s.instructions.chars().cycle() {
            if current_node == "ZZZ" {
                // ZZZ is the goal.
                break;
            }

            let key = current_node;
            current_node = match instruction {
                'L' => &map_s.map.get(key).unwrap().left,
                'R' => &map_s.map.get(key).unwrap().right,
                _ => panic!("Unknown char: {}", instruction),
            };
            steps += 1;
        }
        self.puzzle1_result = steps;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let map_s = Day08::parse_data(input_str);
        let starting_nodes: Vec<&String> = map_s
            .map
            .keys()
            .filter(|str| str.chars().nth(2).unwrap() == 'A')
            .collect();

        let mut steps: Vec<i64> = Vec::new();

        for starting_node in starting_nodes {
            let mut step = 0;
            let mut current_node = starting_node;

            for instruction in map_s.instructions.chars().cycle() {
                if current_node.chars().nth(2).unwrap() == 'Z' {
                    break;
                }

                let key = current_node;
                current_node = match instruction {
                    'L' => &map_s.map.get(key).unwrap().left,
                    'R' => &map_s.map.get(key).unwrap().right,
                    _ => panic!("Unknown char: {}", instruction),
                };
                step += 1;
            }
            steps.push(step);
        }

        self.puzzle2_result = lcm(&steps);
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
    use crate::{puzzle1_test, puzzle2_test_extra};

    puzzle1_test!(Day08, "6", "16897");
    puzzle2_test_extra!(Day08, "6", "16563603485021");
}
