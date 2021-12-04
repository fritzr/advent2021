use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::time::Instant;

pub struct Day3;

fn part1(input: &String) -> (u32, u32) {
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
    (gamma, epsilon)
}

/*
                1*0     + 1               0   place=1    bit=0
          + 2*1         + 2 *             1   place=2    bit=1
        +4*1            + 2 * 2 *         1   place=4    bit=1
    +8*0                + 2 * 2 * 2 *     0   place=8    bit=0
+16*1                   + 2 * 2 * 2 * 2 * 1   place=16   bit=1
*/

fn part2(_input: &String, _gamma: u32, _epsilon: u32) -> (u32, u32) {
    (0, 0)
}

impl Day for Day3 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let mut string = String::with_capacity(13000);
        input.read_to_string(&mut string)?;
        let tick = Instant::now();
        let (gamma, epsilon) = part1(&string);
        let p1_time = tick.elapsed();
        if opts.verbose {
            println!("    gamma = {}, epsilon = {}", gamma, epsilon);
        }
        let tick = Instant::now();
        let (oxy, co2) = part2(&string, gamma, epsilon);
        let p2_time = tick.elapsed();
        if opts.verbose {
            println!("    oxy rating = {}, co2 rating = {}", oxy, co2);
        }
        Ok((PartResult { answer: (gamma * epsilon).to_string(), time: p1_time },
            PartResult { answer: (oxy * co2).to_string(), time: p2_time }))
    }
}

