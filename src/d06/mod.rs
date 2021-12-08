use std::io::BufRead;
use crate::{cli, Day, PartResult, util};
use std::error::Error;
use std::cmp::min;

pub struct Day6;

const SPAWN_PERIOD: usize = 7;
const SPAWN_DELAY: usize = 2;
const RING_SIZE: usize = SPAWN_PERIOD + SPAWN_DELAY;

// Simulate fish with lifetimes seeded by the input.
//
// Each value T in the input represents the number of days
// until the fish will split into two fish:
// one with T=6, and one with T=8.
//
// The return value is the number of fish alive after num_days.
fn simulate<I>(fish: I, num_days: usize, verbose: bool) -> usize
    where I: IntoIterator<Item=usize>
{
    let mut num_fish: usize = 0;
    // Ring buffer: buffer[i] is the number of fish splitting on day (now + i).
    let mut spawn_ring: [usize; RING_SIZE] = [0; RING_SIZE];
    let mut day: usize = SPAWN_PERIOD;
    for t in fish {
        num_fish += 1;
        spawn_ring[t] += 1;
        if verbose {
            println!("  {:>8} fish to split on day {}", spawn_ring[t], t);
        }
        day = min(day, t);
    }
    if verbose {
        println!("  {:>8} fish to start", num_fish);
    }
    let mut day_slot = day; // always day % RING_SIZE
    let mut last_spawned = 0;
    for day in day..num_days+1 /* day zero doesn't count */ {
        if day_slot == RING_SIZE {
            day_slot = 0;
        }
        num_fish += last_spawned;
        last_spawned = spawn_ring[day_slot];
        // spawn_ring[day % RING_SIZE] = 0; cleared below
        //num_fish += last_spawned;
        // accumulate fish known to spawn every period in the ring
        spawn_ring[(day_slot + SPAWN_PERIOD) % RING_SIZE] += last_spawned;
        // delayed spawn always occurs every RING_SIZE, so we're re-using this day slot
        spawn_ring[day_slot /* + SPAWN_PERIOD + SPAWN_DELAY + 1*/] = last_spawned;
        if verbose {
            println!("|day {:2}| created {} fish, now {:<8}", day, last_spawned, num_fish);
            println!(" {:>9} fish to split on day {}",
                 spawn_ring[(day_slot + SPAWN_PERIOD) % RING_SIZE], day + SPAWN_PERIOD);
            println!(" {:>9} fish to split on day {}",
                 spawn_ring[day_slot], day + RING_SIZE);
        }
        day_slot += 1;
    }
    num_fish
}

impl Day for Day6 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let mut num_days = 80;
        if let Some(args) = &opts.args {
            assert_eq!(args.len() > 0, true);
            num_days = args[0].parse()?;
        }
        if opts.verbose {
            println!("Simulating {} days", num_days);
        }
        let input = util::read_csv(input)?;
        Ok((PartResult::from(|| simulate(input, num_days, opts.verbose)),
            PartResult::new()))
    }
}

