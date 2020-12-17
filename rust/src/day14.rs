use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

// --- Day 14: Docking Data ---

lazy_static! {
    static ref MEM: Regex = Regex::new(r"mem\[(?P<loc>\d+)\] = (?P<value>\d+)").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair {
    loc: usize,
    value: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Chunk {
    mask: String,
    values: Vec<Pair>,
}

fn parse(input: String) -> Vec<Chunk> {
    input
        .split("mask = ")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut lines = x.lines();
            let mask = lines.next().unwrap().to_string();
            let mut values = Vec::new();
            for line in lines {
                let matches = MEM.captures(&line).unwrap();
                let loc = matches.name("loc").unwrap();
                let value = matches.name("value").unwrap();
                values.push(Pair {
                    loc: loc.as_str().parse().unwrap(),
                    value: value.as_str().parse().unwrap(),
                });
            }
            Chunk { mask, values }
        })
        .collect()
}

// 9879607673316
pub fn part1(input: String) -> usize {
    let chunks = parse(input);
    let mut memory = [0; 65536];
    for chunk in chunks {
        let mask = chunk.mask;
        let mask1 = usize::from_str_radix(&mask.replace("1", "0").replace("X", "1"), 2).unwrap();
        let mask2 = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        for Pair { loc, value } in chunk.values {
            memory[loc] = value & mask1 | mask2;
        }
    }
    memory.iter().sum::<usize>() as usize
}

// stolen from https://github.com/ritobanrc/aoc2020/blob/main/src/day14.rs
// 3435342392262
pub fn part2(input: String) -> usize {
    let chunks = parse(input);
    let mut memory = HashMap::new();
    for chunk in chunks {
        let mask = chunk.mask;

        for Pair { mut loc, value } in chunk.values {
            let mut floating_bits = Vec::new();

            for (idx, byte) in mask.bytes().rev().enumerate() {
                match byte {
                    b'0' => continue,
                    b'1' => loc = loc | (1 << idx),
                    b'X' => floating_bits.push(idx),
                    _ => unreachable!(),
                }
            }

            for n in 0..1 << floating_bits.len() {
                let mut new_loc = loc;
                for (idx, bit) in floating_bits.iter().enumerate() {
                    match (n & (1 << idx)) >> idx {
                        0 => new_loc = new_loc & !(1 << bit),
                        1 => new_loc = new_loc | (1 << bit),
                        _ => unreachable!(),
                    }
                }
                memory.insert(new_loc, value);
            }
        }
    }
    memory.values().sum()
}
