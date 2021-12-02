use core::i32::MAX;
use std::vec::Vec;
use std::error::Error;

use crate::cli;
use crate::Day;
use crate::util;

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
    fn run(&self, opts: &cli::Cli) -> Result<(String, String), Box<dyn Error>> {
        let depths = util::read_numbers(util::input_path(opts, file!()).as_path());
        Ok((part1(&depths).to_string(), part2(&depths).to_string()))
    }
}

