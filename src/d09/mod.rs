use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::time::Instant;
use std::collections::{BinaryHeap, HashMap};

pub struct Day9;

fn height_at(map: &Vec<String>, (row, col): (usize, usize)) -> Option<u8> {
    if row < map.len() && col < map[row].len() {
        Some(map[row].as_bytes()[col] - b'0')
    } else {
        None
    }
}

fn up(point: (usize, usize)) -> (usize, usize) { (point.0.overflowing_sub(1).0, point.1) }
fn down(point: (usize, usize)) -> (usize, usize) { (point.0 + 1, point.1) }
fn left(point: (usize, usize)) -> (usize, usize) { (point.0, point.1.overflowing_sub(1).0) }
fn right(point: (usize, usize)) -> (usize, usize) { (point.0, point.1 + 1) }

fn local_min(map: &Vec<String>, point: (usize, usize)) -> Option<u8> {
    let height = height_at(map, point)?;
    if point.0 > 0
        && height_at(map, up(point)).map_or(false, |h| h <= height) {
        None
    } else if point.1 > 0
        && height_at(map, left(point)).map_or(false, |h| h <= height) {
        None
    } else if point.1 + 1 < map[point.0].len()
        && height_at(map, right(point)).map_or(false, |h| h <= height) {
        None
    } else if point.0 + 1 < map.len()
        && height_at(map, down(point)).map_or(false, |h| h <= height) {
        None
    } else {
        Some(height)
    }
}

fn low_points(map: &Vec<String>) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .map(|(row_index, line)| {
            (0..line.len()).filter_map(move |col_index| {
                local_min(&map, (row_index, col_index))
                    .and(Some((row_index, col_index)))
            })
        })
        .flatten()
        .collect()
}

// Love recursion, but this is awful for stack size. TODO flatten this into a smarter loop.
fn basin_size_search(map: &Vec<String>,
                     basin: &mut HashMap<(usize, usize), usize>,
                     prev_point: (usize, usize),
                     point: (usize, usize),
                     verbose: bool,
                     ) -> usize {
    // Stop at '9' or out of bounds.
    if height_at(map, point).map_or(true, |h| h == 9) {
        if verbose { println!("({:2}, {:2}) basin bounds", point.0, point.1); }
        0
    } else {
        // We might already have the sum from this point.
        if let Some(size) = basin.get(&point) {
            if verbose {
                println!(
                    "({:2}, {:2}) already know size from here = {}",
                    point.0, point.1, *size
                );
            }
            0
        } else {
            // Insert a dummy size before we recurse to implement a 'visited' check.
            basin.insert(point, usize::MAX);
            if verbose {
                println!("({:2}, {:2}) looking for basin size...", point.0, point.1);
            }
            // Visit the basin in every direction except the direction we just came from.
            let size = 1 + if up(point) != prev_point {
                basin_size_search(map, basin, point, up(point), verbose)
            } else { if verbose { println!("({:2}, {:2}) came from here, skipping -> 0", point.0, point.1); } 0 }
            + if left(point) != prev_point {
                basin_size_search(map, basin, point, left(point), verbose)
            } else { if verbose { println!("({:2}, {:2}) came from here, skipping -> 0", point.0, point.1); } 0 }
            + if right(point) != prev_point {
                basin_size_search(map, basin, point, right(point), verbose)
            } else { if verbose { println!("({:2}, {:2}) came from here, skipping -> 0", point.0, point.1); } 0 }
            + if down(point) != prev_point {
                basin_size_search(map, basin, point, down(point), verbose)
            } else { if verbose { println!("({:2}, {:2}) came from here, skipping -> 0", point.0, point.1); } 0 }
            ;
            basin.insert(point, size);
            if verbose {
                println!("({:2}, {:2}) found basin size {}", point.0, point.1, size);
            }
            size
        }
    }
}

fn basin_size(map: &Vec<String>, point: (usize, usize), verbose: bool) -> usize {
    let mut basin = HashMap::<(usize, usize), usize>::new();
    let size = basin_size_search(map, &mut basin, point, point, verbose);
    if verbose {
        println!("({:2}, {:2}) ------- done: {:?}", point.0, point.1, basin);
    }
    size
}

fn basin_sizes(map: &Vec<String>, low_points: &Vec<(usize, usize)>, verbose: bool)
    -> BinaryHeap<usize>
{
    low_points.iter().map(|p| basin_size(map, *p, verbose)).collect()
}

impl Day for Day9 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let time = Instant::now();
        let map = input.lines().collect::<Result<Vec<String>, std::io::Error>>()?;
        let low_points = low_points(&map);
        let risk: usize = low_points.iter()
            .map(|(row, col)| 1 + usize::from(height_at(&map, (*row, *col)).unwrap()))
            .sum();
        let part1 = PartResult { answer: risk.to_string(), time: time.elapsed() };
        let time = Instant::now();
        let mut basin_sizes = basin_sizes(&map, &low_points, opts.verbose);
        let prod_basins = IntoIterator::into_iter(
            [basin_sizes.pop(), basin_sizes.pop(), basin_sizes.pop()]
            )
            .fold(1, |p, s| p * s.unwrap_or(1));
        let part2 = PartResult { answer: prod_basins.to_string(), time: time.elapsed() };
        Ok((part1, part2))
    }
}

