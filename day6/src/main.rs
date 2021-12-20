use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::collections::HashMap;

fn read_input(fname: &str) -> io::Result<Vec<i32>> {
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

fn lanternfish_brute_force(fish: &Vec<i32>, ndays: i32) -> usize {
    let mut fish = fish.clone();
    let mut nfish = fish.len();

    for _ in 0..ndays {
        for i in 0..nfish {
            if fish[i] == 0 {
                fish.push(8);
                fish[i] = 6;
            } else {
                fish[i] -= 1;
            }
        }

        nfish = fish.len()
    }

    nfish
}

fn trace_fish(days_left: i32, depth: i32) -> usize {
    if days_left < 0 {
        return 0;
    }

    let mut nfish = 0;
    let mut days_left = days_left;

    while days_left > 0 {
        nfish += 1 + trace_fish(days_left - 9, depth + 1);
        days_left -= 7;
    }

    nfish
}

fn lanternfish_brute_force2(fish: &Vec<i32>, ndays: i32) -> usize {
    let mut cache: HashMap<i32, usize> = HashMap::new();
    let mut nfish = fish.len();

    for f in fish.iter() {
        if !cache.contains_key(f) {
            cache.insert(*f, trace_fish(ndays - f, 0));
        }
        nfish += cache[f];
    }

    nfish
}

fn lanternfish(fish: &Vec<i32>, ndays: usize) -> usize {
    let mut counts = [0; 9];

    for &f in fish.iter() {
        counts[f as usize] += 1;
    }

    let mut nfish = fish.len();

    for _ in 0..ndays {
        let spawns = counts[0];

        for i in 0..8 {
            counts[i] = counts[i + 1];
        }
        counts[6] += spawns; // Old fish return to 7 days
        counts[8]  = spawns; // Spawn new fish with 9 days

        nfish += spawns;
    }

    nfish
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fish = read_input(&args[1]).unwrap();

    let nfish = lanternfish(&fish, 18);
    println!("After {} days, there are {} lanternfish", 18, nfish);

    // let nfish = lanternfish_brute_force(&fish, 18);
    // println!("After {} days, there are {} lanternfish", 18, nfish);

    let nfish = lanternfish(&fish, 80);
    println!("After {} days, there are {} lanternfish", 80, nfish);

    // let nfish = lanternfish_brute_force(&fish, 80);
    // println!("After {} days, there are {} lanternfish", 80, nfish);

    let nfish = lanternfish(&fish, 256);
    println!("After {} days, there are {} lanternfish", 256, nfish);

    // let nfish = lanternfish_brute_force(&fish, 256);
    // println!("After {} days, there are {} lanternfish", 256, nfish);
}
