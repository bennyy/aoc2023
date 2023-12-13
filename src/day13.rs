use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day13, i64);
default_new_ctor!(Day13);

#[derive(Debug, Clone)]
struct Dummy {
}

impl Day13 {
    fn parse_data(input_str: String) -> Vec<Dummy> {
        let mut return_vec: Vec<Dummy> = Vec::new();
        let mut lines_str = input_str.split('\n');

        return_vec
    }
}

impl AdventOfCode for Day13 {
    fn day_str(&self) -> String {
        "day13".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let data = Day13::parse_data(input_str);

        self.puzzle1_result = 0;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let data = Day13::parse_data(input_str);

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

    puzzle1_test!(Day13, "0", "0");
    puzzle2_test!(Day13, "0", "0");
}
