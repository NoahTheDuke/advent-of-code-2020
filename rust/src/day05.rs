pub fn part1(input: String) -> String {
    // let lines: Vec<&str> = .collect();
    let lines: isize = input
        .lines()
        .map(|line| {
            line.replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1")
        })
        .map(|line| isize::from_str_radix(&line, 2).unwrap())
        .max()
        .unwrap_or(0);
    lines.to_string()
}

pub fn part2(input: String) -> String {
    let mut max = isize::MIN;
    let mut min = isize::MAX;
    let mut total = 0;

    for line in input.lines() {
        let seat_id = isize::from_str_radix(
            &line
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1"),
            2,
        ).unwrap();
        if seat_id > max {
            max = seat_id;
        }
        if seat_id < min {
            min = seat_id;
        }
        total += seat_id;
    }
    ((min..=max).sum::<isize>() - total).to_string()
}
