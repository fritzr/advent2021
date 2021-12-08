use std::io::BufRead;
use crate::{cli, Day, PartResult, util};
use std::error::Error;

pub struct Day7;

fn median(input: &Vec<usize>) -> usize {
    // assert_eq!(input.is_sorted(), true);
    return input[input.len() / 2];
}

fn linear_cost(positions: &Vec<usize>, midpoint: usize) -> usize {
    positions.iter().map(|s| if s > &midpoint { s - midpoint } else { midpoint - s }).sum()
}

impl Day for Day7 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let mut input = util::read_csv(input)?;
        input.sort();
        Ok((PartResult::from(|| linear_cost(&input, median(&input))),
            PartResult::new()))
    }
}

