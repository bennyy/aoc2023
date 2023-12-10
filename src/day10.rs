use std::{
    collections::{HashSet, VecDeque},
};

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
    height: usize,
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
                c: '.',
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
        let height = lines_str.clone().count();
        for line in lines_str {
            map.extend(line.chars().map(Day10::char_to_node).collect::<Vec<_>>());
        }

        PipeMap { map, width, height }
    }

    fn is_point_inside_polygon(point: &(i64, i64), vertices: &[(i64, i64)]) -> bool {
        let x = point.0;
        let y = point.1;
        let n = vertices.len();
        let mut inside = false;

        for i in 0..n {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[(i + 1) % n]; // Wrap around to the first vertex for the last edge

            // Check if the point is within the y-bounds of the edge
            let y_bounds_check = (y1 > y) != (y2 > y);

            // Check if the edge is not horizontal
            if y1 != y2 {
                // Check if the point is to the left of the edge
                let x_intersect = x > ((x2 - x1) * (y - y1) / (y2 - y1) + x1);
                if y_bounds_check && x_intersect {
                    inside = !inside;
                }
            }
        }

        inside
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
        let pipe_map = Day10::parse_data(input_str);
        let width = pipe_map.width as i32;
        let height = pipe_map.height as i32;

        let map_length = pipe_map.map.len();

        let (pos, _node) = pipe_map
            .map
            .iter()
            .enumerate()
            .find(|x| x.1.c == 'X')
            .unwrap();

        let mut current_pos: usize = pos;
        let mut visited_hs: Vec<usize> = Vec::new();
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
                if x < 0 || y < 0 || x >= width || y >= height {
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
                    visited_hs.push(next_index);
                    break;
                }
            }

            if prev_len == visited_hs.len() {
                // Nothing new have happend, we have reached the end.
                break;
            }
        }

        // Flood fill

        let visited_clone = visited_hs.clone();
        let mut vertices = Vec::new();
        for vhs in visited_hs {
            let x = vhs % width as usize;
            let y = vhs / width as usize;
            vertices.push((x as i64, y as i64));
        }

        let mut points = Vec::new();
        for asdf in 0..pipe_map.map.len() {
            let x = asdf % width as usize;
            let y = asdf / width as usize;
            points.push((x as i64, y as i64));
        }

        let mut new_stuff_thing = vec!['.'; map_length];
        let points_inside_polygon: Vec<&(i64, i64)> = points
            .iter()
            .filter(|&point| Day10::is_point_inside_polygon(point, &vertices))
            .collect();

        for (x, y) in points_inside_polygon {
            let next_index = (*x as usize) + width as usize * (*y as usize);
            new_stuff_thing[next_index] = 'X';
        }

        let mut queue = VecDeque::new();
        let mut visited = vec![false; map_length];
        let mut things_that_is_o = vec![false; map_length];

        for x in 0..width {
            queue.push_back((x, 0));
            queue.push_back((x, height - 1));
        }
        for y in 0..height {
            queue.push_back((0, y));
            queue.push_back((width - 1, y));
        }
        let start_x = 0;
        let start_y = 0;

        queue.push_back((start_x, start_y));

        while let Some((x, y)) = queue.pop_front() {
            if x < 0 || y < 0 || x >= width || y >= height {
                continue;
            }

            let index = (x as usize) + width as usize * (y as usize);
            if visited[index] || new_stuff_thing[index] == 'X' {
                continue;
            }

            if new_stuff_thing[index] == '.' {
                visited[index] = true;
                things_that_is_o[index] = true;

                // Add adjacent pixels to the queue
                queue.push_back((x + 1, y));
                queue.push_back((x - 1, y));
                queue.push_back((x, y + 1));
                queue.push_back((x, y - 1));
            }
        }

        let mut fancy_map: Vec<char> = vec!['.'; pipe_map.map.len()];
        for (i, o) in things_that_is_o.iter().enumerate() {
            if *o {
                fancy_map[i] = 'o';
            }
        }

        for (_i, o) in visited_clone.clone().iter().enumerate() {
            fancy_map[*o] = 'X';
        }

        self.puzzle2_result = fancy_map.iter().filter(|c| **c == '.').count() as i64;
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

    puzzle1_test!(Day10, "8", "6613");
    puzzle2_test_extra!(Day10, "8", "511");
}
