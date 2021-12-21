use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

fn read_input(fname: &str) -> io::Result<Vec<Vec<char>>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut report = Vec::new();

    for line in reader.lines() {
        report.push(line?.chars().collect());
    }

    Ok(report)
}

fn power_consumption(report: &Vec<Vec<char>>) -> (i32, i32) {
    let nbits = report[0].len();
    let mut counts: Vec<i32> = vec![0; nbits];

    let mut gamma   = 0;
    let mut epsilon = 0;

    for line in report.iter() {
        for i in 0..nbits {
            if line[i] == '1' {
                counts[i] += 1;
            } else {
                counts[i] -= 1;
            }
        }
    }

    for i in 0..nbits {
        if counts[i] > 0 {
            gamma |= 1 << (nbits - i - 1);
        } else {
            epsilon |= 1 << (nbits - i - 1);
        }
    }

    (gamma, epsilon)
}

fn life_support(report: &Vec<Vec<char>>) -> (i32, i32) {
    let nbits = report[0].len();

    let digits: Vec<i32> = report
        .iter()
        .map(|line| i32::from_str_radix(&line.iter().cloned().collect::<String>(), 2).unwrap())
        .collect();

    let mut current_bit = nbits - 1;

    let count_bits = |acc, n: &i32, bit| {
        if n & (1 << bit) != 0 {
            acc + 1
        } else {
            acc - 1
        }
    };

    let counts = digits.iter().fold(0, |acc, n| count_bits(acc, n, current_bit));
    let target = (counts >= 0) as i32;
    let (mut o2_vec, mut co2_vec): (Vec<i32>, Vec<i32>)
        = digits.iter()
                .partition(|&n| (n & 1 << current_bit) >> current_bit == target);

    while o2_vec.len() > 1 || co2_vec.len() > 1 {
        current_bit -= 1;

        if o2_vec.len() > 1 {
            let counts = o2_vec.iter().fold(0, |acc, n| count_bits(acc, n, current_bit));
            let target = (counts >= 0) as i32;
            o2_vec.retain(|n| (n & 1 << current_bit) >> current_bit == target);
        }

        if co2_vec.len() > 1 {
            let counts = co2_vec.iter().fold(0, |acc, n| count_bits(acc, n, current_bit));
            let target = (counts < 0) as i32;
            co2_vec.retain(|n| (n & 1 << current_bit) >> current_bit == target);
        }
    }

    (o2_vec[0], co2_vec[0])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let report = read_input(&args[1]).unwrap();

    let (g, e) = power_consumption(&report);
    println!("Gamma = {}, epsilon = {} -> Result: {}", g, e, g * e);

    let (o2, co2) = life_support(&report);
    println!("O2 = {}, CO2 = {} -> Result: {}", o2, co2, o2 * co2);
}
