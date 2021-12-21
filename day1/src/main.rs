use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

fn read_input(fname: &str) -> io::Result<Vec<i32>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut depths = Vec::new();

    for line in reader.lines() {
        depths.push(line?.parse().unwrap());
    }

    Ok(depths)
}

fn count_depth_increases(depths: &[i32]) -> usize {
    let mut count = 0;
    let mut prev  = depths[0];

    for &depth in depths.iter().skip(1) {
        if depth > prev {
            count += 1;
        }
        prev = depth;
    }

    count
}

fn count_sliding_increases(depths: &[i32]) -> usize {
    let mut count = 0;
    let mut window: [i32; 3] = [depths[0] + depths[1] + depths[2],
                                            depths[1] + depths[2],
                                                        depths[2]];

    for (i, &depth) in depths.iter().enumerate().skip(3) {
        let m = i % 3;
        let n = (i + 1) % 3;
        let k = (i + 2) % 3;

        window[n] += depth;
        window[k] += depth;

        if window[n] > window[m] {
            count += 1;
        }

        window[m] = depth;
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let depths = read_input(&args[1]).unwrap();

    let result1 = count_depth_increases(&depths);
    println!("Depth increased {} times", result1);

    let result2 = count_sliding_increases(&depths);
    println!("Sliding window sum increased {} times", result2);
}
