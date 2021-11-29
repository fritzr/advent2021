mod cli;
mod d01;
//mod d02;
//mod d03;
//mod d04;
//mod d05;
//mod d06;
//mod d07;
//mod d08;
//mod d09;
//mod d10;
//mod d11;
//mod d12;
//mod d13;
//mod d14;
//mod d15;
//mod d16;
//mod d17;
//mod d18;
//mod d19;
//mod d20;
//mod d21;
//mod d22;
//mod d23;
//mod d24;
//mod d25;

fn main() {
    use structopt::StructOpt;
    let opts = cli::Cli::from_args();
    if opts.verbose {
        println!("Running day(s) {}..{}", opts.day.0, opts.day.1);
    }
    let mut day = opts.day.0;
    while day <= opts.day.1 {
        match day {
             1 => d01::run(&opts),
            // 2 => d02::run(&opts),
            // 3 => d03::run(&opts),
            // 4 => d04::run(&opts),
            // 5 => d05::run(&opts),
            // 6 => d06::run(&opts),
            // 7 => d07::run(&opts),
            // 8 => d08::run(&opts),
            // 9 => d09::run(&opts),
            //10 => d10::run(&opts),
            //11 => d11::run(&opts),
            //12 => d12::run(&opts),
            //13 => d13::run(&opts),
            //14 => d14::run(&opts),
            //15 => d15::run(&opts),
            //16 => d16::run(&opts),
            //17 => d17::run(&opts),
            //18 => d18::run(&opts),
            //19 => d19::run(&opts),
            //20 => d20::run(&opts),
            //21 => d21::run(&opts),
            //22 => d22::run(&opts),
            //23 => d23::run(&opts),
            //24 => d24::run(&opts),
            //25 => d25::run(&opts),
            _ => if opts.verbose { println!("day {} unimplemented", day) },
        }
        day = day + 1;
    }
}
