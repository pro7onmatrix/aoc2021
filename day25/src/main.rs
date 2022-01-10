use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum SeaCucumber {
    None,
    East,
    South,
}

impl std::convert::From<char> for SeaCucumber {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::None,
            '>' => Self::East,
            'v' => Self::South,
            _ => panic!("Invalid symbol!"),
        }
    }
}

fn read_input(fname: &str) -> io::Result<Vec<Vec<SeaCucumber>>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut grid = Vec::new();

    for line in reader.lines() {
        grid.push(line?.chars().map(SeaCucumber::from).collect());
    }

    Ok(grid)
}

fn step(grid: &mut Vec<Vec<SeaCucumber>>) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let (mut east, other): (Vec<(usize, usize)>, Vec<_>) = (0..nrows)
        .cartesian_product(0..ncols)
        .partition(|&(i, j)| matches!(grid[i][j], SeaCucumber::East));

    let (mut south, _): (Vec<(usize, usize)>, Vec<_>) = other
        .into_iter()
        .partition(|&(i, j)| matches!(grid[i][j], SeaCucumber::South));

    east.retain(|&(i, j)| matches!(grid[i][(j + 1) % ncols], SeaCucumber::None));

    let mut changes = east.len();

    // First, the east-moving cucumbers
    for (i, j) in east.into_iter() {
        grid[i][(j + 1) % ncols] = grid[i][j];
        grid[i][j] = SeaCucumber::None;
    }

    south.retain(|&(i, j)| matches!(grid[(i + 1) % nrows][j], SeaCucumber::None));

    changes += south.len();

    // Then, the south-moving cucumbers
    for (i, j) in south.into_iter() {
         grid[(i + 1) % nrows][j] = grid[i][j];
         grid[i][j] = SeaCucumber::None;
    }

    changes
}

fn simulate(grid: &mut Vec<Vec<SeaCucumber>>) -> usize {
    let mut nsteps = 1;

    while step(grid) > 0 {
        nsteps += 1;
    }

    nsteps
}

fn print_grid(grid: &Vec<Vec<SeaCucumber>>) {
    for row in grid.iter() {
        for cucumber in row.iter() {
            match cucumber {
                SeaCucumber::None => print!("."),
                SeaCucumber::East => print!(">"),
                SeaCucumber::South => print!("v"),
            }
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut grid = read_input(&args[1]).unwrap();

    // print_grid(&grid);
    // println!();

    // step(&mut grid);
    // print_grid(&grid);
    println!("Reached stationary state after {} steps", simulate(&mut grid));
}
