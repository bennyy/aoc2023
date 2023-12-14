use std::collections::VecDeque;

use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day12, i64);
default_new_ctor!(Day12);

#[derive(Debug, Clone)]
struct SpringCondition {
    spring: Vec<char>,
    groups: Vec<i64>,
}

impl Day12 {
    fn parse_data(input_str: String, duplicate_input: i64) -> Vec<SpringCondition> {
        let mut return_vec: Vec<SpringCondition> = Vec::new();
        let lines_str = input_str.split('\n');
        for line in lines_str {
            let mut split = line.split_ascii_whitespace();
            let mut spring: Vec<char> = split.next().unwrap().chars().collect();
            let mut groups: Vec<i64> = split
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect();

            if duplicate_input > 0 {
                let mut spring_tmp: Vec<char> = Vec::new();
                for i in 0..duplicate_input {
                    spring_tmp.extend_from_slice(&spring);
                    if i < duplicate_input - 1 {
                        spring_tmp.push('?');
                    }
                }
                spring = spring_tmp;

                groups = groups
                    .iter()
                    .cloned()
                    .cycle()
                    .take(groups.len() * duplicate_input as usize)
                    .collect();
            }

            return_vec.push(SpringCondition { spring, groups });
        }

        return_vec
    }

    fn do_things(spring_stuff: Vec<char>, queue: VecDeque<i64>, combinations: &mut Vec<String>) {
        let mut queue = queue.clone();
        if queue.is_empty() {
            if spring_stuff.contains(&'#') {
                // If we're done and there is # left. Something have been calculated wrong.
                return;
            }
            combinations.push(spring_stuff.iter().collect());
            return;
        }

        if let Some(number) = queue.pop_front() {
            let window_size = (number) as usize;
            for (i, current) in spring_stuff.windows(window_size).enumerate() {
                let current_index = i;

                let mut spring_copy = spring_stuff.clone();
                let mut prev_char = '.';
                let mut next_char = '.';

                if current_index > 0 {
                    prev_char = spring_copy[current_index - 1];
                }

                if current_index + current.len() < spring_stuff.len() - 1 {
                    next_char = spring_copy[current_index + current.len()];
                }

                if prev_char == '#' {
                    // If we're passing a '#', we're in trouble..
                    break;
                }

                if ((current.iter().all(|&c| c == '?'))
                    || current.iter().all(|&c| c == '#' || c == '?'))
                    && (next_char == '?' || next_char == '.')
                    && (prev_char != '#' && prev_char != 'O')
                {
                    for (i, _c) in current.iter().enumerate() {
                        spring_copy[current_index + i] = 'O';
                    }

                    for i in spring_copy.iter_mut().take(current_index) {
                        // Remove the ? since we're already passing them
                        if *i == '?' {
                            *i = 'X';
                        }
                    }

                    Day12::do_things(spring_copy, queue.clone(), combinations);
                }
            }
        }
    }
}

impl AdventOfCode for Day12 {
    fn day_str(&self) -> String {
        "day12".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let spring_conditions = Day12::parse_data(input_str, 0);
        self.puzzle1_result = 0;
        let mut combinations: Vec<String> = Vec::new();
        for sc in spring_conditions {
            let queue: VecDeque<i64> = sc.groups.into_iter().collect();
            Day12::do_things(sc.spring, queue, &mut combinations);
        }

        self.puzzle1_result = combinations.len() as i64;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let _spring_conditions = Day12::parse_data(input_str, 5);
        // TODO: Fix better algoritm :)
        self.puzzle2_result = 0;
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

    puzzle1_test!(Day12, "21", "7771");
    puzzle2_test!(Day12, "0", "0");
}
