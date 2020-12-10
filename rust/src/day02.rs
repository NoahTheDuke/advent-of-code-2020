use lazy_static::lazy_static;
use regex::Regex;

// --- Day 2: Password Philosophy ---

#[derive(Debug)]
struct PasswordRule {
    low: usize,
    high: usize,
    letter: char,
    password: String,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
}

fn build_password_rule(line: &str) -> PasswordRule {
    let matches = RE.captures(line).unwrap();

    PasswordRule {
        low: matches.get(1).unwrap().as_str().parse().unwrap(),
        high: matches.get(2).unwrap().as_str().parse().unwrap(),
        letter: matches.get(3).unwrap().as_str().chars().next().unwrap(),
        password: matches.get(4).unwrap().as_str().to_string(),
    }
}

// 447
pub fn part1(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let rule = build_password_rule(line);
            let count_matches = rule.password.matches(rule.letter).count();
            rule.low <= count_matches && count_matches <= rule.high
        })
        .count()
}

// 249
pub fn part2(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let rule = build_password_rule(line);
            let first_position = rule.password.chars().nth(rule.low - 1).unwrap();
            let second_position = rule.password.chars().nth(rule.high - 1).unwrap();
            (first_position == rule.letter) ^ (second_position == rule.letter)
        })
        .count()
}
