use std::str::FromStr;

// --- Day 8: Template ---

#[derive(Debug, PartialEq)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    operand: isize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let opcode = match &input[..3] {
            "acc" | "Acc" => Opcode::Acc,
            "jmp" | "Jmp" => Opcode::Jmp,
            "nop" | "Nop" => Opcode::Nop,
            _ => panic!(),
        };

        let operand = input[4..].parse().unwrap();

        Ok(Instruction {
            opcode: opcode,
            operand: operand,
        })
    }
}

fn execute(instructions: &Vec<Instruction>) -> Result<usize, usize> {
    let mut seen = vec![false; instructions.len()];
    let mut accumulator = 0;
    let mut idx = 0;
    let mut step = 0;

    while step < instructions.len() {
        if *seen.get(idx).unwrap_or(&false) {
            return Err(accumulator as usize);
        }

        if let Some(line) = &instructions.get(idx) {
            seen[idx] = true;

            match line.opcode {
                Opcode::Acc => {
                    accumulator += line.operand;
                    idx += 1;
                }
                Opcode::Jmp => {
                    idx = (idx as isize + line.operand) as usize;
                }
                Opcode::Nop => {
                    idx += 1;
                }
            }
            step += 1;
        } else {
            break;
        }
    }

    Ok(accumulator as usize)
}

pub fn part1(input: String) -> usize {
    let instructions = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect::<Vec<_>>();

    execute(&instructions).err().unwrap()
}

fn swap(instruction: &mut Instruction) {
    instruction.opcode = match instruction.opcode {
        Opcode::Acc => Opcode::Acc,
        Opcode::Jmp => Opcode::Nop,
        Opcode::Nop => Opcode::Jmp,
    }
}

pub fn part2(input: String) -> usize {
    let mut instructions = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect::<Vec<Instruction>>();

    let mut step = 0;
    while step < instructions.len() {
        swap(&mut instructions[step]);

        if let Some(result) = execute(&instructions).ok() {
            return result as usize;
        }

        swap(&mut instructions[step]);
        step += 1;
    }

    0
}
