use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;

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

fn low_points_risk(input: &mut dyn BufRead) -> Result<usize, Box<dyn Error>> {
    let mut risk = 0;
    let map = input.lines().collect::<Result<Vec<String>, std::io::Error>>()?;
    for (row_index, line) in map.iter().enumerate() {
        for col_index in 0..line.len() {
            if let Some(height) = local_min(&map, row_index, col_index) {
                risk += 1 + usize::from(height);
            }
        }
    }
    Ok(risk)
}

impl Day for Day9 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        Ok((PartResult::maybe_from(|| low_points_risk(input))?,
            PartResult::new()))
    }
}

