use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::collections::HashSet;
use regex::Regex;

fn read_input(fname: &str) -> io::Result<(i32, i32, i32, i32)> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let re = Regex::new(r"^target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let input = reader.lines().nth(0).unwrap()?;

    let caps = re.captures(&input).unwrap();

    let xmin = caps[1].parse().unwrap();
    let xmax = caps[2].parse().unwrap();
    let ymin = caps[3].parse().unwrap();
    let ymax = caps[4].parse().unwrap();

    Ok((xmin, xmax, ymin, ymax))
}

fn hits_target(vx_0: i32, vy_0: i32, target_area: (i32, i32, i32, i32)) -> bool {
    let mut vx = vx_0;
    let mut vy = vy_0;

    let (xmin, xmax, ymin, ymax) = target_area;

    let mut x = 0;
    let mut y = 0;

    loop {
        x += vx;
        y += vy;

        // Landed in target area
        if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
            return true;
        }

        // Overshot the target area
        if x > xmax || y < ymin {
            return false;
        }

        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }

        vy -= 1;
    }
}

fn find_max_height(target_area: (i32, i32, i32, i32)) -> i32 {
    let mut max_height = 0;

    let (xmin, xmax, ymin, _) = target_area;

    // The smallest possible x velocity to land in the target area:
    // vx * (vx + 1) / 2 = xmin
    // => vx^2 + vx - 2 xmin = 0
    // => vx = -1/2 + sqrt(1/4 + 2 xmin)
    let vx_min = (-0.5 + (0.25 + 2.0 * xmin as f32).sqrt()).ceil() as i32;
    let vx_max = xmax + 1;
    let vy_min = 0;
    let vy_max = ymin.abs();

    for vx in vx_min..vx_max {
        for vy in vy_min..vy_max {
            if hits_target(vx, vy, target_area) {
                // Total height will be the vy-th triangular number
                max_height = max_height.max(vy * (vy + 1) / 2);
            }
        }
    }

    max_height
}

fn find_all_velocities(target_area: (i32, i32, i32, i32)) -> HashSet<(i32, i32)> {
    let mut velocities = HashSet::new();

    let (xmin, xmax, ymin, _) = target_area;

    // The smallest possible x velocity to land in the target area:
    // vx * (vx + 1) / 2 = xmin
    // => vx^2 + vx - 2 xmin = 0
    // => vx = -1/2 + sqrt(1/4 + 2 xmin)
    let vx_min = (-0.5 + (0.25 + 2.0 * xmin as f32).sqrt()).ceil() as i32;
    let vx_max = xmax + 1;
    let vy_max = ymin.abs();
    let vy_min = -vy_max;

    for vx in vx_min..vx_max {
        for vy in vy_min..vy_max {
            if hits_target(vx, vy, target_area) {
                velocities.insert((vx, vy));
            }
        }
    }

    velocities
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = read_input(&args[1]).unwrap();

    let max_height = find_max_height(target);
    let velocities = find_all_velocities(target);
    println!("Maximum height: {}", max_height);
    println!("Can reach the target with {} distinct initial velocities", velocities.len());
}
