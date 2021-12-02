use std::io::BufRead;
use std::error::Error;

use crate::cli;
use crate::Day;

enum Instruction {
    MoveX(i32),
    MoveY(i32),
}

fn part1(instructions: &Vec<Instruction>, verbose: bool) -> i32 {
    // (x, y)
    let pos = instructions.iter().fold((0,0), |pos, instruction| match instruction {
        Instruction::MoveX(val) => (pos.0 + val, pos.1),
        Instruction::MoveY(val) => (pos.0, pos.1 + val),
    });
    if verbose {
        println!("{} forward x {} down", pos.0, pos.1);
    }
    pos.0 * pos.1
}

fn part2(instructions: &Vec<Instruction>, verbose: bool) -> i32 {
    // (x, y, aim)
    let pos = instructions.iter().fold((0, 0, 0), |vec, instruction| match instruction {
        Instruction::MoveX(val) => { (vec.0 + val * vec.2, vec.1 + val, vec.2) }
        Instruction::MoveY(val) => { (vec.0, vec.1, vec.2 + val) },
    });
    if verbose {
        println!("{} forward x {} down ({} aim)", pos.0, pos.1, pos.2);
    }
    pos.0 * pos.1
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
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(String, String), Box<dyn Error>>
    {
        let instructions = input.lines().map(parse_instruction).collect();
        Ok((part1(&instructions, opts.verbose).to_string(),
            part2(&instructions, opts.verbose).to_string()))
    }
}

