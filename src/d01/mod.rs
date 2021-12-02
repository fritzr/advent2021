use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use core::i32::MAX;
use std::vec::Vec;
use std::error::Error;
use std::path::Path;

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

pub fn read_numbers(filename: &Path) -> Vec<i32> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    return reader.lines().map(|s| s.unwrap().parse().unwrap()).collect();
}

pub struct Day1;

impl Day for Day1 {
    fn run(&self, _opts: &cli::Cli) -> Result<(String, String), Box<dyn Error>> {
        let depths = read_numbers(&Path::new(file!()).parent().unwrap().join("input.txt"));
        Ok((part1(&depths).to_string(), part2(&depths).to_string()))
    }
}

