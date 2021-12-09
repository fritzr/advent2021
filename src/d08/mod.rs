use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct Day8;

const INPUT_LEN_GUESS: usize = 200;

type Segment = u8; // one segment
const A: Segment = 0b01000000;
const B: Segment = 0b00100000;
const C: Segment = 0b00010000;
const D: Segment = 0b00001000;
const E: Segment = 0b00000100;
const F: Segment = 0b00000010;
const G: Segment = 0b00000001;

fn segment(character: u8) -> Option<Segment> {
    match character {
        b'a' => Some(A),
        b'b' => Some(B),
        b'c' => Some(C),
        b'd' => Some(D),
        b'e' => Some(E),
        b'f' => Some(F),
        b'g' => Some(G),
        _ => None,
    }
}

type SevenSeg = u8;

/*
const ZERO:  SevenSeg = A + B + C     + E + F + G; // N=6
const ONE:   SevenSeg =         C         + F;     // N=2*
const TWO:   SevenSeg = A     + C + D + E     + G; // N=5
const THREE: SevenSeg = A     + C + D     + F + G; // N=5
const FOUR:  SevenSeg =     B + C + D     + F;     // N=4*
const FIVE:  SevenSeg = A + B     + D     + F + G; // N=5
const SIX:   SevenSeg = A + B     + D + E + F + G; // N=6
const SEVEN: SevenSeg = A     + C         + F;     // N=3*
const EIGHT: SevenSeg = A + B + C + D + E + F + G; // N=7*
const NINE:  SevenSeg = A + B + C + D + F + G;     // N=6
*/

fn sevensegment(bytes: &[u8]) -> SevenSeg {
    bytes.iter()
        .map(|b| segment(*b))
        .reduce(|seg1, seg2| Some(seg1? | seg2?))
        .expect("not enough bytes for seven segments")
        .expect("bad char in segment")
}

/*
fn sevensegment_to_string(ss: SevenSeg) -> &'static str {
    match ss {
        ZERO  => "0",
        ONE   => "1",
        TWO   => "2",
        THREE => "3",
        FOUR  => "4",
        FIVE  => "5",
        SIX   => "6",
        SEVEN => "7",
        EIGHT => "8",
        NINE  => "9",
        _     => "?",
    }
}
*/

#[derive(Debug)]
struct DisplaySet([SevenSeg; 10]); // 10 digits

#[derive(Debug)]
struct Display4(u32);          // 4 digits forming a display

/*
enum Digit {
    Zero(u8),
    One(u8),
    Two(u8),
    Three(u8),
    Four(u8),
    Five(u8),
    Six(u8),
    Seven(u8),
    Eight(u8),
    Nine(u8),
}
*/

impl DisplaySet {
    fn from(line: String) -> Result<DisplaySet, Box<dyn Error>> {
        let mut it = line.split(" ").map(|w| sevensegment(w.trim().as_bytes()));
        Ok(DisplaySet([
           it.next().ok_or("EOF")?, it.next().ok_or("EOF")?, it.next().ok_or("EOF")?,
           it.next().ok_or("EOF")?, it.next().ok_or("EOF")?, it.next().ok_or("EOF")?,
           it.next().ok_or("EOF")?, it.next().ok_or("EOF")?, it.next().ok_or("EOF")?,
           it.next().ok_or("EOF")?,
        ]))
    }
}

impl Display for DisplaySet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "DisplaySet([")?;
        for sevenseg in self.0.iter() {
            write!(f, "  {:07b}", *sevenseg)?;
        }
        write!(f, "  ])")?;
        Ok(())
    }
}

impl Display4 {
    fn from(line: String) -> Display4 {
        Display4(
            line.split(" ")
            .fold(0, |d4, w| (d4 << 8) | u32::from(sevensegment(w.trim().as_bytes())))
        )
    }
}

impl Display for Display4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Display4([")?;
        let mask = (1 << 8) - 1;
        for index in 0..4 {
            let shift = (3 - index) * 8;
            let ss = ((self.0 >> shift) & mask) as SevenSeg;
            write!(f, "  {:07b}", ss)?;
        }
        write!(f, "  ])")?;
        Ok(())
    }
}

fn read_displays(input: &mut dyn BufRead)
    -> Result<(Vec<DisplaySet>, Vec<Display4>), Box<dyn Error>>
{
    let mut display_sets = Vec::<DisplaySet>::with_capacity(INPUT_LEN_GUESS);
    let mut outputs = Vec::<Display4>::with_capacity(INPUT_LEN_GUESS);
    for line in input.lines() {
        let line = line?;
        let mut parts = line.split(" | ");
        display_sets.push(
            DisplaySet::from(parts.next().ok_or("expected ten digit part")?.into())?
        );
        outputs.push(
            Display4::from(parts.next().ok_or("expected output display part")?.into())
        );
    }
    Ok((display_sets, outputs))
}

impl Day for Day8 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let (display_sets, outputs) = read_displays(input)?;
        if opts.verbose {
            println!("display sets:");
            for display_set in display_sets {
                println!("  {}", display_set);
            }
            println!();
            println!("outputs:");
            for output in outputs {
                println!("  {}", output);
            }
        }
        // TODO
        Ok((PartResult::new(), PartResult::new()))
    }
}

