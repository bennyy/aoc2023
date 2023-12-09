use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day09, i64);
default_new_ctor!(Day09);

impl Day09 {
    fn parse_data(input_str: String) -> Vec<Vec<i64>> {
        let mut return_vec: Vec<Vec<i64>> = Vec::new();
        let lines_str = input_str.split('\n');
        lines_str.for_each(|x| {
            return_vec.push(
                x.split_ascii_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            )
        });

        return_vec
    }
}

impl AdventOfCode for Day09 {
    fn day_str(&self) -> String {
        "day09".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let values = Day09::parse_data(input_str);
        let mut result = 0;
        for v in values {
            let mut diffs = vec![v];

            // Run until all is zeros
            while !diffs.last().unwrap().iter().all(|x| *x == 0) {
                let values = diffs.last().unwrap().iter();
                let next_values = diffs.last().unwrap().iter().skip(1);
                let diff: Vec<i64> = values
                    .zip(next_values)
                    .map(|(cur, next)| next - cur)
                    .collect();
                diffs.push(diff);
            }

            let mut last_values = vec![0]; // Zero is always the first number.
            for values in diffs.iter().rev().skip(1) {
                let last_value = last_values.last().unwrap();
                let value = values.last().unwrap();
                last_values.push(value + last_value);
            }

            result += last_values.last().unwrap();
        }
        self.puzzle1_result = result;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let values = Day09::parse_data(input_str);
        let mut result = 0;
        for v in values {
            let mut diffs = vec![v];

            // Run until all is zeros
            while !diffs.last().unwrap().iter().all(|x| *x == 0) {
                let values = diffs.last().unwrap().iter();
                let next_values = diffs.last().unwrap().iter().skip(1);
                let diff: Vec<i64> = values
                    .zip(next_values)
                    .map(|(cur, next)| next - cur)
                    .collect();
                diffs.push(diff);
            }

            let mut last_values = vec![0]; // Zero is always the first number.
            for values in diffs.iter().rev().skip(1) {
                let last_value = last_values.last().unwrap();
                let value = values.first().unwrap();
                last_values.push(value - last_value);
            }

            result += last_values.last().unwrap();
        }
        self.puzzle2_result = result;
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

    puzzle1_test!(Day09, "114", "2008960228");
    puzzle2_test!(Day09, "2", "1097");
}
