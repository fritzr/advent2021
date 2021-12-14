use std::io::BufRead;
use crate::{cli, Day, PartResult, util, util::vec2d::Vec2d};
use std::error::Error;

pub struct Day11;

fn enumerate_adjacent<T>(vec2d: &Vec2d<T>, (row, col): (usize, usize))
    -> impl Iterator<Item = (usize, usize)>
{
    vec2d.enumerate_box(
        (row.saturating_sub(1), col.saturating_sub(1))
        ..
        (row.saturating_add(2), col.saturating_add(2))
    )
}

// Simulate a step and return the number of flashes.
fn step(octopi: &mut Vec2d<u8>, verbose: bool) -> usize {
    let mut flashes = 0;
    let mut to_flash = Vec::<(usize, usize)>::with_capacity(octopi.len());
    for row in 0..octopi.nrows() {
        for col in 0..octopi.ncols() {
            octopi[(row, col)] += 1;
            if octopi[(row, col)] > 9 {
                to_flash.push((row, col));
                octopi[(row, col)] = 0;
                flashes += 1;
            }
        }
    }
    while let Some((row, col)) = to_flash.pop() {
        if verbose {
            println!("({}, {}) FLASH", row, col);
        }
        enumerate_adjacent(octopi, (row, col)).for_each(|(row, col)| {
            if let Some(energy) = octopi.at_mut((row, col)) {
                if *energy != 0 {
                    if verbose {
                        println!("  ({}, {}) bumping {} => {}", row, col, *energy, *energy + 1);
                    }
                    *energy += 1;
                    if *energy > 9 {
                        if verbose {
                            println!("  queueing flash on ({}, {})", row, col);
                        }
                        to_flash.push((row, col));
                        *energy = 0;
                        flashes += 1;
                    }
                }
            }
        })
    }
    flashes
}

// Simulate N steps and return the number of flashes.
fn simulate(octopi: &mut Vec2d<u8>, steps: usize, verbose: bool) -> usize {
    if verbose {
        println!("Before any steps:\n{}", octopi);
    }
    (0..steps).map(|stepnum| {
        let flashes = step(octopi, verbose);
        if verbose {
            println!("After step {}:\n{}==> {} flashes\n", stepnum + 1, octopi, flashes);
        }
        flashes
    }).sum()
}

// After how many steps do the octopi sync (all flash simultaneously)?
fn sync(octopi: &mut Vec2d<u8>, verbose: bool) -> usize {
    let mut steps = 1;
    while step(octopi, false) != octopi.len() {
        steps += 1;
    }
    if verbose {
        println!("Synchronized after {} steps:\n{}\n", steps, octopi);
    }
    steps
}

impl Day for Day11 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let mut octopi = util::read_grid(input)?;
        let steps = if let Some(steps) = &opts.args {
            steps.iter().next().ok_or("empty args")?.parse::<usize>()?
        } else {
            100
        };
        Ok((PartResult::from(|| simulate(&mut octopi, steps, opts.verbose)),
            PartResult::from(|| sync(&mut octopi, opts.verbose) + steps)))
    }
}

