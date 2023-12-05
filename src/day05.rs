use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day05, i64);
default_new_ctor!(Day05);

#[derive(Debug, Clone)]
struct AlmanacMap {
    dest_range: i64,
    src_range: i64,
    range: i64,
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<i64>,
    map: HashMap<String, Vec<AlmanacMap>>,
}

impl Day05 {
    fn parse_data(input_str: String) -> Almanac {
        let mut map: HashMap<String, Vec<AlmanacMap>> = HashMap::new();

        let mut inputs = input_str.split("\n\n");

        let seeds = inputs
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .filter(|x| x.parse::<i64>().is_ok())
            .map(|x| x.parse::<i64>().unwrap_or_default())
            .collect();

        for input in inputs {
            let mut lines = input.split('\n');
            let title = lines.next().unwrap().replace(" map:", "");
            let key = title.split('-').nth(2).unwrap();

            let mut almanac_maps: Vec<AlmanacMap> = Vec::new();
            for line in lines {
                let numbers: Vec<i64> = line
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
                almanac_maps.push(AlmanacMap {
                    dest_range: *numbers.first().unwrap(),
                    src_range: *numbers.get(1).unwrap(),
                    range: *numbers.get(2).unwrap(),
                });
            }
            map.insert(key.to_string(), almanac_maps);
        }
        Almanac { seeds, map }
    }

    fn convert_stuff(input_ranges: &[Range<i64>], almnacs: &Vec<AlmanacMap>) -> Vec<Range<i64>> {
        let mut new_ranges: Vec<Range<i64>> = Vec::new();
        let mut ranges = input_ranges.to_owned();

        for input_range in &mut ranges {
            for almnac in almnacs {
                let almnac_range = almnac.src_range..almnac.src_range + almnac.range;
                let max_start = input_range.start.max(almnac_range.start);
                let min_end = input_range.end.min(almnac_range.end);
                if max_start <= min_end {
                    let start = max_start - almnac.src_range + almnac.dest_range;
                    let end = min_end - almnac.src_range + almnac.dest_range;

                    if min_end < input_range.end {
                        input_range.start = min_end;
                    } else if max_start > input_range.start {
                        input_range.end = max_start;
                    } else {
                        // Hack..
                        input_range.start = -10;
                        input_range.end = -10;
                    }
                    new_ranges.push(start..end);
                }
            }

            new_ranges.push(input_range.clone());
        }

        new_ranges.extend(ranges);

        // Remove duplicates
        let set: HashSet<_> = new_ranges.drain(..).collect();
        set.into_iter().collect()
    }
}

impl AdventOfCode for Day05 {
    fn day_str(&self) -> String {
        "day05".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let almnac = Day05::parse_data(input_str);

        let mut ranges = Vec::new();
        for seed in almnac.seeds {
            let start = seed;
            let no_of_seeds = 1;
            let range = start..start + no_of_seeds;
            ranges.push(range);
        }

        let soil = Day05::convert_stuff(&ranges, almnac.map.get("soil").unwrap());
        let fertilizer = Day05::convert_stuff(&soil, almnac.map.get("fertilizer").unwrap());
        let water = Day05::convert_stuff(&fertilizer, almnac.map.get("water").unwrap());
        let light = Day05::convert_stuff(&water, almnac.map.get("light").unwrap());
        let temperature = Day05::convert_stuff(&light, almnac.map.get("temperature").unwrap());
        let humidity = Day05::convert_stuff(&temperature, almnac.map.get("humidity").unwrap());
        let location = Day05::convert_stuff(&humidity, almnac.map.get("location").unwrap());

        let min_range = location
            .iter()
            .filter(|x| x.start >= 0)
            .min_by(|x, y| x.start.cmp(&y.start))
            .unwrap();
        self.puzzle1_result = min_range.start;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let almnac = Day05::parse_data(input_str);

        let mut ranges = Vec::new();
        for seed in almnac.seeds.chunks(2) {
            let start = *seed.first().unwrap();
            let no_of_seeds = *seed.get(1).unwrap();
            let range = start..start + no_of_seeds;
            ranges.push(range);
        }

        let soil = Day05::convert_stuff(&ranges, almnac.map.get("soil").unwrap());
        let fertilizer = Day05::convert_stuff(&soil, almnac.map.get("fertilizer").unwrap());
        let water = Day05::convert_stuff(&fertilizer, almnac.map.get("water").unwrap());
        let light = Day05::convert_stuff(&water, almnac.map.get("light").unwrap());
        let temperature = Day05::convert_stuff(&light, almnac.map.get("temperature").unwrap());
        let humidity = Day05::convert_stuff(&temperature, almnac.map.get("humidity").unwrap());
        let location = Day05::convert_stuff(&humidity, almnac.map.get("location").unwrap());

        let min_range = location
            .iter()
            .filter(|x| x.start >= 0)
            .min_by(|x, y| x.start.cmp(&y.start))
            .unwrap();
        self.puzzle2_result = min_range.start;
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

    puzzle1_test!(Day05, "35", "551761867");
    puzzle2_test!(Day05, "46", "57451709");
}
