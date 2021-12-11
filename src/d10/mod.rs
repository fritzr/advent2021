use std::io::BufRead;
use crate::{cli, Day, PartResult, util};
use std::error::Error;
use std::time::{Instant, Duration};

pub struct Day10;

fn score_for(mismatch: u8) -> usize {
    match mismatch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => panic!("no score for '{}'", char::from(mismatch)),
    }
}

fn opener_for(closer: u8) -> u8 {
    match closer {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => panic!("no opener for '{}'", char::from(closer)),
    }
}

fn parse(input: &mut dyn BufRead, verbose: bool) -> Result<(usize, usize), Box<dyn Error>> {
    let mut corrupt_score = 0;
    let mut autocomplete_scores = Vec::<usize>::with_capacity(90);
    for line in input.lines() {
        let mut stack = Vec::<u8>::with_capacity(128);
        let mut corrupted = false;
        'line: for ch in line?.into_bytes() {
            match ch {
                b'(' | b'[' | b'{' | b'<' => stack.push(ch),
                b')' | b']' | b'}' | b'>' => {
                    if let Some(top) = stack.pop() {
                        // corrupted line, score the bad token
                        if top != opener_for(ch) {
                            let this_score = score_for(ch);
                            corrupt_score += this_score;
                            if verbose {
                                println!(
                                    "Expected '{}', found '{}': worth {} points",
                                    char::from(opener_for(ch)), char::from(top), this_score
                                );
                            }
                            corrupted = true;
                            break 'line;
                        }
                    } else {
                        panic!("Unexpected '{}'", char::from(ch));
                    }
                }
                _ => (),
            }
        }
        // incomplete line, autocomplete and score
        if !corrupted && stack.len() != 0 {
            let score = stack.iter().rev().fold(0, |score, opener| score * 5 + score_for(*opener));
            if verbose {
                println!("score {:>12} for '{:?}'", score,
                         stack.iter().map(|b| char::from(*b)).collect::<String>());
            }
            autocomplete_scores.push(score);
        }
    }
    autocomplete_scores.sort();
    Ok((corrupt_score, *util::median(&autocomplete_scores)))
}

impl Day for Day10 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let time = Instant::now();
        let (score1, score2) = parse(input, opts.verbose)?;
        let time = time.elapsed();
        Ok((PartResult { answer: score1.to_string(), time: time },
            PartResult { answer: score2.to_string(), time: Duration::new(0, 0) }))
    }
}

