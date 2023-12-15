use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day13, i64);
default_new_ctor!(Day13);

#[derive(Debug, Clone)]
struct Dummy {}

impl Day13 {
    fn parse_data(input_str: String) -> Vec<Vec<String>> {
        let mut return_vec: Vec<Vec<String>> = Vec::new();
        let lines_str = input_str.split("\n\n");

        for line_chunk in lines_str {
            let line: Vec<String> = line_chunk.split('\n').map(|s| s.to_owned()).collect();
            return_vec.push(line);
        }

        return_vec
    }

    fn transform_pattern(patterns_input: &[String]) -> Vec<String> {
        // Check if all strings have the same length
        let all_same_length = patterns_input
            .iter()
            .all(|s| s.len() == patterns_input[0].len());

        if all_same_length {
            let result_vector: Vec<String> = (0..patterns_input[0].len())
                .map(|i| {
                    patterns_input
                        .iter()
                        .map(|s| s.chars().nth(i).unwrap())
                        .collect::<String>()
                })
                .collect();
            return result_vector;
        }
        panic!("All strings must have the same length");
    }

    fn calc_point(
        pattern_input: &Vec<String>,
        multiplier: i64,
        ignore_index_a: Option<usize>,
    ) -> Option<(usize, i64)> {
        for i in 0..pattern_input.len() - 1 {
            if pattern_input[i] == pattern_input[i + 1] {
                if ignore_index_a.is_some() && i == ignore_index_a.unwrap() {
                    continue;
                }

                // Split the vector at the position where the two nearby elements are equal
                let (first_part, second_part) = pattern_input.split_at(i + 1);

                let mut first_part = first_part.to_vec();
                let second_part = second_part.to_vec();
                first_part.reverse();

                let all_have_the_same_string =
                    first_part.iter().zip(second_part).all(|(a, b)| *a == *b);
                if all_have_the_same_string {
                    return Some((i, (i as i64 + 1) * multiplier));
                }
            }
        }
        None
    }

    fn fix_smudge(
        patterns_input: &Vec<String>,
        ignore_index_a: Option<usize>,
    ) -> Option<Vec<String>> {
        let mut pattern = patterns_input.to_owned();

        for (i, pattern_row_a) in pattern.clone().iter().enumerate() {
            for (j, pattern_row_b) in pattern.clone().iter().enumerate().skip(i + 1) {
                if i >= j && j == i {
                    // Ignore self and behind the first index.
                    continue;
                }

                if ignore_index_a.is_some() && i == ignore_index_a.unwrap() {
                    // Ignore index from Part 1
                    continue;
                }

                let no_char_diff = pattern_row_a
                    .chars()
                    .enumerate()
                    .zip(pattern_row_b.chars().enumerate())
                    .filter(|((_, a), (_, b))| a != b)
                    .count();

                // Check where only one char differ
                if no_char_diff == 1 {
                    let splitable = (j - i) % 2;
                    // Check if has a "good" midpoint.
                    if splitable == 1 {
                        // Indexes for the middle point
                        let middle = (i as f32 + j as f32) / 2_f32;
                        let m1 = middle.floor() as usize;
                        let m2 = middle.ceil() as usize;

                        // Check if the smudge is in the "mirror-line"
                        // Or the middle point is a reflection
                        if (j).abs_diff(i) == 1 || pattern[m1] == pattern[m2] {
                            let is_edge = i == 0 || (j == pattern.len() - 1);
                            // Seems that edges we can't see is a reflection(??)
                            // or sanity check that the lines parents is the same.
                            if is_edge || pattern[i - 1] == pattern[j + 1] {
                                // That one with fewest '#' should be replaced with the one with most '#'
                                let a_count = pattern_row_a.chars().filter(|c| *c == '#').count();
                                let b_count = pattern_row_b.chars().filter(|c| *c == '#').count();
                                if b_count > a_count {
                                    let tmp_str = pattern[i].to_owned();
                                    *pattern.get_mut(i).unwrap() = tmp_str.to_owned();
                                    *pattern.get_mut(j).unwrap() = tmp_str.to_owned();
                                } else {
                                    let tmp_str = pattern[j].to_owned();
                                    *pattern.get_mut(i).unwrap() = tmp_str.to_owned();
                                    *pattern.get_mut(j).unwrap() = tmp_str.to_owned();
                                }
                                return Some(pattern);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

impl AdventOfCode for Day13 {
    fn day_str(&self) -> String {
        "day13".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let data = Day13::parse_data(input_str);

        let mut result = 0;
        for pattern in data.clone().iter() {
            // Horizontal
            let mut res = Day13::calc_point(pattern, 100, None);
            if res.is_none() {
                // Vertical
                let transformed_pattern = Day13::transform_pattern(pattern);
                res = Day13::calc_point(&transformed_pattern, 1, None);
            }

            result += res.unwrap().1;
        }

        self.puzzle1_result = result;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let data = Day13::parse_data(input_str);

        let mut result = 0;

        for pattern in data.iter() {
            let mut vertical = false;
            let mut previous_result = Day13::calc_point(pattern, 1, None);
            if previous_result.is_none() {
                let transformed_pattern = Day13::transform_pattern(pattern);
                previous_result = Day13::calc_point(&transformed_pattern, 1, None);
                vertical = true;
            }

            // Get the index, which we should avoid when searching for smudge and new pattern.
            let (i, _) = previous_result.unwrap();

            let point;
            if vertical {
                let transformed_pattern = Day13::transform_pattern(pattern);
                if let Some(tp_smudge_fix) = Day13::fix_smudge(&transformed_pattern, Some(i)) {
                    // The Vertical had some smudges.
                    let multiplier = 1;
                    point = Day13::calc_point(&tp_smudge_fix, multiplier, Some(i));
                } else {
                    // Not Vertical efter smudge fixes, so it must be Horizontal
                    let multiplier = 100;
                    let p_smudge_fix = Day13::fix_smudge(pattern, None).unwrap();
                    point = Day13::calc_point(&p_smudge_fix, multiplier, None);
                }
            } else if let Some(p_smudge_fix) = Day13::fix_smudge(pattern, Some(i)) {
                // The Horizontal had some smudges.
                let multiplier = 100;
                point = Day13::calc_point(&p_smudge_fix, multiplier, Some(i));
            } else {
                // Not Horizontal efter smudge fixes, so it must be Vertical
                let multiplier = 1;
                let transformed_pattern = Day13::transform_pattern(pattern);
                let tp_smudge_fix = Day13::fix_smudge(&transformed_pattern, None).unwrap();
                point = Day13::calc_point(&tp_smudge_fix, multiplier, None);
            }

            if let Some((_, res)) = point {
                result += res;
            } else {
                panic!("Nothing found. Something is probably wrong :)");
            }
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

    puzzle1_test!(Day13, "405", "31265");
    puzzle2_test!(Day13, "400", "39359");
}
