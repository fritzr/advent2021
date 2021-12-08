use std::io::BufRead;
use crate::{cli, Day, PartResult, util};
use std::error::Error;

pub struct Day7;

// minimize cost of linear distance
fn median(input: &Vec<usize>) -> usize {
    // assert_eq!(input.is_sorted(), true);
    return input[input.len() / 2];
}

fn mean(input: &Vec<usize>) -> usize {
    return input.iter().sum::<usize>() / input.len();
}

fn distance(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

fn geometric_distance(a: usize, b: usize) -> usize {
    let n = distance(a, b);
    (n * (n + 1)) / 2
}

fn cost<F>(positions: &Vec<usize>, midpoint: usize, cost: F) -> usize
    where F: Fn(usize, usize) -> usize
{
    positions.iter().map(|s| cost(*s, midpoint)).sum()
}

impl Day for Day7 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let mut input = util::read_csv(input)?;
        input.sort();
        Ok((PartResult::from(|| {
                let position = median(&input);
                if opts.verbose {
                    println!("  optimal linear position is {}", position);
                }
                cost(&input, position, distance)
            }),
            PartResult::from(|| {
                let position = mean(&input);
                if opts.verbose {
                    println!("  optimal geometric position is {}", position);
                }
                cost(&input, position, geometric_distance)
            }),
        ))
    }
}

