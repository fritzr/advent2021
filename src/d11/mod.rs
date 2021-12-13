use std::io::BufRead;
use crate::{cli, Day, PartResult, util, util::vec2d::Vec2d};
use std::error::Error;

pub struct Day11;

struct Square {
    cur:    (usize, usize),
    bounds: (usize, usize),
    end:    usize,
}

impl Square {
    fn adjacent(row: usize, col: usize) -> Square {
        Square {
            cur:    (row.saturating_sub(1), col.saturating_sub(1)),
            bounds: (col.saturating_sub(1), col.saturating_add(1)),
            end:    row.saturating_add(1),
        }
    }
}

impl Iterator for Square {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let this = self.cur;
        if self.cur.0 > self.end {
            None
        } else {
            if self.cur.1 == self.bounds.1 {
                self.cur = (self.cur.0 + 1, self.bounds.0);
            } else {
                self.cur = (self.cur.0, self.cur.1 + 1);
            }
            Some(this)
        }
    }
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
        Square::adjacent(row, col).for_each(|(row, col)| {
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

