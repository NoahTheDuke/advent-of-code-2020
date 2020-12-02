// use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

// --- Day 2: Password Philosophy ---

#[derive(Debug)]
struct PasswordRule {
    low: usize,
    high: usize,
    letter: String,
    password: String,
}

fn build_password_rule(line: String) -> PasswordRule {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }
    let matches = RE.captures(&line).unwrap();

    PasswordRule {
        low: matches.get(1).unwrap().as_str().parse().unwrap(),
        high: matches.get(2).unwrap().as_str().parse().unwrap(),
        letter: matches.get(3).unwrap().as_str().to_string(),
        password: matches.get(4).unwrap().as_str().to_string(),
    }
}

pub fn part1(inp: String) {
    let lines = inp.lines().map(|x| x.to_string()).collect::<Vec<String>>();
    let mut count = 0;
    for line in lines {
        let rule = build_password_rule(line);
        let count_matches = rule.password.matches(&rule.letter).count();
        if rule.low <= count_matches && count_matches <= rule.high {
            count += 1;
        }
    }
    println!("{:?}", count);
}

pub fn part2(inp: String) {
    let lines = inp.lines().map(|x| x.to_string()).collect::<Vec<String>>();
    let mut count = 0;
    for line in lines {
        let rule = build_password_rule(line);
        let first_position = rule.password.chars().nth(rule.low - 1).unwrap();
        let second_position = rule.password.chars().nth(rule.high - 1).unwrap();

        match (first_position.to_string(), second_position.to_string()) {
            (f, s) if f == rule.letter && s == rule.letter => {},
            (f, _) if f == rule.letter => count += 1,
            (_, s) if s == rule.letter => count += 1,
            (_, _) => {},
        }
    }
    println!("{:?}", count);
}
