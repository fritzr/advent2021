use crate::cli;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use core::i32::MAX;
use std::vec::Vec;

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

pub fn read_numbers(filename: &str) -> Vec<i32> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    return reader.lines().map(|s| s.unwrap().parse().unwrap()).collect();
}

pub fn run(opts: &cli::Cli) {
    if opts.verbose {
        println!("Running day 1...");
    }
    let depths = read_numbers("src/d01/input.txt");
    println!("Number of increases: {}", part1(&depths));
    println!("Number of increases in sliding window: {}", part2(&depths));
}
