// --- Day 12: Rain Risk ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DIRECTION {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    heading: DIRECTION,
    value: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ship {
    facing: DIRECTION,
    x_pos: isize,
    y_pos: isize,
    waypoint: Waypoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Waypoint {
    x_pos: isize,
    y_pos: isize,
}

const LEFT_TURN: [DIRECTION; 8] = [
    DIRECTION::North,
    DIRECTION::West,
    DIRECTION::South,
    DIRECTION::East,
    DIRECTION::North,
    DIRECTION::West,
    DIRECTION::South,
    DIRECTION::East,
];

const RIGHT_TURN: [DIRECTION; 8] = [
    DIRECTION::North,
    DIRECTION::East,
    DIRECTION::South,
    DIRECTION::West,
    DIRECTION::North,
    DIRECTION::East,
    DIRECTION::South,
    DIRECTION::West,
];

impl Ship {
    fn new() -> Ship {
        Ship {
            facing: DIRECTION::East,
            x_pos: 0,
            y_pos: 0,
            waypoint: Waypoint::new(),
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) {
        let Instruction { heading, value } = instruction;
        match heading {
            DIRECTION::North => self.y_pos += value,
            DIRECTION::East => self.x_pos += value,
            DIRECTION::South => self.y_pos -= value,
            DIRECTION::West => self.x_pos -= value,
            DIRECTION::Left | DIRECTION::Right => self.turn(instruction),
            DIRECTION::Forward => match self.facing {
                DIRECTION::North => self.y_pos += value,
                DIRECTION::East => self.x_pos += value,
                DIRECTION::South => self.y_pos -= value,
                DIRECTION::West => self.x_pos -= value,
                _ => unreachable!(),
            },
        }
    }

    fn turn(&mut self, instruction: Instruction) {
        let Instruction { heading, value } = instruction;
        let steps = (value / 90) as usize;
        self.facing = match (heading, self.facing) {
            (DIRECTION::Left, DIRECTION::North) => LEFT_TURN[steps],
            (DIRECTION::Left, DIRECTION::West) => LEFT_TURN[steps + 1],
            (DIRECTION::Left, DIRECTION::South) => LEFT_TURN[steps + 2],
            (DIRECTION::Left, DIRECTION::East) => LEFT_TURN[steps + 3],
            (DIRECTION::Right, DIRECTION::North) => RIGHT_TURN[steps],
            (DIRECTION::Right, DIRECTION::East) => RIGHT_TURN[steps + 1],
            (DIRECTION::Right, DIRECTION::South) => RIGHT_TURN[steps + 2],
            (DIRECTION::Right, DIRECTION::West) => RIGHT_TURN[steps + 3],
            _ => unreachable!(),
        }
    }

    fn follow_waypoint(&mut self, instruction: Instruction) {
        let Instruction { heading, value } = instruction;
        match heading {
            DIRECTION::North | DIRECTION::East | DIRECTION::South | DIRECTION::West => {
                self.waypoint.move_(heading, value)
            }
            DIRECTION::Left | DIRECTION::Right => self.waypoint.rotate(heading, value),
            DIRECTION::Forward => {
                for _ in 0..value {
                    self.x_pos += self.waypoint.x_pos;
                    self.y_pos += self.waypoint.y_pos;
                }
            }
        }
    }
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint {
            x_pos: 10,
            y_pos: 1,
        }
    }

    fn move_(&mut self, heading: DIRECTION, value: isize) {
        match heading {
            DIRECTION::North => self.y_pos += value,
            DIRECTION::East => self.x_pos += value,
            DIRECTION::South => self.y_pos -= value,
            DIRECTION::West => self.x_pos -= value,
            _ => unreachable!(),
        }
    }

    fn rotate(&mut self, heading: DIRECTION, value: isize) {
        let x_pos = self.x_pos;
        let y_pos = self.y_pos;
        match (heading, value) {
            (_, 180) => {
                self.x_pos = -x_pos;
                self.y_pos = -y_pos;
            }
            (DIRECTION::Left, 90) | (DIRECTION::Right, 270) => {
                self.x_pos = -y_pos;
                self.y_pos = x_pos;
            }
            (DIRECTION::Left, 270) | (DIRECTION::Right, 90) => {
                self.x_pos = y_pos;
                self.y_pos = -x_pos;
            }
            _ => unreachable!(),
        }
    }
}

fn to_inst(line: &str) -> Instruction {
    let (left, right) = line.split_at(1);
    let heading = match left {
        "N" => DIRECTION::North,
        "E" => DIRECTION::East,
        "S" => DIRECTION::South,
        "W" => DIRECTION::West,
        "L" => DIRECTION::Left,
        "R" => DIRECTION::Right,
        "F" => DIRECTION::Forward,
        _ => unreachable!(),
    };
    let value = right.parse().unwrap();
    Instruction {
        heading: heading,
        value: value,
    }
}

fn parse(input: String) -> Vec<Instruction> {
    input.lines().map(to_inst).collect()
}

// 420
pub fn part1(input: String) -> usize {
    let mut ship = Ship::new();
    for instruction in parse(input) {
        ship.process_instruction(instruction);
    }
    (ship.x_pos.abs() + ship.y_pos.abs()) as usize
}

pub fn part2(input: String) -> usize {
    let mut ship = Ship::new();
    for instruction in parse(input) {
        ship.follow_waypoint(instruction);
    }
    (ship.x_pos.abs() + ship.y_pos.abs()) as usize
}
