use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::fmt::Debug;

use crate::cli;

pub fn read_numbers<T>(filename: &Path) -> Vec<T>
    where T: FromStr,
          <T as FromStr>::Err: Debug
{
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    return reader.lines().map(|s| s.unwrap().parse::<T>().unwrap()).collect();
}

pub fn input_path(opts: &cli::Cli, default: &str) -> PathBuf {
    match &opts.input {
        Some(path) => path.clone(),
        None => Path::new(default).parent().unwrap().join("input.txt"),
    }
}
