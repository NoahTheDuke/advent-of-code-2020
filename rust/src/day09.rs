use std::collections::VecDeque;

// --- Day 9: Encoding Error ---

pub fn part1(input: String) -> usize {
    let mut preamble: VecDeque<_> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    let mut lines = preamble.split_off(25);
    let mut val = 0;

    while let Some(next) = lines.pop_front() {
        let mut found = false;
        'outer: for i in 0..25 {
            for j in i..25 {
                match (preamble.get(i), preamble.get(j)) {
                    (Some(f), Some(s)) => {
                        if f + s == next {
                            found = true;
                            break 'outer;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        if !found {
            val = next;
            break;
        }
        preamble.pop_front();
        preamble.push_back(next);
    }

    val as usize
}

static GOAL: u64 = 50047984;

pub fn part2(input: String) -> usize {
    let lines: Vec<_> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    let mut val = 0;

    'outer: for i in 1..lines.len() {
        for j in (i + 1)..lines.len() {
            let test = &lines[i..j];
            let result: u64 = test.iter().sum();
            if result == GOAL {
                val = test.iter().min().unwrap() + test.iter().max().unwrap();
                break 'outer;
            }
        }
    }

    val as usize
}
