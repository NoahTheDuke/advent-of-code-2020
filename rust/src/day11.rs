// --- Day 11: Template ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TILE {
    Floor,
    Chair,
    Occupied,
}

struct Space(Vec<Vec<TILE>>);

impl core::ops::Deref for Space {
    type Target = Vec<Vec<TILE>>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Space {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

fn parse_into_vec(input: String) -> Space {
    let mut space = Vec::new();
    for line in input.lines() {
        let row: Vec<TILE> = line
            .as_bytes()
            .iter()
            .map(|b| match b {
                b'.' => TILE::Floor,
                b'L' => TILE::Chair,
                _ => unreachable!(),
            })
            .collect();
        space.push(row);
    }
    Space(space)
}

impl Space {
    fn count_occupied(&self) -> usize {
        self.iter()
            .map(|row| row.iter().filter(|point| *point == &TILE::Occupied).count())
            .sum()
    }
}

#[rustfmt::skip]
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn iterate(mut space: Space) -> usize {
    let mut count = 0;
    loop {
        let mut next = Vec::with_capacity(93);
        for (row_idx, row) in space.iter().enumerate() {
            let mut next_col = Vec::with_capacity(97);

            for (col_idx, tile) in row.iter().enumerate() {
                let mut num_adjacent = 0;
                for (row_diff, col_diff) in DIRECTIONS.iter() {
                    let row_offset = (row_idx as isize) + row_diff;
                    let col_offset = (col_idx as isize) + col_diff;
                    if (row_offset >= 0)
                        && (row_offset < space.len() as isize)
                        && (col_offset >= 0)
                        && (col_offset < row.len() as isize)
                        && (space[row_offset as usize][col_offset as usize] == TILE::Occupied)
                    {
                        num_adjacent += 1;
                    }
                }

                let next_tile = match tile {
                    TILE::Chair if num_adjacent == 0 => TILE::Occupied,
                    TILE::Occupied if num_adjacent >= 4 => TILE::Chair,
                    _ => *tile,
                };
                next_col.push(next_tile);
            }
            next.push(next_col);
        }
        let next_space = Space(next);
        let next_space_occupied_count = next_space.count_occupied();
        if next_space_occupied_count == count {
            break;
        }
        count = next_space_occupied_count;
        space = next_space;
    }
    count
}

// 2316
pub fn part1(input: String) -> usize {
    iterate(parse_into_vec(input))
}

fn iterate2(mut space: Space) -> usize {
    let mut count = 0;
    loop {
        let mut next = Vec::with_capacity(93);

        for (row_idx, row) in space.iter().enumerate() {
            let mut next_col = Vec::with_capacity(97);

            for (col_idx, tile) in row.iter().enumerate() {
                let mut num_adjacent = 0;

                let mut multiplier = 1;
                'direction: for (row_diff, col_diff) in DIRECTIONS.iter() {
                    'distance: loop {
                        let row_offset = (row_idx as isize) + (row_diff * multiplier);
                        let col_offset = (col_idx as isize) + (col_diff * multiplier);
                        if (row_offset < 0)
                            || (col_offset < 0)
                            || (row_offset >= space.len() as isize)
                            || (col_offset >= row.len() as isize)
                        {
                            multiplier = 1;
                            continue 'direction;
                        }

                        let tile_to_check = space[row_offset as usize][col_offset as usize];

                        match tile_to_check {
                            TILE::Occupied => {
                                num_adjacent += 1;
                                multiplier = 1;
                                break 'distance;
                            }
                            TILE::Chair => {
                                multiplier = 1;
                                break 'distance;
                            }
                            _ => (),
                        }
                        multiplier += 1;
                    }
                }

                let next_tile = match tile {
                    TILE::Chair if num_adjacent == 0 => TILE::Occupied,
                    TILE::Occupied if num_adjacent >= 5 => TILE::Chair,
                    _ => *tile,
                };
                next_col.push(next_tile);
            }
            next.push(next_col);
        }
        let next_space = Space(next);
        let next_space_occupied_count = next_space.count_occupied();
        if next_space_occupied_count == count {
            break;
        }
        count = next_space_occupied_count;
        space = next_space;
    }
    count
}

// 2128
pub fn part2(input: String) -> usize {
    iterate2(parse_into_vec(input))
}
