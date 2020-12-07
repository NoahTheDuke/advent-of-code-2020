use std::collections::{HashMap, HashSet};

// --- Day 6: Custom Customs ---

pub fn part1(input: String) -> String {
    input.split("\n\n")
        .map(|line| {
            line.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input.split("\n\n")
        .map(|group| {
            let mut counter: HashMap<char, usize> = HashMap::new();
            for c in group.chars() {
                if c.is_alphanumeric() {
                    *counter.entry(c).or_insert(0) += 1;
                }
            }
            let group_size = group.lines().count();
            counter
                .values()
                .filter(|v| **v == group_size)
                .count()
        })
        .sum::<usize>()
        .to_string()
}
