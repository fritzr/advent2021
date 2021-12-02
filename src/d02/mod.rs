use std::io::BufRead;
use std::error::Error;

use crate::cli;
use crate::Day;

enum Instruction {
    MoveX(i32),
    MoveY(i32),
}

fn part1(instructions: &Vec<Instruction>) -> String {
    let mut xpos = 0;
    let mut ypos = 0;
    for instruction in instructions {
        match instruction {
            Instruction::MoveX(val) => xpos += val,
            Instruction::MoveY(val) => ypos += val,
        }
    }
    return format!("{} forward x {} down = {}", xpos, ypos, xpos * ypos);
}

fn part2(instructions: &Vec<Instruction>) -> String {
    let mut xpos = 0;
    let mut ypos = 0;
    let mut aim = 0;
    for instruction in instructions {
        match instruction {
            Instruction::MoveX(val) => {
                ypos += val;
                xpos += val * aim;
            },
            Instruction::MoveY(val) => aim += val,
        }
    }
    return format!("{} forward x {} down = {}", xpos, ypos, xpos * ypos);
}

pub struct Day2;

fn parse_instruction(line: Result<String, std::io::Error>) -> Instruction {
    let line = line.unwrap();
    let mut it = line.split(" ");
    let direction = it.next().unwrap();
    let value = it.next().unwrap().parse::<i32>().unwrap();
    match direction {
        "forward" => Instruction::MoveX(value),
        "down" => Instruction::MoveY(value),
        "up" => Instruction::MoveY(-value),
        _ => panic!("unexpected direction")
    }
}

impl Day for Day2 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli) -> Result<(String, String), Box<dyn Error>> {
        let instructions = input.lines().map(parse_instruction).collect();
        Ok((part1(&instructions), part2(&instructions)))
    }
}

