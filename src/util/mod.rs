use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::mem::swap;

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

