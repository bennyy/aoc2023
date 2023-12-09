use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day06, i64);
default_new_ctor!(Day06);

#[derive(Debug, Clone)]
struct Race {
    time: i64,
    distance: i64,
}

impl Day06 {
    fn parse_data(input_str: String) -> Vec<Race> {
        let mut return_vec: Vec<Race> = Vec::new();
        let mut lines_str = input_str.split('\n');
        let times_str = lines_str.next().unwrap().split_ascii_whitespace();
        let distances_str = lines_str.next().unwrap().split_ascii_whitespace();

        let times: Vec<i64> = times_str
            .skip(1)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let distances: Vec<i64> = distances_str
            .skip(1)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        for (time, distance) in times.iter().zip(distances.iter()) {
            return_vec.push(Race {
                time: *time,
                distance: *distance,
            })
        }

        return_vec
    }
}

impl AdventOfCode for Day06 {
    fn day_str(&self) -> String {
        "day06".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let races = Day06::parse_data(input_str);

        let mut results: Vec<i64> = Vec::new();

        for race in races {
            let mut wins = 0;
            for ms in 0..=race.time {
                let time_left = race.time - ms;
                let tot_distance = time_left * ms;
                if tot_distance > race.distance {
                    wins += 1;
                }
            }
            results.push(wins);
        }
        self.puzzle1_result = results.iter().product();
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let races = Day06::parse_data(input_str);
        let mut total_time_str: String = String::new();
        let mut total_distance_str: String = String::new();
        for race in races {
            total_time_str.push_str(&race.time.to_string());
            total_distance_str.push_str(&race.distance.to_string());
        }
        let total_time: i64 = total_time_str.parse::<i64>().unwrap();
        let total_distance: i64 = total_distance_str.parse::<i64>().unwrap();

        let a = 1_f64;
        let b = total_time as f64;
        let c = total_distance as f64;

        let discriminant = b * b - 4.0 * a * c;
        let root1 = ((-b + discriminant.sqrt()) / (2.0 * a)).floor() as i64;
        let root2 = ((-b - discriminant.sqrt()) / (2.0 * a)).ceil() as i64;

        self.puzzle2_result = root1 - root2 + 1;
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

    puzzle1_test!(Day06, "288", "160816");
    puzzle2_test!(Day06, "71503", "46561107");
}
