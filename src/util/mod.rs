use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::cli;

fn input_path(opts: &cli::Cli, mod_path: &str) -> PathBuf {
    match &opts.input {
        Some(path) => path.clone(),
        // src/dXX/mod.rs -> src/dXX/input.txt
        None => Path::new(mod_path).parent().unwrap().join("input.txt"),
    }
}

pub fn read_input(opts: &cli::Cli, mod_path: &str) -> Result<Box<dyn BufRead>, std::io::Error> {
    Ok(Box::new(BufReader::new(File::open(input_path(opts, mod_path))?)))
}
