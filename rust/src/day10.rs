use arrayvec::ArrayVec;
use lexical::parse_partial;

// --- Day 10: Adapter Array ---

fn parse_and_sort(input: String) -> ArrayVec<[u8; 256]> {
    let mut array = ArrayVec::new();

    let mut s = input.as_bytes();
    while s.len() > 1 {
        let (num, pos) = parse_partial::<u8, _>(s).unwrap();
        array.push(num);
        s = &s[pos + 1..]; // +1 to skip \n
    }
    array.sort_unstable();

    array
}

// 1984
pub fn part1(input: String) -> usize {
    let adapters = parse_and_sort(input);
    let mut ones = 0;
    let mut threes = 1;

    match adapters[0] {
        1 => ones += 1,
        2 => (),
        3 => threes += 1,
        _ => unreachable!(),
    }

    for pair in adapters.windows(2) {
        let left = pair[0];
        let right = pair[1];
        match right - left {
            1 => ones += 1,
            2 => (),
            3 => threes += 1,
            _ => unreachable!(),
        }
    }

    ones * threes
}

// 3543369523456
pub fn part2(input: String) -> usize {
    let adapters = parse_and_sort(input);
    let highest = adapters[adapters.len() - 1];

    let mut cache = [0usize; 256];
    cache[0] = 1;

    for adapter in adapters {
        let n1 = cache.get((adapter as usize) - 1).unwrap_or(&0);
        let n2 = cache.get((adapter as usize) - 2).unwrap_or(&0);
        let n3 = cache.get((adapter as usize) - 3).unwrap_or(&0);

        cache[(adapter as usize)] = n1 + n2 + n3;
    }

    *cache.get(highest as usize).unwrap_or(&0) as usize
}
