use std::io::BufRead;
use std::error::Error;

use crate::cli;
use crate::{Day, PartResult};

pub fn part1(depths: &Vec<i32>) -> i32 {
    depths.windows(2).filter_map(|w| if w[1] > w[0] { Some(1) } else { None }).sum()
}

pub fn part2(depths: &Vec<i32>) -> i32 {
    //    d[n] + d[n-1] + d[n-2] > d[n-3] + d[n-2] + d[n-1]
    // => d[n] > d[n-3]
    depths.windows(4).filter_map(|w| if w[3] > w[0] { Some(1) } else { None }).sum()
}

pub struct Day1;

impl Day for Day1 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli) -> Result<(PartResult, PartResult), Box<dyn Error>> {
        let depths = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();
        Ok((PartResult::from(|| part1(&depths)),
            PartResult::from(|| part2(&depths))))
    }
}

