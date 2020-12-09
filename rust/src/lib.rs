pub mod fmt_dur;
use std::env;
use std::fs;
use std::io;
use std::time::{Instant};
use fmt_dur::{fmt_dur};

// Days
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

pub fn noop(_inp: String) -> usize {
    0
}

pub type DayFn = fn(String) -> usize;

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, day04::part2),
        5 => (day05::part1, day05::part2),
        6 => (day06::part1, day06::part2),
        7 => (day07::part1, day07::part2),
        8 => (day08::part1, day08::part2),
        9 => (day09::part1, day09::part2),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}

pub fn run_both_days(args: Vec<String>) {
    // Get day string
    let mut day = String::new();

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }
    // Parse day as number
    day = day.trim().to_string();
    let day_num: u32 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };
    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd
        .join("..")
        .join("inputs")
        .join(format!("day{:02}.txt", day_num));
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");

    // Get corresponding function

    let to_run = get_day(day_num);
    // Time it
    if to_run.0 != noop {
        println!("Running Part 1");
        let part1_input = input.clone();
        let part1_start = Instant::now();
        let result = to_run.0(part1_input);
        let part1_dur = part1_start.elapsed();
        println!("{:}, {}", result, fmt_dur(part1_dur));
    }

    if to_run.1 != noop {
        println!("Running Part 2");
        let part2_input = input.clone();
        let part2_start = Instant::now();
        let result = to_run.1(part2_input);
        let part2_dur = part2_start.elapsed();
        println!("{:}, {}", result, fmt_dur(part2_dur));
    }
}
