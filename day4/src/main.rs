use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

#[derive(Clone)]
struct Board {
    numbers: [[i32; 5]; 5],
    marked:  [[bool; 5]; 5],
}

impl Board {
    fn new(lines: &[String; 5]) -> Self {
        let mut numbers = [[0; 5]; 5];

        for (i, line) in lines.iter().enumerate() {
            let row: Vec<i32> = line.replace("\n", "")
                                    .split_whitespace()
                                    .map(|n| n.parse().unwrap())
                                    .collect();

            for j in 0..5 {
                numbers[i][j] = row[j];
            }
        }

        let marked = [[false; 5]; 5];

        Self { numbers, marked }
    }

    fn mark_number(&mut self, n: i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == n {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    fn win(&self) -> bool {
        // Check rows
        if self.marked.iter().any(|row| row.iter().all(|&m| m)) {
            return true;
        }

        // Check columns
        for j in 0..5 {
            let mut all = true;

            for i in 0..5 {
                if !self.marked[i][j] {
                    all = false;
                    break;
                }
            }

            if all {
                return true;
            }
        }

        return false;
    }

    fn score(&self) -> i32 {
        let mut sum = 0;

        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    sum += self.numbers[i][j];
                }
            }
        }

        sum
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..5 {
            for j in 0..5 {
                if j > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:2}", self.numbers[i][j])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn bingo_setup(fname: &str) -> io::Result<(Vec<i32>, Vec<Board>)> {
    let f = File::open(fname)?;
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    let numbers: Vec<i32> = buf.replace("\n", "")
                               .split(",")
                               .map(|n| n.parse().unwrap())
                               .collect();

    let mut board_str = [String::new(), String::new(), String::new(), String::new(), String::new()];
    let mut boards: Vec<Board> = Vec::new();

    let mut lines = reader.lines();

    while let Some(_) = lines.next() {
        for i in 0..5 {
            board_str[i] = lines.next().unwrap()?;
        }
        boards.push(Board::new(&board_str));
    }

    Ok((numbers, boards))
}

fn bingo_win(numbers: &Vec<i32>, boards: &Vec<Board>) -> Option<(Board, i32)> {
    let mut local_boards = (*boards).clone();

    for &number in numbers.iter() {
        for board in local_boards.iter_mut() {
            board.mark_number(number);

            if board.win() {
                return Some((board.clone(), number * board.score()));
            }
        }
    }

    None
}

fn bingo_lose(numbers: &Vec<i32>, boards: &Vec<Board>) -> Option<(Board, i32)> {
    let mut local_boards = (*boards).clone();
    let mut nums = numbers.iter();

    let mut number: i32 = 0;

    // Get it down to 1 board
    while local_boards.len() > 1 {
        number = *nums.next().unwrap();
        for board in local_boards.iter_mut() {
            board.mark_number(number);
        }
        local_boards.retain(|board| !board.win());
    }

    if local_boards.is_empty() {
        return None;
    }

    let last = &mut local_boards[0];

    // Keep drawing numbers until the board is complete
    while !last.win() {
        number = *nums.next().unwrap();
        last.mark_number(number);
    }

    Some((last.clone(), number * last.score()))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (numbers, boards) = bingo_setup(&args[1]).unwrap();

    let (winner, hiscore) = bingo_win(&numbers, &boards).unwrap();
    println!("Winning board:");
    print!("{}", winner);
    println!("Score: {}", hiscore);
    println!();

    let (loser, loscore) = bingo_lose(&numbers, &boards).unwrap();
    println!("Losing board:");
    print!("{}", loser);
    println!("Score: {}", loscore);
}
