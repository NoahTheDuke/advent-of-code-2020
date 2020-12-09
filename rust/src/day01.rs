use itertools::Itertools;

// --- Day 1: Report Repair ---

fn search(numbers: Vec<usize>, k: usize) -> Option<usize> {
    for n in numbers.into_iter().combinations(k) {
        if n.iter().sum::<usize>() == 2020 {
            return Some(n.iter().product());
        }
    }
    return None;
}

pub fn part1(inp: String) -> usize {
    let numbers = inp
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    match search(numbers, 2) {
        Some(result) => result,
        None => 0,
    }
}

pub fn part2(inp: String) -> usize {
    let numbers = inp
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    match search(numbers, 3) {
        Some(result) => result,
        None => 0,
    }
}
