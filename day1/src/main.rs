use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn count_depth_increases(fname: &str) -> std::io::Result<i32> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut count = 0;
    let mut prev: i32 = i32::MAX;

    for (i, line) in reader.lines().enumerate() {
        let depth: i32 = line?.parse().unwrap();

        if i > 0 && depth > prev {
            count += 1;
        }

        prev = depth;
    }

    Ok(count)
}

fn count_sliding_increases(fname: &str) -> std::io::Result<i32> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut count = 0;
    let mut window: [i32; 3] = [i32::MAX, i32::MAX, i32::MAX];

    for (i, line) in reader.lines().enumerate() {
        let depth = line?.parse().unwrap();

        let m = i % 3;
        let n = (i + 1) % 3;
        let k = (i + 2) % 3;

        window[n] += depth;
        window[k] += depth;

        if i > 2 && window[n] > window[m] {
            count += 1;
        }

        window[m] = depth;
    }

    Ok(count)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result1 = count_depth_increases(&args[1]).unwrap();
    println!("Depth increased {} times", result1);

    let result2 = count_sliding_increases(&args[1]).unwrap();
    println!("Sliding window sum increased {} times", result2);
}
