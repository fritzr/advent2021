use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::collections::HashMap;

pub struct Day14;

// Make sure each element is an uppercase letter.
fn check_char(b: u8) -> Result<u8, Box<dyn Error>> {
    if b >= b'A' && b <= b'Z' { Ok(b) } else { Err(format!("bad char '{}'", b).into()) }
}

fn polypair(p1: u8, p2: u8) -> u16 {
    (u16::from(p1) << 8) | u16::from(p2)
}

fn pairsplit(p: u16) -> (u8, u8) {
    (((p >> 8) & 0xffu16) as u8, (p & 0xffu16) as u8)
}

fn read_polymer_rules(input: &mut dyn BufRead)
    -> Result<(String, HashMap<u16, u8>), Box<dyn Error>>
{
    let mut lines = input.lines();
    let polymer = lines.next().ok_or("missing polymer")??;
    lines.next().ok_or("missing blank after polymer")??;
    let rules = lines.map(|line| {
        let line = line?;
        let mut split = line.splitn(2, " -> ").map(|c| c.bytes());
        let mut pair = split.next().ok_or("missing pair")?.into_iter();
        let pair: u16 = polypair(
            check_char(pair.next().ok_or("missing left element of pair")?)?,
            check_char(pair.next().ok_or("missing right element of pair")?)?);
        let insertion: u8 = check_char(split.next().ok_or("missing insertion")?
            .next().ok_or("empty insertion")?)?;
        Ok((pair, insertion))
    }).collect::<Result<HashMap<u16, u8>, Box<dyn Error>>>()?;
    Ok((polymer, rules))
}

fn dispchars(chars: &Vec<usize>) -> String {
    let mut s = String::with_capacity(2 + 26 * (2 + 5 + 2));
    s += "[";
    let mut it = chars.iter().enumerate();
    if let Some((byte, count)) = it.next() {
        s += format!("{}: {}",
            char::from_u32((byte + (b'A' as usize)) as u32).unwrap(),
            count).as_str();
    }
    for (byte, count) in it {
        s += format!(", {}: {}",
            char::from_u32((byte + (b'A' as usize)) as u32).unwrap(),
            count).as_str();
    }
    s += "]";
    s
}

fn disppairs<T>(pairs: &HashMap<u16, T>) -> String
    where T: std::fmt::Display
{
    let mut s = String::with_capacity(2 * 26 * (4 + 2 + 5));
    s += "{";
    let mut it = pairs.iter();
    if let Some((pair, count)) = it.next() {
        let (a, b) = pairsplit(*pair);
        s += format!("{}{}: {}",
            char::from_u32(u32::from(a)).unwrap(),
            char::from_u32(u32::from(b)).unwrap(),
            count).as_str();
    }
    for (pair, count) in it {
        let (a, b) = pairsplit(*pair);
        s += format!(", {}{}: {}",
            char::from_u32(u32::from(a)).unwrap(),
            char::from_u32(u32::from(b)).unwrap(),
            count).as_str();
    }
    s += "}";
    s
}

// Count the numbers of each character after n solution steps.
//
// Return the (least, most) frequent counts.
fn polymer_counts(polymer: &String, rules: &HashMap<u16, u8>, steps: usize, verbose: bool)
    -> (usize, usize)
{
    let span = usize::from(1 + b'Z' - b'A');
    /* With a vector instead of a hash map, we need to iterate through all possible pairs
     * each iteration, not just the ones which are nonzero. Let's check later which is faster.
    let mut pairs: Vec<usize> = std::iter::repeat(0).take(span).collect();
    fn ipair(p: u16) -> usize {
        let (a, b) = pairsplit(p);
        a * span + b
    }
    */
    let mut pairs: HashMap<u16, usize> = HashMap::with_capacity(span * span);
    let mut chars: Vec<usize> = std::iter::repeat(0).take(span).collect();
    #[inline]
    fn ichar(b: u8) -> usize { usize::from(b - b'A') }
    for pair in polymer.as_bytes().windows(2) {
        chars[ichar(pair[0])] += 1;
        chars[ichar(pair[1])] += 1;
        *pairs.entry(polypair(pair[0], pair[1])).or_insert(0) += 1;
    }
    if verbose {
        println!("To begin, counts are:\n  [chars] {}\n  [pairs] {}",
                 dispchars(&chars), disppairs(&pairs));
    }
    for step in 0..steps {
        // Apply all updates in one step.
        let mut pair_updates: HashMap<u16, isize> = HashMap::with_capacity(span * span);
        for (pair, count) in &pairs {
            let (pair, count) = (*pair, *count);
            if let Some(new) = rules.get(&pair) {
                let new = *new;
                let (left, right) = pairsplit(pair);
                // Replace the pair with two new pairs.
                *pair_updates.entry(pair).or_insert(0) -= count as isize;
                *pair_updates.entry(polypair(left, new)).or_insert(0) += count as isize;
                *pair_updates.entry(polypair(new, right)).or_insert(0) += count as isize;
                chars[ichar(new)] += count;
            }
        }
        for (pair, inserted) in pair_updates {
            let count = pairs.entry(pair).or_insert(0);
            if inserted > 0 {
                *count += inserted as usize;
            } else {
                *count -= (-inserted) as usize;
            }
        }
        if verbose {
            println!("After step {}:\n  [chars] {}\n  [pairs] {}",
                     step + 1, dispchars(&chars), disppairs(&pairs));
        }
    }
    chars.into_iter().fold((usize::MAX, 0), |(min, max), count| {
        (if count > 0 { min.min(count) } else { min }, max.max(count))
    })
}

impl Day for Day14 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let (polymer, rules) = read_polymer_rules(input)?;
        if opts.verbose {
            println!("polymer: {}\nrules: {}\n", polymer, disppairs(&rules));
        }
        Ok((PartResult::from(|| {
                let (min, max) = polymer_counts(&polymer, &rules, 10, opts.verbose);
                max - min
            }),
            PartResult::from(|| {
                let (min, max) = polymer_counts(&polymer, &rules, 40, opts.verbose);
                max - min
            })))
    }
}

