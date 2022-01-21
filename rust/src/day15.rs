// --- Day 15: Rambunctious Recitation ---

fn parse(input: String) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .filter_map(|x| x.parse().ok())
        .collect()
}

// 2011
pub fn part1(input: String) -> usize {
    let numbers = parse(input);

    let mut memory = [0; 65536];
    let mut order = [0; 65536];
    let mut last_number = 0;

    for (idx, number) in numbers.iter().enumerate() {
        memory[*number] += 1;
        order[*number] = idx;
        last_number = *number;
    }

    for step in numbers.len()..2020 {
        let is_new = memory[last_number];

        let current_number = if is_new == 1 {
            0
        } else {
            step - 1 - order[last_number]
        };

        memory[current_number] += 1;
        order[last_number] = step - 1;
        last_number = current_number;
    }

    last_number
}

// 2159626
pub fn part2(input: String) -> usize {
    let numbers = parse(input);

    let mut memory: Vec<usize> = Vec::new();
    memory.resize_with(30000000, Default::default);
    let mut order: Vec<usize> = Vec::new();
    order.resize_with(30000000, Default::default);
    let mut last_number = 0;

    for (idx, number) in numbers.iter().enumerate() {
        memory[*number] += 1;
        order[*number] = idx;
        last_number = *number;
    }

    for step in numbers.len()..30000000 {
        let is_new = memory[last_number];

        let current_number = if is_new == 1 {
            0
        } else {
            step - 1 - order[last_number]
        };

        memory[current_number] += 1;
        order[last_number] = step - 1;
        last_number = current_number;
    }

    last_number
}
