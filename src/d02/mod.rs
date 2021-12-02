use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

use crate::cli;
use crate::Day;
use crate::util;

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
    if direction == "forward" {
        Instruction::MoveX(value)
    }
    else if direction == "down" {
        Instruction::MoveY(value)
    }
    else if direction == "up" {
        Instruction::MoveY(-value)
    }
    else {
        panic!("unexpected direction");
    }
}

fn read_instructions(filename: &Path) -> Vec<Instruction> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    return reader.lines().map(parse_instruction).collect();
}

impl Day for Day2 {
    fn run(&self, opts: &cli::Cli) -> Result<(String, String), Box<dyn Error>> {
        let nums = read_instructions(util::input_path(opts, file!()).as_path());
        Ok((part1(&nums).to_string(), part2(&nums).to_string()))
    }
}

