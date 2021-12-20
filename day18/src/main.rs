#[cfg(test)]
mod tests;
mod snailfish;

use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

use crate::snailfish::SnailfishNumber;

fn read_input(fname: &str) -> io::Result<Vec<SnailfishNumber>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut numbers = Vec::new();

    for line in reader.lines() {
        numbers.push(SnailfishNumber::new(&line?));
    }

    Ok(numbers)
}

fn find_largest_sum(numbers: &Vec<SnailfishNumber>) -> SnailfishNumber {
    let n = numbers.len();
    let mut largest_sum = SnailfishNumber::Empty;

    for i in 0..n {
        for j in 0..i {
            let sum1 = &numbers[i] + &numbers[j];
            if sum1.magnitude() > largest_sum.magnitude() {
                largest_sum = sum1;
            }

            let sum2 = &numbers[j] + &numbers[i];
            if sum2.magnitude() > largest_sum.magnitude() {
                largest_sum = sum2;
            }
        }
    }

    largest_sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let numbers = read_input(&args[1]).unwrap();

    let sum: SnailfishNumber = numbers.iter().sum();
    println!("Sum: {}, magnitude: {}", sum, sum.magnitude());

    let largest_sum = find_largest_sum(&numbers);
    println!("Largest sum: {}, magnitude: {}", largest_sum, largest_sum.magnitude());
}
