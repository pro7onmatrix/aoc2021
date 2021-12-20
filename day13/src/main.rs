use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Copy, Clone)]
enum FoldAxis {
    X,
    Y,
}

fn read_grid(fname: &str) -> io::Result<(Vec<Vec<bool>>, Vec<(FoldAxis, usize)>)> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut max_x = 0;
    let mut max_y = 0;

    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<(FoldAxis, usize)> = Vec::new();

    lazy_static! {
        static ref POINT_RE: Regex = Regex::new(r"^(\d+),(\d+)$").unwrap();
        static ref FOLD_RE:  Regex = Regex::new(r"^fold along ([xy])=(\d+)$").unwrap();
    }

    for line in reader.lines() {
        let l = line?;
        if let Some(caps) = POINT_RE.captures(&l) {
            let x = caps[1].parse().unwrap();
            let y = caps[2].parse().unwrap();

            max_x = max_x.max(x);
            max_y = max_y.max(y);

            points.push((x, y));
        } else if let Some(caps) = FOLD_RE.captures(&l) {
            let axis = if &caps[1] == "x" { FoldAxis::X } else { FoldAxis::Y };
            let pos = caps[2].parse().unwrap();

            folds.push((axis, pos));
        }
    }

    let nrows = (max_y as usize) + 1;
    let ncols = (max_x as usize) + 1;
    let mut grid = vec![vec![false; ncols]; nrows];

    for &(i, j) in points.iter() {
        grid[j][i] = true;
    }

    Ok((grid, folds))
}

fn perform_fold(grid: &mut Vec<Vec<bool>>, axis: FoldAxis, pos: usize) {
    let nrows = grid.len();
    let ncols = grid[0].len();

    match axis {
        FoldAxis::X => {
            for j in 0..nrows {
                for i in 0..pos {
                    grid[j][i] |= grid[j][ncols - i - 1];
                }
                grid[j].resize(pos, false);
            }
        },
        FoldAxis::Y => {
            for j in 0..pos {
                for i in 0..ncols {
                    grid[j][i] |= grid[nrows - j - 1][i];
                }
            }
            grid.resize(pos, Vec::new());
        }
    }
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid.iter() {
        println!("{}", row.iter().map(|&point| if point { '#' } else { '.' }).collect::<String>());
    }
}

fn count_dots(grid: &Vec<Vec<bool>>) -> usize {
    let mut dots = 0;

    for row in grid.iter() {
        dots += row.iter().filter(|&&point| point).count();
    }

    dots
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (mut grid, folds) = read_grid(&args[1]).unwrap();

    let mut first = true;
    for &(axis, pos) in folds.iter() {
        perform_fold(&mut grid, axis, pos);
        if first {
            println!("After the first fold, there are {} dots", count_dots(&grid));
            first = false;
        }
    }

    print_grid(&grid);
    println!("After all folds, there are {} dots", count_dots(&grid));
}
