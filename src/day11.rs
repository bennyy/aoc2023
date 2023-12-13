use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day11, i64);
default_new_ctor!(Day11);

#[derive(Debug, Clone)]
struct GalaxyMap {
    map: Vec<(i64, i64)>,
    width: usize,
    height: usize,
}

impl Day11 {
    fn parse_data(input_str: String) -> GalaxyMap {
        let mut map: Vec<(i64, i64)> = Vec::new();
        let lines_str = input_str.split('\n');
        let height = lines_str.clone().count();
        let width = lines_str.clone().next().unwrap().len();

        for (y, line) in input_str.split('\n').enumerate() {
            let a: Vec<(i64, i64)> = line
                .chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(|(x, _c)| (x as i64, y as i64))
                .collect();
            map.extend(a);
        }

        GalaxyMap { map, width, height }
    }

    fn manhattan_distance(point1: (i64, i64), point2: (i64, i64)) -> i64 {
        let (x1, y1) = point1;
        let (x2, y2) = point2;

        let dx = (x1 - x2).abs();
        let dy = (y1 - y2).abs();

        dx + dy
    }

    fn run(distance_between_galaxies: i64, galaxy_map: &GalaxyMap) -> i64 {
        let width = galaxy_map.width as i64;
        let height = galaxy_map.height as i64;

        let x: Vec<i64> = galaxy_map.map.iter().map(|(x, _y)| *x).collect();
        let y: Vec<i64> = galaxy_map.map.iter().map(|(_x, y)| *y).collect();

        let mut no_galaxies_on_x: HashSet<i64> = (0..width).collect();
        let mut no_galaxies_on_y: HashSet<i64> = (0..height).collect();

        for &n in &x {
            no_galaxies_on_x.remove(&n);
        }
        for &n in &y {
            no_galaxies_on_y.remove(&n);
        }

        let mut no_galaxies_on_x: Vec<_> = no_galaxies_on_x.into_iter().collect();
        let mut no_galaxies_on_y: Vec<_> = no_galaxies_on_y.into_iter().collect();

        no_galaxies_on_x.sort();
        no_galaxies_on_y.sort();

        let mut updated_map = galaxy_map.map.clone();

        let multiplier = if distance_between_galaxies == 1 {
            1
        } else {
            distance_between_galaxies - 1
        };
        for (x, _y) in updated_map.iter_mut() {
            let tmp_x = *x;
            *x += multiplier * no_galaxies_on_x.iter().filter(|a| **a < tmp_x).count() as i64;
        }

        for (_x, y) in updated_map.iter_mut() {
            let tmp_y = *y;
            *y += multiplier * no_galaxies_on_y.iter().filter(|a| **a < tmp_y).count() as i64;
        }

        let mut visited_distances: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
        let mut result: i64 = 0;
        for start in updated_map.clone() {
            for end in updated_map.clone() {
                if start == end {
                    continue;
                }
                let hs_first = min(start, end);
                let hs_end = max(start, end);

                if visited_distances.contains(&(hs_first, hs_end)) {
                    continue;
                }
                visited_distances.insert((hs_first, hs_end));
                result += Day11::manhattan_distance(hs_first, hs_end);
            }
        }
        result
    }
}

impl AdventOfCode for Day11 {
    fn day_str(&self) -> String {
        "day11".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let galaxy_map = Day11::parse_data(input_str);
        self.puzzle1_result = Day11::run(1, &galaxy_map);
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let galaxy_map = Day11::parse_data(input_str);
        self.puzzle2_result = Day11::run(1_000_000, &galaxy_map);
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

    puzzle1_test!(Day11, "374", "9799681");
    puzzle2_test!(Day11, "82000210", "513171773355");
}
