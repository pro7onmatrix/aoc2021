use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

fn load_heightmap(fname: &str) -> io::Result<Vec<Vec<i32>>> {
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

fn find_lowpoints(heightmap: &Vec<Vec<i32>>) -> (Vec<(usize, usize)>, i32) {
    let nrows = heightmap.len();
    let ncols = heightmap[0].len();

    let mut lowpoints = Vec::new();

    for i in 0..nrows {
        for j in 0..ncols {
            if i > 0 && heightmap[i][j] >= heightmap[i - 1][j] {
                continue;
            }

            if i < nrows - 1 && heightmap[i][j] >= heightmap[i + 1][j] {
                continue;
            }

            if j > 0 && heightmap[i][j] >= heightmap[i][j - 1] {
                continue;
            }

            if j < ncols - 1 && heightmap[i][j] >= heightmap[i][j + 1] {
                continue;
            }

            lowpoints.push((i, j));
        }
    }

    let risk_level = lowpoints.iter().map(|&(i, j)| heightmap[i][j] + 1).sum();
    (lowpoints, risk_level)
}

fn find_basins(heightmap: &Vec<Vec<i32>>) -> Vec<usize> {
    let (lowpoints, _) = find_lowpoints(heightmap);
    let mut counted = vec![vec![false; heightmap[0].len()]; heightmap.len()];

    let mut basins: Vec<usize> = lowpoints.iter()
                                          .map(|&(i, j)| basin_helper(heightmap, &mut counted, i, j))
                                          .collect();
    basins.sort();
    basins.reverse();

    basins
}

fn basin_helper(heightmap: &Vec<Vec<i32>>, counted: &mut Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    if heightmap[i][j] == 9 {
        return 0;
    }

    counted[i][j] = true;

    let nrows = heightmap.len();
    let ncols = heightmap[0].len();
    let mut basin_size = 1; // This spot

    if i > 0 && heightmap[i - 1][j] > heightmap[i][j] && !counted[i - 1][j] {
        basin_size += basin_helper(heightmap, counted, i - 1, j);
    }

    if i < nrows - 1 && heightmap[i + 1][j] > heightmap[i][j] && !counted[i + 1][j] {
        basin_size += basin_helper(heightmap, counted, i + 1, j);
    }

    if j > 0 && heightmap[i][j - 1] > heightmap[i][j] && !counted[i][j - 1] {
        basin_size += basin_helper(heightmap, counted, i, j - 1);
    }

    if j < ncols - 1 && heightmap[i][j + 1] > heightmap[i][j] && !counted[i][j + 1] {
        basin_size += basin_helper(heightmap, counted, i, j + 1);
    }

    basin_size
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let heightmap = load_heightmap(&args[1]).unwrap();

    let (lowpoints, risk_level) = find_lowpoints(&heightmap);
    println!("Low points: {:?} -> Risk level: {}", lowpoints, risk_level);

    let basins = find_basins(&heightmap);
    let product: usize = basins[..3].iter().product();
    println!("Found basins of sizes {:?} -> Product of three largest: {}", basins, product);
}
