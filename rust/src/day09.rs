use std::collections::VecDeque;

// --- Day 9: Encoding Error ---

pub fn part1(input: String) -> usize {
    let mut preamble: VecDeque<_> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    let mut lines = preamble.split_off(25);
    let mut val = 0;

    while let Some(next) = lines.pop_front() {
        let mut found = false;
        'outer: for i in 0..24 {
            for j in i + 1..25 {
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
    let mut result = 0;

    // // 19ms 254µs
    // for i in 1..lines.len() {
    //     for j in (i + 1)..lines.len() {
    //         let test = &lines[i..j];
    //         let sum: u64 = test.iter().sum();
    //         if sum == GOAL {
    //             result = test.iter().min().unwrap() + test.iter().max().unwrap();
    //         }
    //     }
    // }

    // // 76µs
    // 'outer: for i in 2..lines.len() {
    //     for window in lines.windows(i) {
    //         let sum: u64 = window.iter().sum();
    //         if sum == GOAL {
    //             result = window.iter().min().unwrap() + window.iter().max().unwrap();
    //             break 'outer;
    //         }
    //     }
    // }

    // 1µs
    let mut tail = 0;
    let mut head = 1;
    let mut sum = lines[0];
    while head < lines.len() {
        if sum < GOAL {
            sum += lines[head];
            head += 1;
        } else if sum > GOAL {
            sum -= lines[tail];
            tail += 1;
        } else {
            let set = &lines[tail..head + 1];
            result = set.iter().min().unwrap() + set.iter().max().unwrap();
            break;
        }
    }

    result as usize
}
