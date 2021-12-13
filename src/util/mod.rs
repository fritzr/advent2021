use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::mem::swap;
use std::error::Error;

use crate::cli;

pub mod vec2d;

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

// true for a surprising number of days
const INPUT_LEN_GUESS: usize = 1000;

// Read one comma-separated line from the input.
pub fn read_csv(input: &mut dyn BufRead) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let mut input = Vec::<usize>::with_capacity(INPUT_LEN_GUESS);
    for num_string in buffer.split(",") {
        input.push(num_string.trim().parse()?);
    }
    Ok(input)
}

pub struct SplitGroup<'a> {
    reader: &'a mut dyn BufRead,
    buffer: String,
}

impl Iterator for SplitGroup<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        let mut bytes_read = 0;
        while let Some(count) = self.reader.read_line(&mut self.buffer).ok() {
            bytes_read = count;
            if count == 1 {
                self.buffer.pop().unwrap();
            }
            if count <= 1 {
                break
            }
        }
        if bytes_read > 0 {
            let mut new_buffer = String::new();
            swap(&mut new_buffer, &mut self.buffer);
            Some(new_buffer)
        } else {
            None
        }
    }
}

pub fn split_groups(input: &mut dyn BufRead) -> SplitGroup {
    SplitGroup { reader: input, buffer: String::new() }
}

pub fn median<T>(input: &Vec<T>) -> &T {
    // assert_eq!(input.is_sorted(), true);
    return &input[input.len() / 2];
}

pub fn read_grid(input: &mut dyn BufRead) -> Result<vec2d::Vec2d<u8>, Box<dyn Error>> {
    let mut lines = input.lines().peekable();
    let width = lines.peek().ok_or("empty input")?.as_ref().map_err(|e| e.to_string())?.len();
    Ok(lines.flat_map(|s| s.expect("EOF").into_bytes())
       .map(|b| b - b'0')
       .collect::<vec2d::Vec2d<u8>>()
       .reshaped_from(|_, len| (len / width, width))?)
}
