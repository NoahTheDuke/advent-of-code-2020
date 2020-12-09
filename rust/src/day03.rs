// --- Day 3: Toboggan Trajectory ---

fn stepper(input: &String, right: usize, down: usize) -> u64 {
    // this was my initial thought, but the off-by-one errors really hecked me.
    // if i hardcode width to be 32, this is only slightly slower than the `filter`
    // method that's popular on reddit. given that the fun is solving this however,
    // i don't mind either way.

    // let width = input.find("\n").unwrap() + 1;
    let width = 32;
    let mut count: u64 = 0;
    let mut right_count = 0;
    let mut row = 0;
    loop {
        let col = (right_count * right) % (width - 1);
        let idx = (row * width) + col;
        if idx >= input.len() {
            break;
        } else if input.as_bytes()[idx] == b'#' {
            count += 1;
        }
        row += down;
        right_count += 1;
    }

    count
}

pub fn part1(input: String) -> usize {
    stepper(&input, 3, 1) as usize
}

pub fn part2(input: String) -> usize {
    let angles = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result: u64 = 1;

    for (right, down) in angles {
        result *= stepper(&input, right, down);
    }

    result as usize
}
