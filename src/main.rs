use clap::{App, Arg};

use crate::aoc::AdventOfCode;

use crate::day01::Day01;
use crate::day02::Day02;
use crate::day03::Day03;
use crate::day04::Day04;
use crate::day05::Day05;
use crate::day06::Day06;
use crate::day07::Day07;
use crate::day08::Day08;
use crate::day09::Day09;
use crate::day10::Day10;
use crate::day11::Day11;
use crate::day12::Day12;
use crate::day13::Day13;
use crate::day14::Day14;
use crate::day15::Day15;
use crate::day16::Day16;
use crate::day17::Day17;
use crate::day18::Day18;
use crate::day19::Day19;
use crate::day20::Day20;
use crate::day21::Day21;
use crate::day22::Day22;
use crate::day23::Day23;
use crate::day24::Day24;
use crate::day25::Day25;
use std::time::Instant;

mod aoc;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod file_util;

fn run_all_puzzles(input_day: Option<u32>) {
    let array: [Box<dyn AdventOfCode>; 25] = [
        Box::new(Day01::new()),
        Box::new(Day02::new()),
        Box::new(Day03::new()),
        Box::new(Day04::new()),
        Box::new(Day05::new()),
        Box::new(Day06::new()),
        Box::new(Day07::new()),
        Box::new(Day08::new()),
        Box::new(Day09::new()),
        Box::new(Day10::new()),
        Box::new(Day11::new()),
        Box::new(Day12::new()),
        Box::new(Day13::new()),
        Box::new(Day14::new()),
        Box::new(Day15::new()),
        Box::new(Day16::new()),
        Box::new(Day17::new()),
        Box::new(Day18::new()),
        Box::new(Day19::new()),
        Box::new(Day20::new()),
        Box::new(Day21::new()),
        Box::new(Day22::new()),
        Box::new(Day23::new()),
        Box::new(Day24::new()),
        Box::new(Day25::new()),
    ];

    let mut days: Vec<Box<dyn AdventOfCode>> = Vec::from(array);
    let value_width = 15;
    let time_width = 10;
    println!(
        "+ {:->4} + {:-<value_width$} + {:-<time_width$} + {:-<value_width$} + {:-<time_width$} +",
        "", "", "", "", ""
    );
    println!(
        "| {:>4} | {:<value_width$} | {:<time_width$} | {:<value_width$} | {:<time_width$} |",
        "Day", "Puzzle 1", "Time", "Puzzle 2", "Time"
    );
    println!(
        "+ {:->4} + {:-<value_width$} + {:-<time_width$} + {:-<value_width$} + {:-<time_width$} +",
        "", "", "", "", ""
    );
    for (day, puzzle) in days.iter_mut().enumerate() {
        if input_day.is_some() && input_day.unwrap() as usize != day + 1 {
            continue;
        }

        let input_str: String =
            file_util::file_to_string(format!("inputs/{}.txt", puzzle.day_str()));

        let start1 = Instant::now();
        puzzle.run_puzzle1(input_str.clone());
        let puzzle1_time = start1.elapsed().as_micros();

        let start2 = Instant::now();
        puzzle.run_puzzle2(input_str.clone());
        let puzzle2_time = start2.elapsed().as_micros();

        let result1 = puzzle.get_puzzle1_result();
        let result2 = puzzle.get_puzzle2_result();

        let result1_str = result1.unwrap_or_else(|| "-".to_string());
        let result2_str = result2.unwrap_or_else(|| "-".to_string());

        let time1_str = puzzle1_time.to_string() + " us";
        let time2_str = puzzle2_time.to_string() + " us";

        println!(
            "| {:>4} | {:<value_width$} | {:<time_width$} | {:<value_width$} | {:<time_width$} |",
            day + 1,
            result1_str,
            time1_str,
            result2_str,
            time2_str
        );
    }
    println!(
        "+ {:->4} + {:-<value_width$} + {:-<time_width$} + {:-<value_width$} + {:-<time_width$} +",
        "", "", "", "", ""
    );
}

fn main() {
    // Define the command-line interface using clap
    let matches = App::new("Advent of Code 2023")
        .version("1.0")
        .author("bennyy")
        .about("Solutions for Adcent of Code 2023")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Run the specific day")
                .takes_value(true),
        )
        .get_matches();

    let mut day: Option<u32> = None;
    if let Some(day_str) = matches.value_of("day") {
        let d: u32 = day_str.parse().expect("Failed to parse day as u32");
        day = Some(d);
    }

    run_all_puzzles(day);
}
