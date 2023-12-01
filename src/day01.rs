use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
default_aoc_struct!(Day01, i32);
default_new_ctor!(Day01);

impl Day01 {}

impl AdventOfCode for Day01 {
    fn day_str(&self) -> String {
        "day01".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let calibration_values_inputs: Vec<&str> = input_str.split('\n').collect();
        let mut calibration_values: Vec<i32> = Vec::new();

        for calibration_values_input in calibration_values_inputs.iter() {
            let v: Vec<char> = calibration_values_input
                .chars()
                .filter(|&c| c.is_ascii_digit())
                .collect();

            // Push the number into the vector.
            let first_digit: i32 = v.first().unwrap().to_digit(10).unwrap() as i32;
            let last_digit: i32 = v.last().unwrap().to_digit(10).unwrap() as i32;
            calibration_values.push((first_digit * 10) + last_digit);
        }

        self.puzzle1_result = calibration_values.iter().sum();
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let calibration_values_inputs: Vec<&str> = input_str.split('\n').collect();
        let mut calibration_values: Vec<i32> = Vec::new();

        let replacements = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        for calibration_values_input in calibration_values_inputs.iter() {
            let mut calibration_values_positions: Vec<(u32, i32)> = Vec::new();

            // Find all occurences of each "number-string" in the word.
            replacements.iter().for_each(|(from, to)| {
                if calibration_values_input.contains(from) {
                    calibration_values_input
                        .match_indices(from)
                        .for_each(|(position, _)| {
                            calibration_values_positions.push((position as u32, *to));
                        });
                }
            });

            // Find all occurences of each number in the word.
            calibration_values_input
                .chars()
                .enumerate()
                .for_each(|(i, c)| {
                    if c.is_ascii_digit() {
                        calibration_values_positions
                            .push((i as u32, c.to_digit(10).unwrap() as i32))
                    }
                });

            // First argument is the position of that number in the string. So sort it in
            // that order, then we can find the first and the last number
            calibration_values_positions.sort_by(|left, right| left.0.cmp(&right.0));

            // Push the number into the vector.
            let first_digit = calibration_values_positions.first().unwrap().1;
            let last_digit = calibration_values_positions.last().unwrap().1;
            calibration_values.push((first_digit * 10) + last_digit);
        }

        self.puzzle2_result = calibration_values.iter().sum();
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

    puzzle1_test!(Day01, "142", "56049");
    puzzle2_test_extra!(Day01, "281", "54530");
}
