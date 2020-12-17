// --- Day 13: Shuttle Search ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bus {
    id: usize,
    rem: usize,
}

impl Bus {
    fn new(id: usize) -> Bus {
        Bus {
            id,
            rem: usize::MAX,
        }
    }

    fn calculate(&self) -> usize {
        self.id * self.rem
    }
}

struct Data {
    wait: usize,
    nums: Vec<Bus>,
}

fn parse_p1(input: String) -> Data {
    let lines: Vec<&str> = input.lines().collect();
    let wait = lines[0].parse().unwrap();
    let nums: Vec<Bus> = lines[1]
        .split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse::<usize>().unwrap_or(0))
        .filter(|x| *x > 0)
        .map(Bus::new)
        .collect();
    Data { wait, nums }
}

// 161
pub fn part1(input: String) -> usize {
    let Data {nums, wait} = parse_p1(input);
    nums[1..]
        .iter()
        .map(|bus| Bus {
            id: bus.id,
            rem: bus.id - (wait % bus.id),
        })
        .fold(
            Bus::new(1),
            |acc: Bus, bus: Bus| {
                if bus.rem < acc.rem {
                    bus
                } else {
                    acc
                }
            },
        )
        .calculate()
}

// stolen from https://www.reddit.com/r/adventofcode/comments/kc4njx/2020_day_13_solutions/gfosi5s/
// 213890632230818
pub fn part2(input: String) -> usize {
    input
        .lines()
        .last()
        .unwrap()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::ok)
        .enumerate()
        .filter_map(|(idx, bus)| bus.map(|bus| (idx, bus)))
        .fold((0, 1), |(mut solution, step), (idx, bus)| {
            solution = (solution..)
                .step_by(step)
                .find(|x| (x + idx) % bus == 0)
                .unwrap();
            (solution, step * bus)
        })
        .0
}
