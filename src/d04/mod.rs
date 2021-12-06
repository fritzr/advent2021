use std::io::BufRead;
use crate::{cli, Day, PartResult, util};
use std::error::Error;
use std::convert::From;
use std::time::{Instant, Duration};

pub struct Day4;

fn read_numbers(input: &mut dyn BufRead) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut number_str = String::new();
    input.read_line(&mut number_str)?;
    number_str.pop(); // pop trailing newline
    let mut numbers = Vec::<u8>::with_capacity(100);
    for num in number_str.split(",").map(|s| s.parse::<u8>()) {
        numbers.push(num?);
    }
    Ok(numbers)
}

struct BingoBoard {
    // Board itself -- don't need to actually store this.
    // board: [u8; 25],
    // lookup[value] = None or Some(row, col) of number in board.
    lookup: [Option<(usize, usize)>; 100],
    // row_marks[row] = number of marked values in row
    row_marks: [u8; 5],
    // col_marks[col] = number of marked values in col
    col_marks: [u8; 5],
    // Sum of unmarked numbers.
    sum: usize,
    // Winning number.
    winner: Option<u8>,
}

impl BingoBoard {
    pub fn new() -> BingoBoard {
        BingoBoard {
            // board: [0; 25],
            lookup: [None; 100],
            row_marks: [0; 5],
            col_marks: [0; 5],
            sum: 0,
            winner: None,
        }
    }

    pub fn from(string: String) -> Result<BingoBoard, Box<dyn Error>> {
        let mut board = BingoBoard::new();
        // println!("board from: {:?}", board_str);
        for (row_index, line) in string.lines().enumerate() {
            for (col_index, value) in line.split_whitespace()
                    .map(|s| s.parse::<u8>())
                    .enumerate() {
                // board.board[row_index + col_index * board.row_marks.len()] = value;
                let value = value?;
                board.lookup[usize::from(value)] = Some((row_index, col_index));
                board.sum = board.sum.saturating_add(usize::from(value));
            }
        }
        Ok(board)
    }

    // Check for a number on the board.
    //
    // Return Some(value) if the number was marked and caused the board to win.
    pub fn check(&mut self, value: u8) -> Option<u8> {
        if let Some((row, col)) = self.lookup[usize::from(value)] {
            // Number is present on board, mark it and check if the board won.
            if let Some(winner) = self.mark(row, col, value) {
                return Some(winner);
            }
        }
        None
    }

    // Mark a called number on the board at (row, col).
    //
    // Return Some(value) if the number caused this board to win.
    fn mark(&mut self, row: usize, col: usize, value: u8) -> Option<u8> {
        // Subtract value from unmarked sum.
        self.sum = self.sum.saturating_sub(value.into());
        self.row_marks[row] += 1;
        self.col_marks[col] += 1;
        // YOWO (you only win once)
        if (usize::from(self.row_marks[row]) == self.row_marks.len()
            || usize::from(self.col_marks[col]) == self.col_marks.len())
            && self.winner.is_none() {
            self.winner = Some(value);
            self.winner
        }
        else {
            None
        }
    }

    // Return sum of unmarked numbers.
    pub fn sum(&self) -> usize { self.sum }
}

fn bingo(input: &mut dyn BufRead, verbose: bool) -> Result<(usize, usize), Box<dyn Error>> {
    let numbers = read_numbers(input)?;
    let mut first_win_value = 0;
    let (mut l_value, mut l_number, mut l_index, mut l_board, mut l_sum)
        = (0, 0, 0, 0, 0);
    if verbose {
        println!("bingo numbers: {:?}", &numbers);
    }
    let mut boards: Vec<BingoBoard> = util::split_groups(input)
        .map(|s| BingoBoard::from(s).unwrap())
        .collect();
    for (num_index, num) in numbers.iter().enumerate() {
        for (board_index, board) in boards.iter_mut().enumerate() {
            if let Some(winning_number) = board.check(*num) {
                l_sum = board.sum();
                l_number = winning_number.into();
                l_value = l_sum.saturating_mul(l_number);
                if verbose {
                    l_index = num_index;
                    l_board = board_index;
                }
                if first_win_value == 0 {
                    if verbose {
                        println!(
                            "  First win on number {} after {} / {} moves: board {}, sum = {}",
                            winning_number, num_index + 1, numbers.len(), board_index + 1,
                            board.sum());
                    }
                    first_win_value = l_value;
                }
            }
        }
    }
    if verbose {
        println!("  Last win on number {} after {} / {} moves: board {}, sum = {}",
            l_number, l_index + 1, numbers.len(), l_board + 1, l_sum);
    }
    Ok((first_win_value, l_value))
}

impl Day for Day4 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let time = Instant::now();
        let (first_win, last_win) = bingo(input, opts.verbose)?;
        let time = time.elapsed();
        Ok((PartResult { answer: first_win.to_string(), time },
            PartResult { answer: last_win.to_string(), time: Duration::new(0, 0) }))
    }
}

