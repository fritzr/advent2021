use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::time::Instant;

pub struct Day3;

fn part1(input: &String) -> (u32, u32, usize) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let bit_width = first_line.len();
    let sums: Vec<u32> = vec![0; bit_width];
    let (num_lines, sums) = IntoIterator::into_iter([first_line; 1])
        .chain(lines)
        .fold((0, sums), |(num_lines, sums), line|
            (num_lines + 1,
                line
                .as_bytes()
                .into_iter()
                .enumerate()
                .map(|(index, chr)| sums[index].saturating_add((chr - b'0').into()))
                .collect()
            )
    );
    let gamma = sums.into_iter().rfold((1, 0), |(place, decimal), sum|
       (place*2, decimal + place * { if sum > num_lines / 2 { 1 } else { 0 } })
    ).1;
    let mask = (1 << bit_width) - 1;
    let epsilon = mask & !gamma;
    (gamma, epsilon, bit_width)
}

fn collect_rating<F>(mut valid: Vec<u32>, mut msb: u32, decide: F) -> u32
    where F: Fn(Vec<u32>, Vec<u32>) -> Vec<u32>
{
    while msb > 0 && valid.len() > 1 {
        let (ones, zeros): (Vec<u32>, Vec<u32>) = valid
            .into_iter()
            .partition(|x| x & msb == msb);
        valid = decide(ones, zeros);
        msb >>= 1;
    }
    assert_eq!(valid.len(), 1);
    valid[0]
}

fn part2(input: &String, width: usize) -> (u32, u32) {
    let input: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    let msb: u32 = 1 << (width - 1);

    let oxy = collect_rating(input.clone(), msb, |ones, zeros|
        if ones.len() >= zeros.len() { ones } else { zeros });

    let co2 = collect_rating(input, msb, |ones, zeros|
        if zeros.len() <= ones.len() { zeros } else { ones });

    (oxy, co2)
}

impl Day for Day3 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let mut string = String::with_capacity(13000);
        input.read_to_string(&mut string)?;
        let tick = Instant::now();
        let (gamma, epsilon, bit_width) = part1(&string);
        let p1_time = tick.elapsed();
        if opts.verbose {
            println!("    gamma = {0} ({0:b}), epsilon = {1} ({1:b})", gamma, epsilon);
        }
        let tick = Instant::now();
        let (oxy, co2) = part2(&string, bit_width);
        let p2_time = tick.elapsed();
        if opts.verbose {
            println!("    oxy rating = {0} ({0:b}), co2 rating = {1} ({1:b})", oxy, co2);
        }
        Ok((PartResult { answer: (gamma * epsilon).to_string(), time: p1_time },
            PartResult { answer: (oxy * co2).to_string(), time: p2_time }))
    }
}

