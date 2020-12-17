use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

pub mod fmt_dur;
use fmt_dur::{fmt_dur, fmt_time};

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
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

pub fn noop(_inp: String) -> usize {
    0
}

pub type DayFn = fn(String) -> usize;

pub type DaysMap = HashMap<usize, (DayFn, DayFn)>;

fn build_days() -> DaysMap {
    let mut days: DaysMap = HashMap::new();
    days.insert(01, (day01::part1, day01::part2));
    days.insert(02, (day02::part1, day02::part2));
    days.insert(03, (day03::part1, day03::part2));
    days.insert(04, (day04::part1, day04::part2));
    days.insert(05, (day05::part1, day05::part2));
    days.insert(06, (day06::part1, day06::part2));
    days.insert(07, (day07::part1, day07::part2));
    days.insert(08, (day08::part1, day08::part2));
    days.insert(09, (day09::part1, day09::part2));
    days.insert(10, (day10::part1, day10::part2));
    days.insert(11, (day11::part1, day11::part2));
    days.insert(12, (day12::part1, day12::part2));
    days.insert(13, (day13::part1, day13::part2));
    days.insert(14, (day14::part1, day14::part2));
    days.insert(15, (day15::part1, day15::part2));

    days
}

fn load_file(day_num: usize) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd
        .join("..")
        .join("inputs")
        .join(format!("day{:02}.txt", day_num));

    fs::read_to_string(filename).expect("Error while reading")
}

fn run_both_days(day_fns: &DaysMap, day_num: usize) -> f64 {
    // Read input file
    let input = load_file(day_num);

    let mut duration = 0.0;

    // Get corresponding function

    if let Some(to_run) = day_fns.get(&day_num) {
        // Time it
        if to_run.0 != noop {
            let part1_input = input.clone();
            let part1_start = Instant::now();
            let result = to_run.0(part1_input);
            let part1_dur = part1_start.elapsed();
            println!(
                "day{:02}::part1: {:}, {}",
                day_num,
                result,
                fmt_dur(part1_dur)
            );
            duration += part1_dur.as_secs_f64();
        }

        if to_run.1 != noop {
            let part2_input = input.clone();
            let part2_start = Instant::now();
            let result = to_run.1(part2_input);
            let part2_dur = part2_start.elapsed();
            println!(
                "day{:02}::part2: {:}, {}",
                day_num,
                result,
                fmt_dur(part2_dur)
            );
            duration += part2_dur.as_secs_f64();
        }
    }

    duration
}

pub fn run(args: Vec<String>) {
    // Get day string
    let mut day = if args.len() >= 2 {
        args[1].clone()
    } else {
        String::from("0")
    };

    // Parse day as number
    day = day.trim().to_string();
    let day_num: usize = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };

    let day_fns = build_days();

    if day_num == 0 {
        let mut duration = 0.0;
        for i in 0..day_fns.len() {
            duration += run_both_days(&day_fns, i + 1);
        }
        println!("\ntotal duration: {}", fmt_time(duration * 1000.0))
    } else {
        run_both_days(&day_fns, day_num);
    }
}
