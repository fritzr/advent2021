use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Add;

pub struct Day8;

const INPUT_LEN_GUESS: usize = 200;

// one segment
#[derive(Debug, Clone, Copy)]
struct Segment(u64);

const A: u64 = 1 << 0;
const B: u64 = 1 << 8;
const C: u64 = 1 << 16;
const D: u64 = 1 << 24;
const E: u64 = 1 << 32;
const F: u64 = 1 << 40;
const G: u64 = 1 << 48;

impl Segment {
    fn from(character: u8) -> Option<Segment> {
        match character {
            b'a' => Some(Segment(A)),
            b'b' => Some(Segment(B)),
            b'c' => Some(Segment(C)),
            b'd' => Some(Segment(D)),
            b'e' => Some(Segment(E)),
            b'f' => Some(Segment(F)),
            b'g' => Some(Segment(G)),
            _ => None,
        }
    }

    fn from_id(id: usize) -> Segment {
        assert_eq!(id < 7, true);
        Segment(1 << (id * 8))
    }

    fn id(&self) -> usize {
        match self.0 {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            E => 4,
            F => 5,
            G => 6,
            _ => panic!("bad value for Segment"),
        }
    }

    fn to_string(&self) -> String {
        match self.0 {
            A => "a".into(),
            B => "b".into(),
            C => "c".into(),
            D => "d".into(),
            E => "e".into(),
            F => "f".into(),
            G => "g".into(),
            _ => "?".into(),
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "'{}'", self.to_string())
    }
}

impl Add for Segment {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Segment(self.0 + other.0)
    }
}

#[derive(Debug, Clone, Copy)]
struct Digit(u64);

const ZERO:  u64 = A + B + C     + E + F + G; // N=6
const ONE:   u64 =         C         + F    ; // N=2*
const TWO:   u64 = A     + C + D + E     + G; // N=5
const THREE: u64 = A     + C + D     + F + G; // N=5
const FOUR:  u64 =     B + C + D     + F    ; // N=4*
const FIVE:  u64 = A + B     + D     + F + G; // N=5
const SIX:   u64 = A + B     + D + E + F + G; // N=6
const SEVEN: u64 = A     + C         + F    ; // N=3*
const EIGHT: u64 = A + B + C + D + E + F + G; // N=7*
const NINE:  u64 = A + B + C + D + F + G    ; // N=6

struct Segments {
    index: usize,
    digit: u64,
}

impl Iterator for Segments {
    type Item = Segment;
    fn next(&mut self) -> Option<Self::Item> {
        let mut mask = 0xff << (self.index * 8);
        while self.index < 7 && (self.digit & mask) == 0 {
            self.index += 1;
            mask <<= 8;
        }
        if self.index == 7 {
            None
        } else {
            let segment = Segment(self.digit & mask);
            self.index += 1;
            Some(segment)
        }
    }
}

impl Digit {
    fn from(bytes: &[u8]) -> Result<Digit, Box<dyn Error>> {
        let mut d = 0;
        for segment in bytes.iter().map(|b| Segment::from(*b)) {
            d |= segment.ok_or("bad segment character")?.0;
        }
        Ok(Digit(d))
    }
    fn from_segments<I>(it: I) -> Digit
        where I: IntoIterator<Item=Segment>
    {
        Digit(it.into_iter().fold(0, |acc, seg| acc | seg.0))
    }
    // Get an iterator over the segments in the digit.
    fn segments(&self) -> Segments {
        Segments { index: 0, digit: self.0 }
    }
    // Get the value (0-9) this 7-segment digit represents.
    fn value(&self) -> u8 {
        match self.0 {
            ZERO  => 0,
            ONE   => 1,
            TWO   => 2,
            THREE => 3,
            FOUR  => 4,
            FIVE  => 5,
            SIX   => 6,
            SEVEN => 7,
            EIGHT => 8,
            NINE  => 9,
            _     => panic!  ("bad value for Digit: {:056b}", self.0),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "  {:056b}", self.0)
    }
}

#[derive(Debug, Clone)]
struct SegDisplay(Vec<Digit>);

impl SegDisplay {
    fn from(line: String) -> Result<SegDisplay, Box<dyn Error>> {
        let mut vec = Vec::with_capacity(10);
        for word in line.split(" ") {
            vec.push(Digit::from(word.trim().as_bytes())?);
        }
        Ok(SegDisplay(vec))
    }
}

impl Display for SegDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "SegDisplay([")?;
        for digit in &self.0 {
            write!(f, "  {}", digit)?;
        }
        write!(f, "  ])")?;
        Ok(())
    }
}

fn read_displays(input: &mut dyn BufRead)
    -> Result<(Vec<SegDisplay>, Vec<SegDisplay>), Box<dyn Error>>
{
    let mut inputs = Vec::<SegDisplay>::with_capacity(INPUT_LEN_GUESS);
    let mut outputs = Vec::<SegDisplay>::with_capacity(INPUT_LEN_GUESS);
    for line in input.lines() {
        let line = line?;
        let mut parts = line.split(" | ");
        inputs.push(
            SegDisplay::from(parts.next().ok_or("expected ten digit part")?.into())?
        );
        outputs.push(
            SegDisplay::from(parts.next().ok_or("expected output display part")?.into())?
        );
    }
    Ok((inputs, outputs))
}

fn part1(_inputs: &Vec<SegDisplay>, outputs: &Vec<SegDisplay>) -> usize {
    let mut count = 0;
    for output in outputs {
        for digit in &output.0 {
            if 0 != match digit.0.count_ones() {
                2 => ONE,
                3 => SEVEN,
                4 => FOUR,
                7 => EIGHT,
                _ => 0,
            } {
                count += 1;
            }
        }
    }
    count
}

// after some careful analysis, the mapping between signal wires and segments makes sense :-)
//
// Returns a segment decoder D such that D[s] is the segment for the encoded segment s.
fn careful_analysis(digits: &SegDisplay, verbose: bool) -> [usize; 7] {
    assert_eq!(digits.0.len(), 10);
    // The segment numbering is shown on the left.
    // 0  ---      The mapping of segment number to byte position in the u64 is below.
    // 1 |   | 2    ___ ___ ___ ___ ___ ___ ___ ___ ___
    // 3  ---      |_x_|_7_|_6_|_5_|_4_|_3_|_2_|_1_|_0_|
    // 4 |   | 5
    // 6  ---
    let mut seg_codec = [0; 7];
    // Generate counts of number of digits in which each segment appears.
    // Each segment is conveniently laid over disjoint bytes of a u64,
    // so simply summing segments gives the right number.
    // We won't have more than 7 segments so we can't overflow the bytes.
    let counts: u64 = digits.0.iter().map(|digit| digit.0).sum();
    // Now we have assignments for segment 1, 4, and 5, which uniquely appear 6, 4, and 9 times.
    {
        let mut counts = counts;
        for index in 0..7 {
            match counts & 0xff {
                6 => {
                    seg_codec[index] = 1;
                    if verbose {
                        println!(
                           "  segment {} has {} occurrences, so it must be segment {}",
                           Segment::from_id(index).to_string(), 6, 1,
                       );
                    }
                }, // segment 1 appears in 0,4,5,6,8,9
                4 =>  {
                    seg_codec[index] = 4; // segment 4 appears in 0,2,6,8
                    if verbose {
                        println!(
                           "  segment {} has {} occurrences, so it must be segment {}",
                           Segment::from_id(index).to_string(), 4, 4,
                       );
                    }
                },
                9 => {
                    seg_codec[index] = 5; // segment 5 appears in 0,1,3,4,5,6,7,8,9
                    if verbose {
                        println!(
                           "  segment {} has {} occurrences, so it must be segment {}",
                           Segment::from_id(index).to_string(), 9, 5,
                       );
                    }
                },
                _ => (),
            }
            counts >>= 8;
        }
    }
    // Now identify the numbers we know to be 1 (2 segs), 4 (4 segs), 7 (3 segs), or 8 (7 segs).
    // While we're at it, repeat the segment counts without these numbers.
    // This identifies segments 2 and 3, which have counts 4 and 5.
    let mut digit_seven = 0;
    let counts: u64 = digits.0.iter().filter_map(|digit| {
        match digit.0.count_ones() {
            2 => {
                if verbose {
                    println!(
                        "  digit {} has {} segments, so must be digit {}",
                        digit, 2, 1,
                    );
                }
                None
            },
            4 => {
                if verbose {
                    println!(
                        "  digit {} has {} segments, so must be digit {}",
                        digit, 4, 4,
                    );
                }
                None
            },
            3 => {
                if verbose {
                    println!(
                        "  digit {} has {} segments, so must be digit {}",
                        digit, 3, 7,
                    );
                }
                digit_seven = digit.0;
                None
            },
            7 => {
                if verbose {
                    println!(
                        "  digit {} has {} segments, so must be digit {}",
                        digit, 7, 8,
                    );
                }
                None
            },
            _ => Some(digit.0),
        }
    }).sum();
    // Now we know the assignments for segments 2 and 3, which have counts 4 and 5.
    {
        let mut counts = counts;
        for index in 0..7 {
            // Skip segments we already identified (1, 4, 5)
            if seg_codec[index] == 0 {
                match counts & 0xff {
                    4 => {
                        seg_codec[index] = 2; // segment 2 appears in 0,2,3,9 (ignoring 1,4,7,8)
                        if verbose {
                            println!(
                               "  segment {} has {} occurrences, so it must be segment {}",
                               Segment::from_id(index).to_string(), 4, 2,
                           );
                        }
                    },
                    5 => {
                        seg_codec[index] = 3; // segment 3 appears in 2,3,5,6,9 (ignoring 1,4,7,8)
                        if verbose {
                            println!(
                               "  segment {} has {} occurrences, so it must be segment {}",
                               Segment::from_id(index).to_string(), 5, 3,
                           );
                        }
                    },
                    _ => (),
                }
            }
            counts >>= 8;
        }
    }
    // Only segments 0 and 6 remain unassigned.
    // Segment 0 appears in digit 7 along with 2 and 5, which we've already decoded.
    // The unassigned segment in this position must be segment 0, and the last remaining
    // unassigned segment must be segment 6.
    assert_ne!(digit_seven, 0);
    for (index, decoded_seg) in seg_codec.iter().enumerate() {
        let shift = index * 8;
        if verbose {
            let seg = Segment::from_id(index);
            println!("  testing segment {} ({}, {:056b}) (known to be {}) against digit 7 ({:056b})",
                seg, index, seg.0, *decoded_seg, digit_seven);
            println!("  shift and segment are {}, {}", shift, (digit_seven >> shift) & 0xff);
        }
        if *decoded_seg == 0 && ((digit_seven >> shift) & 0xff) == 0 {
            seg_codec[index] = 6;
            if verbose {
                println!(
                   "  unidentified segment {} does not belong to digit 7 ({:056b}) \
                   , so it must be segment {}",
                   Segment::from_id(index).to_string(), digit_seven, 6,
               );
            }
            break;
        }
        // Don't need to explicitly assign 0 in the codec, because all slots are 0 by default.
        // But we are done once we've found the slot for 6.
    }
    if verbose {
        println!("  final segment map: {:?}", seg_codec);
    }
    // Done!
    seg_codec
}

fn unscramble_outputs(inputs: &Vec<SegDisplay>, outputs: &Vec<SegDisplay>, verbose: bool)
    -> Vec<usize>
{
    let mut results = Vec::<usize>::with_capacity(outputs.len());
    for (digits, output) in inputs.iter().zip(outputs.iter()) {
        let seg_codec = careful_analysis(digits, verbose);
        results.push(output.0.iter().fold(0, |num, digit| {
            let digit_value = Digit::from_segments(
                digit.segments().map(|seg| Segment::from_id(seg_codec[seg.id()]))
            ).value();
            num * 10 + usize::from(digit_value)
        }));
        if verbose {
            println!("decoded digits {:?}", results[results.len()-1]);
        }
    }
    results
}

impl Day for Day8 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let (display_sets, outputs) = read_displays(input)?;
        if opts.verbose {
            println!("display sets:");
            for display_set in &display_sets {
                println!("  {}", display_set);
            }
            println!();
            println!("outputs:");
            for output in &outputs {
                println!("  {}", output);
            }
        }
        Ok((PartResult::from(|| part1(&display_sets, &outputs)),
            PartResult::from(|| unscramble_outputs(&display_sets, &outputs, opts.verbose)
                                 .into_iter()
                                 .sum::<usize>())))
    }
}

