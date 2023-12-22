use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day14, i64);
default_new_ctor!(Day14);

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    c: char,
}

#[derive(Debug, Clone)]
struct Dummy {
    map: Vec<Point>,
    width: usize,
    height: usize,
}

impl Day14 {
    fn parse_data(input_str: String) -> Dummy {
        let mut map: Vec<Point> = Vec::new();
        let lines_str = input_str.split('\n');

        let mut width = 0;
        let mut y: i64 = 0;
        for line_chunk in lines_str {
            width = std::cmp::max(width, line_chunk.len());
            line_chunk.chars().enumerate().for_each(|(i, c)| {
                let i = i as i64;
                match c {
                    'O' => map.push(Point { x: i, y, c: 'O' }),
                    '#' => map.push(Point { x: i, y, c: '#' }),
                    '.' => (),
                    _ => panic!("Unknown char {:?}", c),
                };
            });
            y += 1
        }

        let height = y as usize;

        Dummy { map, width, height }
    }

    fn increment_duplicates(vec: &mut [(i64, i64)]) {
        let mut counts = std::collections::HashMap::new();

        for tuple in vec.iter_mut() {
            let count = counts.entry(*tuple).or_insert(0);
            *count += 1;

            if *count > 1 {
                tuple.1 = tuple.1 + *count - 1;
            }
        }
    }
}

impl AdventOfCode for Day14 {
    fn day_str(&self) -> String {
        "day14".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let data = Day14::parse_data(input_str);
        let _width = data.width;

        let mut new_stone_pos: Vec<(i64, i64)> = Vec::new();

        for move_stone in data.map.clone() {
            if move_stone.c != 'O' {
                continue;
            }

            let nearest_stone = data
                .map
                .iter()
                .filter(|point| point.c == '#' && point.y < move_stone.y && point.x == move_stone.x)
                .max_by(|a, b| a.y.cmp(&b.y));

            if nearest_stone.is_none() {
                new_stone_pos.push((move_stone.x, 0));
            } else {
                let nearest_stone = nearest_stone.unwrap();
                new_stone_pos.push((move_stone.x, nearest_stone.y + 1));
            }
        }
        Day14::increment_duplicates(&mut new_stone_pos);

        let mut result = 0;
        for (_, y) in new_stone_pos {
            let weight = data.height as i64 - y;
            result += weight;
        }

        self.puzzle1_result = result;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let _data = Day14::parse_data(input_str);

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

    puzzle1_test!(Day14, "136", "106648");
    puzzle2_test!(Day14, "0", "0");
}
