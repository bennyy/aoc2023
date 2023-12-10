use std::collections::HashSet;

use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day10, i64);
default_new_ctor!(Day10);

#[derive(Debug, Clone)]
struct Node {
    north: bool,
    south: bool,
    west: bool,
    east: bool,
    c: char,
}

struct PipeMap {
    map: Vec<Node>,
    width: usize,
}

impl Day10 {
    fn char_to_node(c: char) -> Node {
        match c {
            '|' => Node {
                north: true,
                south: true,
                west: false,
                east: false,
                c: '│',
            },
            '-' => Node {
                north: false,
                south: false,
                west: true,
                east: true,
                c: '─',
            },
            'L' => Node {
                north: true,
                south: false,
                west: false,
                east: true,
                c: '└',
            },
            'J' => Node {
                north: true,
                south: false,
                west: true,
                east: false,
                c: '┘',
            },
            '7' => Node {
                north: false,
                south: true,
                west: true,
                east: false,
                c: '┐',
            },
            'F' => Node {
                north: false,
                south: true,
                west: false,
                east: true,
                c: '┌',
            },
            '.' => Node {
                north: false,
                south: false,
                west: false,
                east: false,
                c: ' ',
            },
            'S' => Node {
                north: true,
                south: true,
                west: true,
                east: true,
                c: 'X',
            },
            _ => panic!("Unknown char {}", c),
        }
    }

    fn parse_data(input_str: String) -> PipeMap {
        let mut map: Vec<Node> = Vec::new();
        let lines_str = input_str.split('\n');
        let width = lines_str.clone().next().unwrap().chars().count();
        for line in lines_str {
            map.extend(line.chars().map(Day10::char_to_node).collect::<Vec<_>>());
        }

        PipeMap { map, width }
    }
}

impl AdventOfCode for Day10 {
    fn day_str(&self) -> String {
        "day10".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let pipe_map = Day10::parse_data(input_str);
        let width = pipe_map.width as i32;

        // Print map
        // for map in pipe_map.map.chunks(pipe_map.width) {
        //    let s: String = map.iter().map(|x| x.c).collect();
        //    println!("{}", s);
        //}

        let (pos, _node) = pipe_map
            .map
            .iter()
            .enumerate()
            .find(|x| x.1.c == 'X')
            .unwrap();

        let mut current_pos: usize = pos;
        let mut visited_hs: HashSet<usize> = HashSet::new();
        loop {
            let directions: [(i32, i32); 4] = [
                (0, -1), // North
                (0, 1),  // South
                (-1, 0), // West
                (1, 0),  // East
            ];

            let prev_len = visited_hs.len();
            for (dx, dy) in directions.into_iter() {
                let current_index = current_pos as i32;

                // Check out of boundaries
                let x = (current_index % width) + dx;
                let y = (current_index / width) + dy;
                if x < 0 || y < 0 || x >= width || y >= width {
                    continue;
                }

                let next_index = (x as usize) + width as usize * (y as usize);

                // If we already visited this, then it's trying to go backwards or found the end
                if visited_hs.contains(&next_index) {
                    continue;
                }

                let this_node = pipe_map.map.get(current_pos).unwrap();
                let next_node = pipe_map.map.get(next_index).unwrap();
                if ((dx, dy) == (0, -1) && next_node.south && this_node.north)
                    || ((dx, dy) == (0, 1) && next_node.north && this_node.south)
                    || ((dx, dy) == (-1, 0) && next_node.east && this_node.west)
                    || ((dx, dy) == (1, 0) && next_node.west && this_node.east)
                {
                    current_pos = next_index;
                    visited_hs.insert(next_index);
                    break;
                }
            }

            if prev_len == visited_hs.len() {
                // Nothing new have happend, we have reached the end.
                break;
            }
        }

        self.puzzle1_result = visited_hs.len() as i64 / 2;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let _races = Day10::parse_data(input_str);

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

    puzzle1_test!(Day10, "8", "6613");
    puzzle2_test!(Day10, "0", "0");
}
