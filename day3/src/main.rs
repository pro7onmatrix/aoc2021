use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn power_consumption(fname: &str) -> io::Result<(i32, i32)> {
    let f = File::open(fname)?;
    let mut reader = BufReader::new(f);

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let nbits = first_line.len() - 1;
    let mut counts: Vec<i32> = vec![0; nbits];

    let mut gamma   = 0;
    let mut epsilon = 0;

    reader.rewind()?;

    for line in reader.lines() {
        let l: Vec<char> = line?.chars().collect();
        for i in 0..nbits {
            if l[i] == '1' {
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

    Ok((gamma, epsilon))
}

fn life_support(fname: &str) -> io::Result<(i32, i32)> {
    let f = File::open(fname)?;
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let nbits = buf.len() - 1;

    reader.rewind()?;

    let digits: Vec<i32> = reader.split(b'\n')
                                 .map(|l| i32::from_str_radix(std::str::from_utf8(&l.unwrap()).unwrap(), 2).unwrap())
                                 .collect();

    let mut current_bit = nbits - 1;

    let count_bits = |acc, n: &i32, bit| {
        if n & 1 << bit != 0{
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

    Ok((o2_vec[0], co2_vec[0]))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (g, e) = power_consumption(&args[1]).unwrap();
    println!("Gamma = {}, epsilon = {} -> Result: {}", g, e, g * e);

    let (o2, co2) = life_support(&args[1]).unwrap();
    println!("O2 = {}, CO2 = {} -> Result: {}", o2, co2, o2 * co2);
}
