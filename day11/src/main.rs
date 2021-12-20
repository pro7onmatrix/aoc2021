use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

fn read_octopodes(fname: &str) -> io::Result<Vec<Vec<i32>>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut v = Vec::new();

    for line in reader.lines() {
        let row = line?.chars()
                       .map(|c| (c as i32) - ('0' as i32))
                       .collect();

        v.push(row);
    }

    Ok(v)
}

fn flash(octopodes: &mut Vec<Vec<i32>>, flashed: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    if flashed[i][j] || octopodes[i][j] <= 9 {
        return;
    }

    flashed[i][j] = true;

    let top_edge    = i == 0;
    let bottom_edge = i == octopodes.len() - 1;
    let left_edge   = j == 0;
    let right_edge  = j == octopodes[0].len() - 1;

    // Left and right neighbors
    if !left_edge {
        octopodes[i][j - 1] += 1;
        flash(octopodes, flashed, i, j - 1);
    }

    if !right_edge {
        octopodes[i][j + 1] += 1;
        flash(octopodes, flashed, i, j + 1);
    }

    // Neighbors in the row above
    if !top_edge {
        octopodes[i - 1][j] += 1;
        flash(octopodes, flashed, i - 1, j);

        if !left_edge {
            octopodes[i - 1][j - 1] += 1;
            flash(octopodes, flashed, i - 1, j - 1);
        }

        if !right_edge {
            octopodes[i - 1][j + 1] += 1;
            flash(octopodes, flashed, i - 1, j + 1);
        }
    }

    // Neighbors in the row below
    if !bottom_edge {
        octopodes[i + 1][j] += 1;
        flash(octopodes, flashed, i + 1, j);

        if !left_edge {
            octopodes[i + 1][j - 1] += 1;
            flash(octopodes, flashed, i + 1, j - 1);
        }

        if !right_edge {
            octopodes[i + 1][j + 1] += 1;
            flash(octopodes, flashed, i + 1, j + 1);
        }
    }
}

fn step(octopodes: &mut Vec<Vec<i32>>, flashed: &mut Vec<Vec<bool>>) -> (usize, bool) {
    let nrows = octopodes.len();
    let ncols = octopodes[0].len();

    let mut nflashes = 0;

    // First, increase all by 1
    for i in 0..nrows {
        for j in 0..ncols {
            octopodes[i][j] += 1;
        }
    }

    // Perform the flashes
    for i in 0..nrows {
        for j in 0..ncols {
            flash(octopodes, flashed, i, j);
        }
    }

    // Count the flashes, check for synchronization
    // and reset the grid
    let mut sync = true;
    for i in 0..nrows {
        for j in 0..ncols {
            if flashed[i][j] {
                nflashes += 1;
                octopodes[i][j] = 0;
                flashed[i][j] = false;
            } else {
                sync = false;
            }
        }
    }

    (nflashes, sync)
}

fn simulate(octopodes: &Vec<Vec<i32>>, nsteps: usize) -> usize {
    let nrows = octopodes.len();
    let ncols = octopodes[0].len();

    let mut grid = octopodes.clone();
    let mut flashed = vec![vec![false; ncols]; nrows];

    let mut nflashes = 0;

    for _ in 0..nsteps {
        nflashes += step(&mut grid, &mut flashed).0;
    }

    nflashes
}

fn find_sync(octopodes: &Vec<Vec<i32>>) -> usize {
    let nrows = octopodes.len();
    let ncols = octopodes[0].len();

    let mut grid = octopodes.clone();
    let mut flashed = vec![vec![false; ncols]; nrows];

    for n in 0.. {
        if step(&mut grid, &mut flashed).1 {
            return n + 1;
        }
    }

    usize::MAX
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let octopodes = read_octopodes(&args[1]).unwrap();

    let nflashes = simulate(&octopodes, 100);
    println!("After 100 steps, there were {} flashes", nflashes);

    let nsync = find_sync(&octopodes);
    println!("First synchronization after {} steps", nsync);
}
