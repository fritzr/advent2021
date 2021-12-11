use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::time::{Instant};

pub struct Day9;

fn height_at(map: &Vec<String>, row: usize, col: usize) -> Option<u8> {
    if row < map.len() && col < map[row].len() {
        Some(map[row].as_bytes()[col] - b'0')
    } else {
        None
    }
}

fn local_min(map: &Vec<String>, row: usize, col: usize) -> Option<u8> {
    let height = height_at(map, row, col)?;
    if row > 0
            && height_at(map, row - 1, col).map_or(false, |h| h <= height) {
        None
    } else if col > 0
            && height_at(map, row, col - 1).map_or(false, |h| h <= height) {
        None
    } else if col + 1 < map[row].len()
            && height_at(map, row, col + 1).map_or(false, |h| h <= height) {
        None
    } else if row + 1 < map.len()
            && height_at(map, row + 1, col).map_or(false, |h| h <= height) {
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
                local_min(&map, row_index, col_index)
                    .and(Some((row_index, col_index)))
            })
        })
        .flatten()
        .collect()
}

impl Day for Day9 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let time = Instant::now();
        let map = input.lines().collect::<Result<Vec<String>, std::io::Error>>()?;
        let low_points = low_points(&map);
        let risk: usize = low_points.iter()
            .map(|(row, col)| 1 + usize::from(height_at(&map, *row, *col).unwrap()))
            .sum();
        let time = time.elapsed();
        Ok((PartResult { answer: risk.to_string(), time }, PartResult::new()))
    }
}

