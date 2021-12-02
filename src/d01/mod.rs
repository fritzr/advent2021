use core::i32::MAX;
use std::io::BufRead;
use std::error::Error;

use crate::cli;
use crate::Day;

pub fn part1(depths: &Vec<i32>) -> i32 {
    let mut last_depth = MAX;
    let mut increases = 0;
    for depth in depths {
        if depth > &last_depth {
            increases += 1;
        }
        last_depth = *depth;
    }
    increases
}

pub fn part2(depths: &Vec<i32>) -> i32 {
    let mut increases = 0;
    for index in 3..depths.len() {
        if depths[index] > depths[index - 3] {
            increases += 1;
        }
    }
    increases
}

pub struct Day1;

impl Day for Day1 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli) -> Result<(String, String), Box<dyn Error>> {
        let depths = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();
        Ok((part1(&depths).to_string(), part2(&depths).to_string()))
    }
}

