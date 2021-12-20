use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

fn read_positions(fname: &str) -> io::Result<Vec<i32>> {
    let f = File::open(fname)?;
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    let v = buf.replace("\n", "")
               .split(',')
               .map(|n| n.parse().unwrap())
               .collect();

    Ok(v)
}

fn minmax(v: &[i32]) -> (i32, i32) {
    v.iter()
     .fold((i32::MAX, i32::MIN), |(min, max), &x| (min.min(x), max.max(x)))
}

#[inline(always)]
fn triangular(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn minimize_fuel(submarines: &Vec<i32>) -> (i32, i32) {
    let (min, max) = minmax(submarines);
    (min..max+1).into_iter()
                .fold((-1, i32::MAX), |(best_pos, best_fuel), pos| {
                    let fuel = submarines.iter()
                                         .map(|x| (x - pos).abs())
                                         .sum();
                    if fuel < best_fuel {
                        (pos, fuel)
                    } else {
                        (best_pos, best_fuel)
                    }
                })
}

fn minimize_fuel_increasing(submarines: &Vec<i32>) -> (i32, i32) {
    let (min, max) = minmax(submarines);
    (min..max+1).into_iter()
                .fold((-1, i32::MAX), |(best_pos, best_fuel), pos| {
                    let fuel = submarines.iter()
                                         .map(|x| triangular((x - pos).abs()))
                                         .sum();
                    if fuel < best_fuel {
                        (pos, fuel)
                    } else {
                        (best_pos, best_fuel)
                    }
                })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let submarines = read_positions(&args[1]).unwrap();

    let (pos, fuel) = minimize_fuel(&submarines);
    println!("Best position: {}, fuel cost {}", pos, fuel);

    let (pos, fuel) = minimize_fuel_increasing(&submarines);
    println!("Best position with increasing cost: {}, fuel cost {}", pos, fuel);
}
