use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::collections::{LinkedList, HashMap};

pub struct Day14;

// Make sure each element is an uppercase letter.
fn check_char(b: u8) -> Result<u8, Box<dyn Error>> {
    if b >= b'A' && b <= b'Z' { Ok(b) } else { Err(format!("bad char '{}'", b).into()) }
}

fn read_polymer_rules(input: &mut dyn BufRead)
    -> Result<(LinkedList<u8>, HashMap<u16, u8>), Box<dyn Error>>
{
    let mut lines = input.lines();
    let line = lines.next().ok_or("missing polymer")??;
    let polymer: LinkedList<u8> = line.into_bytes().into_iter().collect();
    lines.next().ok_or("missing blank after polymer")??;
    let rules = lines.map(|line| {
        let line = line?;
        let mut split = line.splitn(2, " -> ").map(|c| c.bytes());
        let mut pair = split.next().ok_or("missing pair")?.into_iter();
        let pair: u16 =
            (u16::from(check_char(pair.next().ok_or("missing left element of pair")?)?) << 8)
            | u16::from(check_char(pair.next().ok_or("missing right element of pair")?)?);
        let insertion: u8 = check_char(split.next().ok_or("missing insertion")?
            .next().ok_or("empty insertion")?)?;
        Ok((pair, insertion))
    }).collect::<Result<HashMap<u16, u8>, Box<dyn Error>>>()?;
    Ok((polymer, rules))
}

fn solve_step(polymer: &mut LinkedList<u8>, rules: &HashMap<u16, u8>, counts: &mut Vec<usize>)
{
    let mut c = polymer.cursor_front_mut();
    while let Some(cur) = c.current() {
        let cur = *cur;
        let next = if let Some(next) = c.peek_next() { *next } else { break };
        if let Some(insertion) = rules.get(&((u16::from(cur) << 8) | u16::from(next))) {
            c.move_next();
            c.insert_before(*insertion);
            counts[usize::from(*insertion - b'A')] += 1;
        } else {
            c.move_next();
        }
    }
}

// Returns ((lce, count(lce)), (mce, count(mce)))
// for the least common element (lce) and most common element (mce).
// Updates the polymer in-place.
fn solve(polymer: &mut LinkedList<u8>, rules: &HashMap<u16, u8>, steps: usize)
    -> (usize, usize)
{
    let mut counts: Vec<usize> = std::iter::repeat(0).take(usize::from(b'Z' - b'A')).collect();
    for e in polymer.iter() {
        counts[usize::from(*e - b'A')] += 1;
    }
    for _ in 0..steps {
        solve_step(polymer, rules, &mut counts);
    }
    counts.into_iter().fold((usize::MAX, 0), |extents, count| {
        (if count > 0 { extents.0.min(count) } else { extents.0 }, extents.1.max(count))
    })
}

impl Day for Day14 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let (mut polymer, rules) = read_polymer_rules(input)?;
        Ok((PartResult::from(|| {
                let (min, max) = solve(&mut polymer, &rules, 10);
                max - min
            }),
            PartResult::new()))
    }
}

