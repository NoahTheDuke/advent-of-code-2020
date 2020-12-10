use std::collections::HashSet;
use itertools::Itertools;

// --- Day 1: Report Repair ---

static GOAL: u64 = 2020;

// 996996
pub fn part1(input: String) -> usize {
    let numbers = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    let mut result: u64 = 0;

    for n in numbers.into_iter().combinations(2) {
        if n.iter().sum::<u64>() == GOAL {
            result = n.iter().product();
            break;
        }
    }

    result as usize
}

// 9210402
pub fn part2(input: String) -> usize {
    let numbers = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<HashSet<u64>>();
    let mut result: u64 = 0;

    for n in numbers.iter().combinations(2) {
        let n1 = n[0];
        let n2 = n[1];
        let n3 = GOAL.checked_sub(n1 + n2);
        if let Some(subbed) = n3 {
            if numbers.contains(&subbed) {
                result = n1 * n2 * subbed;
                break;
            }
        }
    }

    result as usize
}
