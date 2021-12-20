mod scanner;
mod vec3;

use vec3::Vec3;
use scanner::Scanner;

use std::env;
use std::io::{BufReader, prelude::*};
use std::fs::File;
use std::collections::HashSet;

fn read_input(fname: &str) -> Vec<Scanner> {
    let f = File::open(fname).unwrap();
    let reader = BufReader::new(f);

    let mut scanners = Vec::new();

    let mut lines = reader.lines().map(|l| l.unwrap());
    while let Some(line) = lines.next() {
        if line.starts_with("--- scanner") {
            scanners.push(Scanner::from(&mut lines));
        }
    }

    scanners
}

fn find_all_beacons(scanners: &mut [Scanner]) -> HashSet<Vec3> {
    let mut beacons: HashSet<Vec3> = scanners[0]
        .get_beacons()
        .iter()
        .cloned()
        .collect();

    let mut is_adjusted = vec![false; scanners.len()];
    is_adjusted[0] = true;

    loop {
        let (adjusted_scanners, unadjusted_scanners): (Vec<usize>, Vec<usize>)
            = (0..scanners.len()).into_iter().partition(|&i| is_adjusted[i]);

        if unadjusted_scanners.is_empty() {
            return beacons;
        }

        'outer: for &to_adjust in unadjusted_scanners.iter() {
            for &adjusted in adjusted_scanners.iter() {
                if let Some((a, b)) = scanners[adjusted].find_overlap(&scanners[to_adjust]) {
                    scanners[to_adjust].adjust_orientation(&a, &b);

                    for beacon in scanners[to_adjust].get_beacons() {
                        beacons.insert(beacon.clone());
                    }

                    is_adjusted[to_adjust] = true;

                    break 'outer;
                }
            }
        }
    }
}

fn largest_manhattan_dist(scanners: &[Scanner]) -> i32 {
    let mut largest = i32::MIN;

    for i in 0..scanners.len()-1 {
        for j in i+1..scanners.len() {
            largest = largest.max((scanners[i].get_position() - scanners[j].get_position()).manhattan());
        }
    }

    largest
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut scanners = read_input(&args[1]);

    let mut beacons: Vec<Vec3> = find_all_beacons(&mut scanners)
        .into_iter()
        .collect();
    beacons.sort_by(|&v, &w| v.x().partial_cmp(w.x()).unwrap());
    for beacon in beacons.iter() {
        println!("{}", beacon);
    }
    println!("{} total beacons", beacons.len());

    println!("Largest Manhattan distance: {}", largest_manhattan_dist(&scanners));
}
