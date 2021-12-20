use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

fn read_grid(fname: &str) -> io::Result<Vec<Vec<i32>>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut cave = Vec::new();

    for line in reader.lines() {
        let row = line?.chars().map(|c| (c as i32) - ('0' as i32)).collect();
        cave.push(row);
    }

    Ok(cave)
}

fn find_path(grid: &Vec<Vec<i32>>) -> i32 {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut visited = vec![vec![false; ncols]; nrows];

    let mut risk_scores = vec![vec![i32::MAX; ncols]; nrows];
    risk_scores[0][0] = 0;

    let mut i = 0;
    let mut j = 0;

    // The most inefficient implementation of Dijkstra you'll ever see
    while i != nrows - 1 || j != ncols - 1 {
        let current_risk = risk_scores[i][j];

        if i > 0 && !visited[i - 1][j] && current_risk + grid[i - 1][j] < risk_scores[i - 1][j] {
            risk_scores[i - 1][j] = current_risk + grid[i - 1][j];
        }

        if i < nrows - 1 && !visited[i + 1][j] && current_risk + grid[i + 1][j] < risk_scores[i + 1][j] {
            risk_scores[i + 1][j] = current_risk + grid[i + 1][j];
        }

        if j > 0 && !visited[i][j - 1] && current_risk + grid[i][j - 1] < risk_scores[i][j - 1] {
            risk_scores[i][j - 1] = current_risk + grid[i][j - 1];
        }

        if j < ncols - 1 && !visited[i][j + 1] && current_risk + grid[i][j + 1] < risk_scores[i][j + 1] {
            risk_scores[i][j + 1] = current_risk + grid[i][j + 1];
        }

        visited[i][j] = true;

        let mut lowest = i32::MAX;
        for ii in 0..nrows {
            for jj in 0..ncols {
                if !visited[ii][jj] && risk_scores[ii][jj] < lowest {
                    i = ii;
                    j = jj;
                    lowest = risk_scores[ii][jj];
                }
            }
        }
    }

    risk_scores[nrows - 1][ncols - 1]
}

fn generate_tiles(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut tiles = vec![vec![0; ncols * 5]; nrows * 5];

    // Loop over tiles
    for m in 0..5 {
        for n in m..5 {
            for i in 0..nrows {
                for j in 0..ncols {
                    let mut value = grid[i][j] + (m as i32) + (n as i32);

                    if value > 9 {
                        value -= 9;
                    }

                    tiles[i + m * nrows][j + n * ncols] = value;
                    tiles[i + n * nrows][j + m * ncols] = value;
                }
            }
        }
    }

    tiles
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    let nrows = grid.len();
    let ncols = grid[0].len();

    for i in 0..nrows {
        for j in 0..ncols {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let grid = read_grid(&args[1]).unwrap();

    let score = find_path(&grid);
    println!("Risk score part 1: {}", score);

    let tiles = generate_tiles(&grid);
    let score2 = find_path(&tiles);
    println!("Risk score part 2: {}", score2);
}
