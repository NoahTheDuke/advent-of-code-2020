use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

// --- Day 7: Handy Haversacks ---

lazy_static! {
    static ref NODE: Regex =
        Regex::new(r"^(?P<node>\w+? \w+?) bags contain (?P<rest>.*)\.$").unwrap();
    static ref EDGE: Regex = Regex::new(r"(?P<qty>\d+) (?P<node>\w+? \w+?) bag").unwrap();
}

fn parse1(input: &String) -> HashMap<&str, Vec<(&str, &str)>> {
    let mut bags = HashMap::new();

    for line in input.lines() {
        if let Some(matches) = NODE.captures(&line) {
            match (matches.name("node"), matches.name("rest")) {
                (Some(node), Some(rest)) => {
                    for capture in EDGE.captures_iter(&rest.as_str()) {
                        match (capture.name("qty"), capture.name("node")) {
                            (Some(qty), Some(edge)) => {
                                bags.entry(edge.as_str())
                                    .or_insert(Vec::new())
                                    .push((qty.as_str(), node.as_str()));
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
    }
    bags
}

pub fn part1(input: String) -> String {
    let bags = parse1(&input);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back("shiny gold");
    while !queue.is_empty() {
        let bag = queue.pop_front().unwrap();

        if visited.contains(&bag) {
            continue;
        }

        if bag != "shiny gold" {
            visited.insert(bag);
        }

        if let Some(container_bags) = bags.get(bag) {
            for (_qty, container_bag) in container_bags {
                queue.push_back(container_bag);
            }
        }
    }

    visited.len().to_string()
}

fn parse2(input: &String) -> HashMap<&str, Vec<(&str, &str)>> {
    let mut bags = HashMap::new();

    for line in input.lines() {
        if let Some(matches) = NODE.captures(&line) {
            match (matches.name("node"), matches.name("rest")) {
                (Some(node), Some(rest)) => {
                    for capture in EDGE.captures_iter(&rest.as_str()) {
                        match (capture.name("qty"), capture.name("node")) {
                            (Some(qty), Some(edge)) => {
                                bags.entry(node.as_str())
                                    .or_insert(Vec::new())
                                    .push((qty.as_str(), edge.as_str()));
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
    }
    bags
}

fn dfs(bags: &HashMap<&str, Vec<(&str, &str)>>, current: &str) -> usize {
    let mut result: usize = 1;
    if let Some(container_bags) = bags.get(current) {
        for (qty, container_bag) in container_bags {
            result += qty.parse::<usize>().unwrap() * dfs(bags, container_bag);
        }
    }
    result
}

pub fn part2(input: String) -> String {
    let bags = parse2(&input);
    let result = dfs(&bags, "shiny gold") - 1;

    result.to_string()
}
