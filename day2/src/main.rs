use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn dive1(fname: &str) -> std::io::Result<(i32, i32)> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut horizontal = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let l = line?;
        let s: Vec<&str> = l.split_whitespace().collect();
        let amount: i32 = s[1].parse().unwrap();

        match s[0] {
            "forward" => horizontal += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("Illegal command '{}'!", s[0]),
        }
    }

    Ok((horizontal, depth))
}

fn dive2(fname: &str) -> std::io::Result<(i32, i32)> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let l = line?;
        let s: Vec<&str> = l.split_whitespace().collect();
        let amount: i32 = s[1].parse().unwrap();

        match s[0] {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Illegal command '{}'!", s[0]),
        }
    }

    Ok((horizontal, depth))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (h1, d1) = dive1(&args[1]).unwrap();
    println!("Horizontal position: {}, depth: {} -> Result: {}", h1, d1, h1 * d1);

    let (h2, d2) = dive2(&args[1]).unwrap();
    println!("Horizontal position: {}, depth: {} -> Result: {}", h2, d2, h2 * d2);
}
