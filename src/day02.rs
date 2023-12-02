use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day02, i32);
default_new_ctor!(Day02);

#[derive(Copy, Debug, Clone)]
struct RoundStat {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    rounds: Vec<RoundStat>,
}

impl Day02 {
    fn parse_data(input_str: String) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        for input in input_str.split('\n') {
            let modified_input = input.replace("Game ", "");

            // Get the game id, and the other half is the "rounds",
            let game_id_split: Vec<&str> = modified_input.split(':').collect();
            let id = game_id_split.first().unwrap().parse::<i32>().unwrap();
            let game_rounds = game_id_split.last().unwrap();

            let mut rounds_result: Vec<RoundStat> = Vec::new();
            for round in game_rounds.split(';') {
                let mut current_round = RoundStat {
                    blue: 0,
                    red: 0,
                    green: 0,
                };

                for cube in round.split(',') {
                    let cube_info: Vec<&str> = cube.trim().split(' ').collect();
                    let number_of_cubes = cube_info.first().unwrap().trim().parse::<i32>().unwrap();
                    let color = cube_info.last().unwrap();
                    if color.contains("green") {
                        current_round.green = number_of_cubes;
                    }
                    if color.contains("red") {
                        current_round.red = number_of_cubes;
                    }
                    if color.contains("blue") {
                        current_round.blue = number_of_cubes;
                    }
                }

                rounds_result.push(current_round);
            }

            games.push(Game {
                id,
                rounds: rounds_result,
            });
        }

        games
    }
}

impl AdventOfCode for Day02 {
    fn day_str(&self) -> String {
        "day02".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let games: Vec<Game> = Day02::parse_data(input_str);

        self.puzzle1_result = games
            .iter()
            .filter(|game| {
                game.rounds
                    .iter()
                    // 12 red cubes, 13 green cubes, and 14 blue cubes
                    .all(|x| x.red <= 12 && x.green <= 13 && x.blue <= 14)
            })
            .map(|x| x.id)
            .sum();
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let games: Vec<Game> = Day02::parse_data(input_str);
        self.puzzle2_result = games
            .iter()
            .map(|x| {
                let red = x
                    .rounds
                    .iter()
                    .max_by(|x, y| x.red.cmp(&y.red))
                    .unwrap()
                    .red;
                let green = x
                    .rounds
                    .iter()
                    .max_by(|x, y| x.green.cmp(&y.green))
                    .unwrap()
                    .green;
                let blue = x
                    .rounds
                    .iter()
                    .max_by(|x, y| x.blue.cmp(&y.blue))
                    .unwrap()
                    .blue;
                red * green * blue
            })
            .sum();
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

    puzzle1_test!(Day02, "8", "1931");
    puzzle2_test!(Day02, "2286", "83105");
}
